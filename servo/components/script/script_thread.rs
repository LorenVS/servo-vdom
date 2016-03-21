/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! The script thread is the thread that owns the DOM in memory, runs JavaScript, and spawns parsing
//! and layout threads. It's in charge of processing events for all same-origin pages in a frame
//! tree, and manages the entire lifetime of pages in the frame tree from initial request to
//! teardown.
//!
//! Page loads follow a two-step process. When a request for a new page load is received, the
//! network request is initiated and the relevant data pertaining to the new page is stashed.
//! While the non-blocking request is ongoing, the script thread is free to process further events,
//! noting when they pertain to ongoing loads (such as resizes/viewport adjustments). When the
//! initial response is received for an ongoing load, the second phase starts - the frame tree
//! entry is created, along with the Window and Document objects, and the appropriate parser
//! takes over the response body. Once parsing is complete, the document lifecycle for loading
//! a page runs its course and the script thread returns to processing events in the main event
//! loop.

use devtools;
use devtools_traits::CSSError;
use devtools_traits::{DevtoolScriptControlMsg, DevtoolsPageInfo};
use devtools_traits::{ScriptToDevtoolsControlMsg, WorkerId};
use document_loader::DocumentLoader;
use dom::bindings::cell::DOMRefCell;
use dom::bindings::codegen::Bindings::DocumentBinding::{DocumentMethods, DocumentReadyState};
use dom::bindings::codegen::Bindings::NodeBinding::NodeMethods;
use dom::bindings::global::GlobalRef;
use dom::bindings::inheritance::Castable;
use dom::bindings::js::{JS, MutNullableHeap, Root};
use dom::bindings::js::{RootedReference};
use dom::bindings::refcounted::{LiveDOMReferences, Trusted, TrustedReference};
use dom::bindings::trace::{JSTraceable};
use dom::browsingcontext::BrowsingContext;
use dom::create::create_element_simple;
use dom::document::{Document, DocumentProgressHandler, DocumentSource, IsHTMLDocument};
use dom::element::{Element, ElementCreator};
use dom::event::{Event, EventBubbles, EventCancelable};
use dom::htmlanchorelement::HTMLAnchorElement;
use dom::node::{Node, NodeDamage, window_from_node};
use dom::text::Text;
use dom::uievent::UIEvent;
use dom::window::{ReflowReason, Window};
use euclid::Rect;
use euclid::point::Point2D;
use gfx_traits::LayerId;
use hyper::method::Method;
use ipc_channel::ipc::{self, IpcSender};
use ipc_channel::router::ROUTER;
use layout_interface::{ReflowQueryType};
use layout_interface::{self, LayoutChan, ScriptLayoutChan};
use mem::heap_size_of_self_and_children;
use msg::constellation_msg::{ConstellationChan, LoadData};
use msg::constellation_msg::{PipelineId, PipelineNamespace};
use msg::constellation_msg::{SubpageId, WindowSizeData};
use net_traits::image_cache_thread::{ImageCacheChan, ImageCacheResult, ImageCacheThread};
use net_traits::storage_thread::StorageThread;
use net_traits::{ResourceThread};
use page::{Frame, IterablePage, Page};
use profile_traits::mem::{self, OpaqueSender, Report, ReportKind, ReportsChan};
use profile_traits::time::{self, ProfilerCategory, profile};
use script_traits::CompositorEvent::{KeyEvent, MouseButtonEvent, MouseMoveEvent, ResizeEvent};
use script_traits::CompositorEvent::{TouchEvent};
use script_traits::{CompositorEvent, ConstellationControlMsg, EventResult};
use script_traits::{InitialScriptState, MouseButton, MouseEventType};
use script_traits::{LayoutMsg, OpaqueScriptLayoutChannel, ScriptMsg as ConstellationMsg};
use script_traits::{ScriptThreadFactory, ScriptToCompositorMsg, TimerEvent, TimerEventRequest, TimerSource};
use script_traits::{TouchEventType, TouchId};
use std::any::Any;
use std::borrow::ToOwned;
use std::cell::{RefCell};
use std::collections::HashSet;
use std::option::Option;
use std::rc::Rc;
use std::result::Result;
use std::sync::atomic::{Ordering, AtomicBool};
use std::sync::mpsc::{Receiver, Select, Sender, channel};
use std::sync::{Arc};
use style::context::ReflowGoal;
use task_source::TaskSource;
use task_source::dom_manipulation::{DOMManipulationTaskSource, DOMManipulationTask};
use task_source::file_reading::FileReadingTaskSource;
use task_source::history_traversal::HistoryTraversalTaskSource;
use task_source::networking::NetworkingTaskSource;
use task_source::user_interaction::UserInteractionTaskSource;
use url::Url;
use util::opts;
use util::str::DOMString;
use util::thread;
use util::thread_state;

thread_local!(static SCRIPT_THREAD_ROOT: RefCell<Option<*const ScriptThread>> = RefCell::new(None));


/// A document load that is in the process of fetching the requested resource. Contains
/// data that will need to be present when the document and frame tree entry are created,
/// but is only easily available at initiation of the load and on a push basis (so some
/// data will be updated according to future resize events, viewport changes, etc.)
#[derive(JSTraceable)]
struct InProgressLoad {
    /// The pipeline which requested this load.
    pipeline_id: PipelineId,
    /// The parent pipeline and child subpage associated with this load, if any.
    parent_info: Option<(PipelineId, SubpageId)>,
    /// The current window size associated with this pipeline.
    window_size: Option<WindowSizeData>,
    /// Channel to the layout thread associated with this pipeline.
    layout_chan: LayoutChan,
    /// The current viewport clipping rectangle applying to this pipeline, if any.
    clip_rect: Option<Rect<f32>>,
    /// Window is frozen (navigated away while loading for example).
    is_frozen: bool,
    /// The requested URL of the load.
    url: Url,
}

impl InProgressLoad {
    /// Create a new InProgressLoad object.
    fn new(id: PipelineId,
           parent_info: Option<(PipelineId, SubpageId)>,
           layout_chan: LayoutChan,
           window_size: Option<WindowSizeData>,
           url: Url) -> InProgressLoad {
        InProgressLoad {
            pipeline_id: id,
            parent_info: parent_info,
            layout_chan: layout_chan,
            window_size: window_size,
            clip_rect: None,
            is_frozen: false,
            url: url,
        }
    }
}

/// Encapsulated state required to create cancellable runnables from non-script threads.
pub struct RunnableWrapper {
    pub cancelled: Arc<AtomicBool>,
}

impl RunnableWrapper {
    pub fn wrap_runnable<T: Runnable + Send + 'static>(&self, runnable: T) -> Box<Runnable + Send> {
        box CancellableRunnable {
            cancelled: self.cancelled.clone(),
            inner: box runnable,
        }
    }
}

/// A runnable that can be discarded by toggling a shared flag.
pub struct CancellableRunnable<T: Runnable + Send> {
    cancelled: Arc<AtomicBool>,
    inner: Box<T>,
}

impl<T: Runnable + Send> Runnable for CancellableRunnable<T> {
    fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Relaxed)
    }

    fn handler(self: Box<CancellableRunnable<T>>) {
        self.inner.handler()
    }
}

pub trait Runnable {
    fn is_cancelled(&self) -> bool { false }
    fn handler(self: Box<Self>);
}

pub trait MainThreadRunnable {
    fn handler(self: Box<Self>, script_thread: &ScriptThread);
}

enum MixedMessage {
    FromConstellation(ConstellationControlMsg),
    FromScript(MainThreadScriptMsg),
    FromDevtools(DevtoolScriptControlMsg),
    FromImageCache(ImageCacheResult),
    FromScheduler(TimerEvent),
}

/// Common messages used to control the event loops in both the script and the worker
pub enum CommonScriptMsg {
    /// Requests that the script thread measure its memory usage. The results are sent back via the
    /// supplied channel.
    CollectReports(ReportsChan),
    /// A DOM object's last pinned reference was removed (dispatched to all threads).
    RefcountCleanup(TrustedReference),
    /// Generic message that encapsulates event handling.
    RunnableMsg(ScriptThreadEventCategory, Box<Runnable + Send>),
}

#[derive(Clone, Copy, Debug, Eq, Hash, JSTraceable, PartialEq)]
pub enum ScriptThreadEventCategory {
    AttachLayout,
    ConstellationMsg,
    DevtoolsMsg,
    DocumentEvent,
    DomEvent,
    FileRead,
    FormPlannedNavigation,
    ImageCacheMsg,
    InputEvent,
    NetworkEvent,
    Resize,
    ScriptEvent,
    SetViewport,
    StylesheetLoad,
    TimerEvent,
    UpdateReplacedElement,
    WebSocketEvent,
    WorkerEvent,
}

/// Messages used to control the script event loop
pub enum MainThreadScriptMsg {
    /// Common variants associated with the script messages
    Common(CommonScriptMsg),
    /// Notify a document that all pending loads are complete.
    DocumentLoadsComplete(PipelineId),
    /// Notifies the script that a window associated with a particular pipeline
    /// should be closed (only dispatched to ScriptThread).
    ExitWindow(PipelineId),
    /// Begins a content-initiated load on the specified pipeline (only
    /// dispatched to ScriptThread).
    Navigate(PipelineId, LoadData),
    /// Tasks that originate from the DOM manipulation task source
    DOMManipulation(DOMManipulationTask),
}

/// A cloneable interface for communicating with an event loop.
pub trait ScriptChan {
    /// Send a message to the associated event loop.
    fn send(&self, msg: CommonScriptMsg) -> Result<(), ()>;
    /// Clone this handle.
    fn clone(&self) -> Box<ScriptChan + Send>;
}

impl OpaqueSender<CommonScriptMsg> for Box<ScriptChan + Send> {
    fn send(&self, msg: CommonScriptMsg) {
        ScriptChan::send(&**self, msg).unwrap();
    }
}

/// An interface for receiving ScriptMsg values in an event loop. Used for synchronous DOM
/// APIs that need to abstract over multiple kinds of event loops (worker/main thread) with
/// different Receiver interfaces.
pub trait ScriptPort {
    fn recv(&self) -> CommonScriptMsg;
}

impl ScriptPort for Receiver<CommonScriptMsg> {
    fn recv(&self) -> CommonScriptMsg {
        self.recv().unwrap()
    }
}

impl ScriptPort for Receiver<MainThreadScriptMsg> {
    fn recv(&self) -> CommonScriptMsg {
        match self.recv().unwrap() {
            MainThreadScriptMsg::Common(script_msg) => script_msg,
            _ => panic!("unexpected main thread event message!")
        }
    }
}

/// Encapsulates internal communication of shared messages within the script thread.
#[derive(JSTraceable)]
pub struct SendableMainThreadScriptChan(pub Sender<CommonScriptMsg>);

impl ScriptChan for SendableMainThreadScriptChan {
    fn send(&self, msg: CommonScriptMsg) -> Result<(), ()> {
        let SendableMainThreadScriptChan(ref chan) = *self;
        chan.send(msg).map_err(|_| ())
    }

    fn clone(&self) -> Box<ScriptChan + Send> {
        let SendableMainThreadScriptChan(ref chan) = *self;
        box SendableMainThreadScriptChan((*chan).clone())
    }
}

impl SendableMainThreadScriptChan {
    /// Creates a new script chan.
    pub fn new() -> (Receiver<CommonScriptMsg>, Box<SendableMainThreadScriptChan>) {
        let (chan, port) = channel();
        (port, box SendableMainThreadScriptChan(chan))
    }
}

/// Encapsulates internal communication of main thread messages within the script thread.
#[derive(JSTraceable)]
pub struct MainThreadScriptChan(pub Sender<MainThreadScriptMsg>);

impl ScriptChan for MainThreadScriptChan {
    fn send(&self, msg: CommonScriptMsg) -> Result<(), ()> {
        let MainThreadScriptChan(ref chan) = *self;
        chan.send(MainThreadScriptMsg::Common(msg)).map_err(|_| ())
    }

    fn clone(&self) -> Box<ScriptChan + Send> {
        let MainThreadScriptChan(ref chan) = *self;
        box MainThreadScriptChan((*chan).clone())
    }
}

impl MainThreadScriptChan {
    /// Creates a new script chan.
    pub fn new() -> (Receiver<MainThreadScriptMsg>, Box<MainThreadScriptChan>) {
        let (chan, port) = channel();
        (port, box MainThreadScriptChan(chan))
    }
}

/// Information for an entire page. Pages are top-level browsing contexts and can contain multiple
/// frames.
#[derive(JSTraceable)]
// ScriptThread instances are rooted on creation, so this is okay
#[allow(unrooted_must_root)]
pub struct ScriptThread {
    /// A handle to the information pertaining to page layout
    page: DOMRefCell<Option<Rc<Page>>>,
    /// A list of data pertaining to loads that have not yet received a network response
    incomplete_loads: DOMRefCell<Vec<InProgressLoad>>,
    /// A handle to the image cache thread.
    image_cache_thread: ImageCacheThread,
    /// A handle to the resource thread. This is an `Arc` to avoid running out of file descriptors if
    /// there are many iframes.
    resource_thread: Arc<ResourceThread>,
    /// A handle to the storage thread.
    storage_thread: StorageThread,

    /// The port on which the script thread receives messages (load URL, exit, etc.)
    port: Receiver<MainThreadScriptMsg>,
    /// A channel to hand out to script thread-based entities that need to be able to enqueue
    /// events in the event queue.
    chan: MainThreadScriptChan,

    dom_manipulation_task_source: DOMManipulationTaskSource,

    user_interaction_task_source: UserInteractionTaskSource,

    networking_task_source: NetworkingTaskSource,

    history_traversal_task_source: HistoryTraversalTaskSource,

    file_reading_task_source: FileReadingTaskSource,

    /// A channel to hand out to threads that need to respond to a message from the script thread.
    control_chan: IpcSender<ConstellationControlMsg>,

    /// The port on which the constellation and layout threads can communicate with the
    /// script thread.
    control_port: Receiver<ConstellationControlMsg>,

    /// For communicating load url messages to the constellation
    constellation_chan: ConstellationChan<ConstellationMsg>,

    /// For communicating layout messages to the constellation
    layout_to_constellation_chan: ConstellationChan<LayoutMsg>,

    /// A handle to the compositor for communicating ready state messages.
    compositor: DOMRefCell<IpcSender<ScriptToCompositorMsg>>,

    /// The port on which we receive messages from the image cache
    image_cache_port: Receiver<ImageCacheResult>,

    /// The channel on which the image cache can send messages to ourself.
    image_cache_channel: ImageCacheChan,

    /// For providing contact with the time profiler.
    time_profiler_chan: time::ProfilerChan,

    /// For providing contact with the memory profiler.
    mem_profiler_chan: mem::ProfilerChan,

    /// For providing instructions to an optional devtools server.
    devtools_chan: Option<IpcSender<ScriptToDevtoolsControlMsg>>,
    /// For receiving commands from an optional devtools server. Will be ignored if
    /// no such server exists.
    devtools_port: Receiver<DevtoolScriptControlMsg>,
    devtools_sender: IpcSender<DevtoolScriptControlMsg>,

    /// The topmost element over the mouse.
    topmost_mouse_over_target: MutNullableHeap<JS<Element>>,

    /// List of pipelines that have been owned and closed by this script thread.
    closed_pipelines: DOMRefCell<HashSet<PipelineId>>,

    scheduler_chan: IpcSender<TimerEventRequest>,
    timer_event_chan: Sender<TimerEvent>,
    timer_event_port: Receiver<TimerEvent>,

    content_process_shutdown_chan: IpcSender<()>,
}

/// In the event of thread failure, all data on the stack runs its destructor. However, there
/// are no reachable, owning pointers to the DOM memory, so it never gets freed by default
/// when the script thread fails. The ScriptMemoryFailsafe uses the destructor bomb pattern
/// to forcibly tear down the JS compartments for pages associated with the failing ScriptThread.
struct ScriptMemoryFailsafe<'a> {
    owner: Option<&'a ScriptThread>,
}

impl<'a> ScriptMemoryFailsafe<'a> {
    fn neuter(&mut self) {
        self.owner = None;
    }

    fn new(owner: &'a ScriptThread) -> ScriptMemoryFailsafe<'a> {
        ScriptMemoryFailsafe {
            owner: Some(owner),
        }
    }
}

impl<'a> Drop for ScriptMemoryFailsafe<'a> {
    #[allow(unrooted_must_root)]
    fn drop(&mut self) {
        match self.owner {
            Some(owner) => {
                unsafe {
                    let page = owner.page.borrow_for_script_deallocation();
                    for page in page.iter() {
                        let window = page.window();
                        window.clear_js_runtime_for_script_deallocation();
                    }
                }
            }
            None => (),
        }
    }
}

impl ScriptThreadFactory for ScriptThread {
    fn create_layout_channel(_phantom: Option<&mut ScriptThread>) -> OpaqueScriptLayoutChannel {
        let (chan, port) = channel();
        ScriptLayoutChan::new(chan, port)
    }

    fn clone_layout_channel(_phantom: Option<&mut ScriptThread>, pair: &OpaqueScriptLayoutChannel)
                            -> Box<Any + Send> {
        box pair.sender() as Box<Any + Send>
    }

    fn create(_phantom: Option<&mut ScriptThread>,
              state: InitialScriptState,
              layout_chan: &OpaqueScriptLayoutChannel,
              load_data: LoadData) {
        let ConstellationChan(const_chan) = state.constellation_chan.clone();
        let (script_chan, script_port) = channel();
        let layout_chan = LayoutChan(layout_chan.sender());
        let failure_info = state.failure_info;
        thread::spawn_named_with_send_on_failure(format!("ScriptThread {:?}", state.id),
                                               thread_state::SCRIPT,
                                               move || {
            PipelineNamespace::install(state.pipeline_namespace_id);
            let chan = MainThreadScriptChan(script_chan.clone());
            let channel_for_reporter = chan.clone();
            let id = state.id;
            let parent_info = state.parent_info;
            let mem_profiler_chan = state.mem_profiler_chan.clone();
            let window_size = state.window_size;
            let script_thread = ScriptThread::new(state,
                                              script_port,
                                              script_chan);

            SCRIPT_THREAD_ROOT.with(|root| {
                *root.borrow_mut() = Some(&script_thread as *const _);
            });

            let mut failsafe = ScriptMemoryFailsafe::new(&script_thread);

            let new_load = InProgressLoad::new(id, parent_info, layout_chan, window_size,
                                               load_data.url.clone());
            script_thread.initialize_default_content(new_load);

            let reporter_name = format!("script-reporter-{}", id);
            mem_profiler_chan.run_with_memory_reporting(|| {
                script_thread.start();
                let _ = script_thread.content_process_shutdown_chan.send(());
            }, reporter_name, channel_for_reporter, CommonScriptMsg::CollectReports);

            // This must always be the very last operation performed before the thread completes
            failsafe.neuter();
        }, ConstellationMsg::Failure(failure_info), const_chan);
    }
}

impl ScriptThread {

    pub fn parsing_complete(id: PipelineId) {
        SCRIPT_THREAD_ROOT.with(|root| {
            let script_thread = unsafe { &*root.borrow().unwrap() };
            script_thread.handle_parsing_complete(id);
        });
    }

    pub fn process_event(msg: CommonScriptMsg) {
        SCRIPT_THREAD_ROOT.with(|root| {
            if let Some(script_thread) = *root.borrow() {
                let script_thread = unsafe { &*script_thread };
                script_thread.handle_msg_from_script(MainThreadScriptMsg::Common(msg));
            }
        });
    }

    /// Creates a new script thread.
    pub fn new(state: InitialScriptState,
               port: Receiver<MainThreadScriptMsg>,
               chan: Sender<MainThreadScriptMsg>)
               -> ScriptThread {

        // Ask the router to proxy IPC messages from the devtools to us.
        let (ipc_devtools_sender, ipc_devtools_receiver) = ipc::channel().unwrap();
        let devtools_port = ROUTER.route_ipc_receiver_to_new_mpsc_receiver(ipc_devtools_receiver);

        // Ask the router to proxy IPC messages from the image cache thread to us.
        let (ipc_image_cache_channel, ipc_image_cache_port) = ipc::channel().unwrap();
        let image_cache_port =
            ROUTER.route_ipc_receiver_to_new_mpsc_receiver(ipc_image_cache_port);

        let (timer_event_chan, timer_event_port) = channel();

        // Ask the router to proxy IPC messages from the control port to us.
        let control_port = ROUTER.route_ipc_receiver_to_new_mpsc_receiver(state.control_port);

        ScriptThread {
            page: DOMRefCell::new(None),
            incomplete_loads: DOMRefCell::new(vec!()),

            image_cache_thread: state.image_cache_thread,
            image_cache_channel: ImageCacheChan(ipc_image_cache_channel),
            image_cache_port: image_cache_port,

            resource_thread: Arc::new(state.resource_thread),
            storage_thread: state.storage_thread,

            port: port,
            chan: MainThreadScriptChan(chan.clone()),
            dom_manipulation_task_source: DOMManipulationTaskSource(chan.clone()),
            user_interaction_task_source: UserInteractionTaskSource(chan.clone()),
            networking_task_source: NetworkingTaskSource(chan.clone()),
            history_traversal_task_source: HistoryTraversalTaskSource(chan.clone()),
            file_reading_task_source: FileReadingTaskSource(chan),

            control_chan: state.control_chan,
            control_port: control_port,
            constellation_chan: state.constellation_chan,
            layout_to_constellation_chan: state.layout_to_constellation_chan,
            compositor: DOMRefCell::new(state.compositor),
            time_profiler_chan: state.time_profiler_chan,
            mem_profiler_chan: state.mem_profiler_chan,

            devtools_chan: state.devtools_chan,
            devtools_port: devtools_port,
            devtools_sender: ipc_devtools_sender,

            topmost_mouse_over_target: MutNullableHeap::new(Default::default()),
            closed_pipelines: DOMRefCell::new(HashSet::new()),

            scheduler_chan: state.scheduler_chan,
            timer_event_chan: timer_event_chan,
            timer_event_port: timer_event_port,

            content_process_shutdown_chan: state.content_process_shutdown_chan,
        }
    }

    // Return the root page in the frame tree. Panics if it doesn't exist.
    pub fn root_page(&self) -> Rc<Page> {
        self.page.borrow().as_ref().unwrap().clone()
    }

    fn root_page_exists(&self) -> bool {
        self.page.borrow().is_some()
    }

    /// Find a child page of the root page by pipeline id. Returns `None` if the root page does
    /// not exist or the subpage cannot be found.
    fn find_subpage(&self, pipeline_id: PipelineId) -> Option<Rc<Page>> {
        self.page.borrow().as_ref().and_then(|page| page.find(pipeline_id))
    }

    /// Starts the script thread. After calling this method, the script thread will loop receiving
    /// messages on its port.
    pub fn start(&self) {
        while self.handle_msgs() {
            // Go on...
        }
    }

    /// Handle incoming control messages.
    fn handle_msgs(&self) -> bool {
        use self::MixedMessage::{FromScript, FromConstellation, FromScheduler, FromDevtools, FromImageCache};

        // Handle pending resize events.
        // Gather them first to avoid a double mut borrow on self.
        let mut resizes = vec!();

        {
            let page = self.page.borrow();
            if let Some(page) = page.as_ref() {
                for page in page.iter() {
                    // Only process a resize if layout is idle.
                    let window = page.window();
                    let resize_event = window.steal_resize_event();
                    match resize_event {
                        Some(size) => resizes.push((window.pipeline(), size)),
                        None => ()
                    }
                }
            }
        }

        for (id, size) in resizes {
            self.handle_event(id, ResizeEvent(size));
        }

        // Store new resizes, and gather all other events.
        let mut sequential = vec!();

        // Receive at least one message so we don't spinloop.
        let mut event = {
            let sel = Select::new();
            let mut script_port = sel.handle(&self.port);
            let mut control_port = sel.handle(&self.control_port);
            let mut timer_event_port = sel.handle(&self.timer_event_port);
            let mut devtools_port = sel.handle(&self.devtools_port);
            let mut image_cache_port = sel.handle(&self.image_cache_port);
            unsafe {
                script_port.add();
                control_port.add();
                timer_event_port.add();
                if self.devtools_chan.is_some() {
                    devtools_port.add();
                }
                image_cache_port.add();
            }
            let ret = sel.wait();
            if ret == script_port.id() {
                FromScript(self.port.recv().unwrap())
            } else if ret == control_port.id() {
                FromConstellation(self.control_port.recv().unwrap())
            } else if ret == timer_event_port.id() {
                FromScheduler(self.timer_event_port.recv().unwrap())
            } else if ret == devtools_port.id() {
                FromDevtools(self.devtools_port.recv().unwrap())
            } else if ret == image_cache_port.id() {
                FromImageCache(self.image_cache_port.recv().unwrap())
            } else {
                panic!("unexpected select result")
            }
        };

        // Squash any pending resize, reflow, animation tick, and mouse-move events in the queue.
        let mut mouse_move_event_index = None;
        let mut animation_ticks = HashSet::new();
        loop {
            match event {
                // This has to be handled before the ResizeMsg below,
                // otherwise the page may not have been added to the
                // child list yet, causing the find() to fail.
                FromConstellation(ConstellationControlMsg::AttachLayout(_)) => {
                    self.profile_event(ScriptThreadEventCategory::AttachLayout, || {
                        assert!(false);
                    })
                }
                FromConstellation(ConstellationControlMsg::Resize(id, size)) => {
                    self.profile_event(ScriptThreadEventCategory::Resize, || {
                        self.handle_resize(id, size);
                    })
                }
                FromConstellation(ConstellationControlMsg::Viewport(id, rect)) => {
                    self.profile_event(ScriptThreadEventCategory::SetViewport, || {
                        self.handle_viewport(id, rect);
                    })
                }
                FromConstellation(ConstellationControlMsg::TickAllAnimations(
                        pipeline_id)) => {
                    if !animation_ticks.contains(&pipeline_id) {
                        animation_ticks.insert(pipeline_id);
                        sequential.push(event);
                    }
                }
                FromConstellation(ConstellationControlMsg::SendEvent(
                        _,
                        MouseMoveEvent(_))) => {
                    match mouse_move_event_index {
                        None => {
                            mouse_move_event_index = Some(sequential.len());
                            sequential.push(event);
                        }
                        Some(index) => {
                            sequential[index] = event
                        }
                    }
                }
                _ => {
                    sequential.push(event);
                }
            }

            // If any of our input sources has an event pending, we'll perform another iteration
            // and check for more resize events. If there are no events pending, we'll move
            // on and execute the sequential non-resize events we've seen.
            match self.control_port.try_recv() {
                Err(_) => match self.port.try_recv() {
                    Err(_) => match self.timer_event_port.try_recv() {
                        Err(_) => match self.devtools_port.try_recv() {
                            Err(_) => match self.image_cache_port.try_recv() {
                                Err(_) => break,
                                Ok(ev) => event = FromImageCache(ev),
                            },
                            Ok(ev) => event = FromDevtools(ev),
                        },
                        Ok(ev) => event = FromScheduler(ev),
                    },
                    Ok(ev) => event = FromScript(ev),
                },
                Ok(ev) => event = FromConstellation(ev),
            }
        }

        // Process the gathered events.
        for msg in sequential {
            let category = self.categorize_msg(&msg);

            let result = self.profile_event(category, move || {
                match msg {
                    FromConstellation(ConstellationControlMsg::ExitPipeline(id)) => {
                        if self.handle_exit_pipeline_msg(id) {
                            return Some(false)
                        }
                    },
                    FromConstellation(inner_msg) => self.handle_msg_from_constellation(inner_msg),
                    FromScript(inner_msg) => self.handle_msg_from_script(inner_msg),
                    FromScheduler(inner_msg) => self.handle_timer_event(inner_msg),
                    FromDevtools(inner_msg) => self.handle_msg_from_devtools(inner_msg),
                    FromImageCache(inner_msg) => self.handle_msg_from_image_cache(inner_msg),
                }

                None
            });

            if let Some(retval) = result {
                return retval
            }
        }

        // Issue batched reflows on any pages that require it (e.g. if images loaded)
        // TODO(gw): In the future we could probably batch other types of reflows
        // into this loop too, but for now it's only images.
        let page = self.page.borrow();
        if let Some(page) = page.as_ref() {
            for page in page.iter() {
                let window = page.window();
                let pending_reflows = window.get_pending_reflow_count();
                if pending_reflows > 0 {
                    window.reflow(ReflowGoal::ForDisplay,
                                  ReflowQueryType::NoQuery,
                                  ReflowReason::ImageLoaded);
                } else {
                    // Reflow currently happens when explicitly invoked by code that
                    // knows the document could have been modified. This should really
                    // be driven by the compositor on an as-needed basis instead, to
                    // minimize unnecessary work.
                    window.reflow(ReflowGoal::ForDisplay,
                                  ReflowQueryType::NoQuery,
                                  ReflowReason::MissingExplicitReflow);
                }
            }
        }

        true
    }

    fn categorize_msg(&self, msg: &MixedMessage) -> ScriptThreadEventCategory {
        match *msg {
            MixedMessage::FromConstellation(ref inner_msg) => {
                match *inner_msg {
                    ConstellationControlMsg::SendEvent(_, _) =>
                        ScriptThreadEventCategory::DomEvent,
                    _ => ScriptThreadEventCategory::ConstellationMsg
                }
            },
            MixedMessage::FromDevtools(_) => ScriptThreadEventCategory::DevtoolsMsg,
            MixedMessage::FromImageCache(_) => ScriptThreadEventCategory::ImageCacheMsg,
            MixedMessage::FromScript(ref inner_msg) => {
                match *inner_msg {
                    MainThreadScriptMsg::Common(CommonScriptMsg::RunnableMsg(ref category, _)) =>
                        *category,
                    _ => ScriptThreadEventCategory::ScriptEvent
                }
            },
            MixedMessage::FromScheduler(_) => ScriptThreadEventCategory::TimerEvent,
        }
    }

    fn profile_event<F, R>(&self, category: ScriptThreadEventCategory, f: F) -> R
        where F: FnOnce() -> R {

        if opts::get().profile_script_events {
            let profiler_cat = match category {
                ScriptThreadEventCategory::AttachLayout => ProfilerCategory::ScriptAttachLayout,
                ScriptThreadEventCategory::ConstellationMsg => ProfilerCategory::ScriptConstellationMsg,
                ScriptThreadEventCategory::DevtoolsMsg => ProfilerCategory::ScriptDevtoolsMsg,
                ScriptThreadEventCategory::DocumentEvent => ProfilerCategory::ScriptDocumentEvent,
                ScriptThreadEventCategory::DomEvent => ProfilerCategory::ScriptDomEvent,
                ScriptThreadEventCategory::FileRead => ProfilerCategory::ScriptFileRead,
                ScriptThreadEventCategory::FormPlannedNavigation => ProfilerCategory::ScriptPlannedNavigation,
                ScriptThreadEventCategory::ImageCacheMsg => ProfilerCategory::ScriptImageCacheMsg,
                ScriptThreadEventCategory::InputEvent => ProfilerCategory::ScriptInputEvent,
                ScriptThreadEventCategory::NetworkEvent => ProfilerCategory::ScriptNetworkEvent,
                ScriptThreadEventCategory::Resize => ProfilerCategory::ScriptResize,
                ScriptThreadEventCategory::ScriptEvent => ProfilerCategory::ScriptEvent,
                ScriptThreadEventCategory::UpdateReplacedElement => {
                    ProfilerCategory::ScriptUpdateReplacedElement
                }
                ScriptThreadEventCategory::StylesheetLoad => ProfilerCategory::ScriptStylesheetLoad,
                ScriptThreadEventCategory::SetViewport => ProfilerCategory::ScriptSetViewport,
                ScriptThreadEventCategory::TimerEvent => ProfilerCategory::ScriptTimerEvent,
                ScriptThreadEventCategory::WebSocketEvent => ProfilerCategory::ScriptWebSocketEvent,
                ScriptThreadEventCategory::WorkerEvent => ProfilerCategory::ScriptWorkerEvent,
            };
            profile(profiler_cat, None, self.time_profiler_chan.clone(), f)
        } else {
            f()
        }
    }

    fn handle_msg_from_constellation(&self, msg: ConstellationControlMsg) {
        match msg {
            ConstellationControlMsg::AttachLayout(_) =>
                panic!("should have handled AttachLayout already"),
            ConstellationControlMsg::Navigate(pipeline_id, subpage_id, load_data) =>
                self.handle_navigate(pipeline_id, Some(subpage_id), load_data),
            ConstellationControlMsg::SendEvent(id, event) =>
                self.handle_event(id, event),
            ConstellationControlMsg::ResizeInactive(id, new_size) =>
                self.handle_resize_inactive_msg(id, new_size),
            ConstellationControlMsg::Viewport(..) =>
                panic!("should have handled Viewport already"),
            ConstellationControlMsg::Resize(..) =>
                panic!("should have handled Resize already"),
            ConstellationControlMsg::ExitPipeline(..) =>
                panic!("should have handled ExitPipeline already"),
            ConstellationControlMsg::GetTitle(pipeline_id) =>
                self.handle_get_title_msg(pipeline_id),
            ConstellationControlMsg::Freeze(pipeline_id) =>
                self.handle_freeze_msg(pipeline_id),
            ConstellationControlMsg::Thaw(pipeline_id) =>
                self.handle_thaw_msg(pipeline_id),
            ConstellationControlMsg::MozBrowserEvent(_,_,_) => {},
            ConstellationControlMsg::UpdateSubpageId(_,_,_) => {},
            ConstellationControlMsg::FocusIFrame(_,_) => {},
            ConstellationControlMsg::WebDriverScriptCommand(_, _) => {},
            ConstellationControlMsg::TickAllAnimations(pipeline_id) =>
                self.handle_tick_all_animations(pipeline_id),
            ConstellationControlMsg::WebFontLoaded(pipeline_id) =>
                self.handle_web_font_loaded(pipeline_id),
            ConstellationControlMsg::DispatchFrameLoadEvent { target: _, parent: _ } => {},
            ConstellationControlMsg::FramedContentChanged(_,_) => {},
            ConstellationControlMsg::ReportCSSError(pipeline_id, filename, line, column, msg) =>
                self.handle_css_error_reporting(pipeline_id, filename, line, column, msg),
        }
    }

    fn handle_msg_from_script(&self, msg: MainThreadScriptMsg) {
        match msg {
            MainThreadScriptMsg::Navigate(id, load_data) =>
                self.handle_navigate(id, None, load_data),
            MainThreadScriptMsg::ExitWindow(id) =>
                self.handle_exit_window_msg(id),
            MainThreadScriptMsg::DocumentLoadsComplete(id) =>
                self.handle_loads_complete(id),
            MainThreadScriptMsg::Common(CommonScriptMsg::RunnableMsg(_, runnable)) => {
                // The category of the runnable is ignored by the pattern, however
                // it is still respected by profiling (see categorize_msg).
                if !runnable.is_cancelled() {
                    runnable.handler()
                }
            }
            MainThreadScriptMsg::Common(CommonScriptMsg::RefcountCleanup(addr)) =>
                LiveDOMReferences::cleanup(addr),
            MainThreadScriptMsg::Common(CommonScriptMsg::CollectReports(reports_chan)) =>
                self.collect_reports(reports_chan),
            MainThreadScriptMsg::DOMManipulation(msg) =>
                msg.handle_msg(self),
        }
    }

    fn handle_timer_event(&self, timer_event: TimerEvent) {
        let TimerEvent(source, id) = timer_event;

        let pipeline_id = match source {
            TimerSource::FromWindow(pipeline_id) => pipeline_id,
            TimerSource::FromWorker => panic!("Worker timeouts must not be sent to script thread"),
        };

        let page = self.root_page();
        let page = page.find(pipeline_id).expect("ScriptThread: received fire timer msg for a
            pipeline ID not associated with this script thread. This is a bug.");
        let window = page.window();

        window.handle_fire_timer(id);
    }

    fn handle_msg_from_devtools(&self, msg: DevtoolScriptControlMsg) {
        let page = self.root_page();
        match msg {
            DevtoolScriptControlMsg::EvaluateJS(_,_,_) => {},
            DevtoolScriptControlMsg::GetRootNode(id, reply) =>
                devtools::handle_get_root_node(&page, id, reply),
            DevtoolScriptControlMsg::GetDocumentElement(id, reply) =>
                devtools::handle_get_document_element(&page, id, reply),
            DevtoolScriptControlMsg::GetChildren(id, node_id, reply) =>
                devtools::handle_get_children(&page, id, node_id, reply),
            DevtoolScriptControlMsg::GetLayout(id, node_id, reply) =>
                devtools::handle_get_layout(&page, id, node_id, reply),
            DevtoolScriptControlMsg::GetCachedMessages(pipeline_id, message_types, reply) =>
                devtools::handle_get_cached_messages(pipeline_id, message_types, reply),
            DevtoolScriptControlMsg::ModifyAttribute(id, node_id, modifications) =>
                devtools::handle_modify_attribute(&page, id, node_id, modifications),
            DevtoolScriptControlMsg::WantsLiveNotifications(id, to_send) => {
                let window = get_page(&page, id).window();
                let global_ref = GlobalRef::Window(window.r());
                devtools::handle_wants_live_notifications(&global_ref, to_send)
            },
            DevtoolScriptControlMsg::SetTimelineMarkers(_pipeline_id, marker_types, reply) =>
                devtools::handle_set_timeline_markers(&page, marker_types, reply),
            DevtoolScriptControlMsg::DropTimelineMarkers(_pipeline_id, marker_types) =>
                devtools::handle_drop_timeline_markers(&page, marker_types),
            DevtoolScriptControlMsg::RequestAnimationFrame(pipeline_id, name) =>
                devtools::handle_request_animation_frame(&page, pipeline_id, name),
        }
    }

    fn handle_msg_from_image_cache(&self, msg: ImageCacheResult) {
        msg.responder.unwrap().respond(msg.image_response);
    }

    fn handle_resize(&self, id: PipelineId, size: WindowSizeData) {
        if let Some(ref page) = self.find_subpage(id) {
            let window = page.window();
            window.set_resize_event(size);
            return;
        }
        let mut loads = self.incomplete_loads.borrow_mut();
        if let Some(ref mut load) = loads.iter_mut().find(|load| load.pipeline_id == id) {
            load.window_size = Some(size);
            return;
        }
        panic!("resize sent to nonexistent pipeline");
    }

    fn handle_viewport(&self, id: PipelineId, rect: Rect<f32>) {
        let page = self.page.borrow();
        if let Some(page) = page.as_ref() {
            if let Some(ref inner_page) = page.find(id) {
                let window = inner_page.window();
                if window.set_page_clip_rect_with_new_viewport(rect) {
                    let page = get_page(page, id);
                    self.rebuild_and_force_reflow(&*page, ReflowReason::Viewport);
                }
                return;
            }
        }
        let mut loads = self.incomplete_loads.borrow_mut();
        if let Some(ref mut load) = loads.iter_mut().find(|load| load.pipeline_id == id) {
            load.clip_rect = Some(rect);
            return;
        }
        panic!("Page rect message sent to nonexistent pipeline");
    }

    fn handle_loads_complete(&self, pipeline: PipelineId) {
        let page = get_page(&self.root_page(), pipeline);
        let doc = page.document();
        let doc = doc.r();
        if doc.loader().is_blocked() {
            return;
        }

        doc.mut_loader().inhibit_events();

        // https://html.spec.whatwg.org/multipage/#the-end step 7
        let addr: Trusted<Document> = Trusted::new(doc, self.chan.clone());
        let handler = box DocumentProgressHandler::new(addr.clone());
        self.dom_manipulation_task_source.queue(DOMManipulationTask::DocumentProgress(handler)).unwrap();

        let ConstellationChan(ref chan) = self.constellation_chan;
        chan.send(ConstellationMsg::LoadComplete(pipeline)).unwrap();
    }

    fn collect_reports(&self, reports_chan: ReportsChan) {
        let mut urls = vec![];
        let mut dom_tree_size = 0;
        let mut reports = vec![];

        if let Some(root_page) = self.page.borrow().as_ref() {
            for it_page in root_page.iter() {
                let current_url = it_page.document().url().serialize();
                urls.push(current_url.clone());

                for child in it_page.document().upcast::<Node>().traverse_preorder() {
                    dom_tree_size += heap_size_of_self_and_children(&*child);
                }
                let window = it_page.window();
                dom_tree_size += heap_size_of_self_and_children(&*window);

                reports.push(Report {
                    path: path![format!("url({})", current_url), "dom-tree"],
                    kind: ReportKind::ExplicitJemallocHeapSize,
                    size: dom_tree_size,
                })
            }
        }
        reports_chan.send(reports);
    }

    /// Handles freeze message
    fn handle_freeze_msg(&self, id: PipelineId) {
        if let Some(root_page) = self.page.borrow().as_ref() {
            if let Some(ref inner_page) = root_page.find(id) {
                let window = inner_page.window();
                window.freeze();
                return;
            }
        }
        let mut loads = self.incomplete_loads.borrow_mut();
        if let Some(ref mut load) = loads.iter_mut().find(|load| load.pipeline_id == id) {
            load.is_frozen = true;
            return;
        }
        panic!("freeze sent to nonexistent pipeline");
    }

    /// Handles thaw message
    fn handle_thaw_msg(&self, id: PipelineId) {
        if let Some(ref inner_page) = self.root_page().find(id) {
            let needed_reflow = inner_page.set_reflow_status(false);
            if needed_reflow {
                self.rebuild_and_force_reflow(&*inner_page, ReflowReason::CachedPageNeededReflow);
            }
            let window = inner_page.window();
            window.thaw();
            return;
        }
        let mut loads = self.incomplete_loads.borrow_mut();
        if let Some(ref mut load) = loads.iter_mut().find(|load| load.pipeline_id == id) {
            load.is_frozen = false;
            return;
        }
        panic!("thaw sent to nonexistent pipeline");
    }

    /// Window was resized, but this script was not active, so don't reflow yet
    fn handle_resize_inactive_msg(&self, id: PipelineId, new_size: WindowSizeData) {
        let page = self.root_page();
        let page = page.find(id).expect("Received resize message for PipelineId not associated
            with a page in the page tree. This is a bug.");
        let window = page.window();
        window.set_window_size(new_size);
        page.set_reflow_status(true);
    }

    /// We have gotten a window.close from script, which we pass on to the compositor.
    /// We do not shut down the script thread now, because the compositor will ask the
    /// constellation to shut down the pipeline, which will clean everything up
    /// normally. If we do exit, we will tear down the DOM nodes, possibly at a point
    /// where layout is still accessing them.
    fn handle_exit_window_msg(&self, _: PipelineId) {
        debug!("script thread handling exit window msg");

        // TODO(tkuehn): currently there is only one window,
        // so this can afford to be naive and just shut down the
        // compositor. In the future it'll need to be smarter.
        self.compositor.borrow_mut().send(ScriptToCompositorMsg::Exit).unwrap();
    }

    /// Handles a request for the window title.
    fn handle_get_title_msg(&self, pipeline_id: PipelineId) {
        let page = get_page(&self.root_page(), pipeline_id);
        let document = page.document();
        document.send_title_to_compositor();
    }

    /// Handles a request to exit the script thread and shut down layout.
    /// Returns true if the script thread should shut down and false otherwise.
    fn handle_exit_pipeline_msg(&self, id: PipelineId) -> bool {
        self.closed_pipelines.borrow_mut().insert(id);

        // Check if the exit message is for an in progress load.
        let idx = self.incomplete_loads.borrow().iter().position(|load| {
            load.pipeline_id == id
        });

        if let Some(idx) = idx {
            let load = self.incomplete_loads.borrow_mut().remove(idx);

            // Tell the layout thread to begin shutting down, and wait until it
            // processed this message.
            let (response_chan, response_port) = channel();
            let LayoutChan(chan) = load.layout_chan;
            if chan.send(layout_interface::Msg::PrepareToExit(response_chan)).is_ok() {
                debug!("shutting down layout for page {:?}", id);
                response_port.recv().unwrap();
                chan.send(layout_interface::Msg::ExitNow).ok();
            }

            let has_pending_loads = self.incomplete_loads.borrow().len() > 0;
            let has_root_page = self.page.borrow().is_some();

            // Exit if no pending loads and no root page
            return !has_pending_loads && !has_root_page;
        }

        // If root is being exited, shut down all pages
        let page = self.root_page();
        let window = page.window();
        if window.pipeline() == id {
            debug!("shutting down layout for root page {:?}", id);
            shut_down_layout(&page);
            return true
        }

        // otherwise find just the matching page and exit all sub-pages
        if let Some(ref mut child_page) = page.remove(id) {
            shut_down_layout(&*child_page);
        }
        false
    }

    /// Handles when layout thread finishes all animation in one tick
    fn handle_tick_all_animations(&self, id: PipelineId) {
        let page = get_page(&self.root_page(), id);
        let document = page.document();
        document.run_the_animation_frame_callbacks();
    }

    /// Handles a Web font being loaded. Does nothing if the page no longer exists.
    fn handle_web_font_loaded(&self, pipeline_id: PipelineId) {
        if let Some(ref page) = self.find_subpage(pipeline_id)  {
            self.rebuild_and_force_reflow(page, ReflowReason::WebFontLoaded);
        }
    }

    /// Initializes the default window/document into a state that is ready to accept
    /// VDOM patches.
    fn initialize_default_content(&self, incomplete: InProgressLoad) {

        // Create a new frame tree entry.
        let page = Rc::new(Page::new(incomplete.pipeline_id));
        *self.page.borrow_mut() = Some(page.clone());

        let MainThreadScriptChan(ref sender) = self.chan;
        let DOMManipulationTaskSource(ref dom_sender) = self.dom_manipulation_task_source;
        let UserInteractionTaskSource(ref user_sender) = self.user_interaction_task_source;
        let NetworkingTaskSource(ref network_sender) = self.networking_task_source;
        let HistoryTraversalTaskSource(ref history_sender) = self.history_traversal_task_source;
        let FileReadingTaskSource(ref file_sender) = self.file_reading_task_source;

        let (ipc_timer_event_chan, ipc_timer_event_port) = ipc::channel().unwrap();
        ROUTER.route_ipc_receiver_to_mpsc_sender(ipc_timer_event_port,
                                                 self.timer_event_chan.clone());

        // Create the window and document objects.
        let window = Window::new(page.clone(),
                                 MainThreadScriptChan(sender.clone()),
                                 DOMManipulationTaskSource(dom_sender.clone()),
                                 UserInteractionTaskSource(user_sender.clone()),
                                 NetworkingTaskSource(network_sender.clone()),
                                 HistoryTraversalTaskSource(history_sender.clone()),
                                 FileReadingTaskSource(file_sender.clone()),
                                 self.image_cache_channel.clone(),
                                 self.compositor.borrow_mut().clone(),
                                 self.image_cache_thread.clone(),
                                 self.resource_thread.clone(),
                                 self.storage_thread.clone(),
                                 self.mem_profiler_chan.clone(),
                                 self.devtools_chan.clone(),
                                 self.constellation_chan.clone(),
                                 self.control_chan.clone(),
                                 self.scheduler_chan.clone(),
                                 ipc_timer_event_chan,
                                 incomplete.layout_chan,
                                 incomplete.pipeline_id,
                                 None,
                                 incomplete.window_size);

        let browsing_context = BrowsingContext::new(&window, None);
        window.init_browsing_context(&browsing_context);

        let loader = DocumentLoader::new_with_thread(self.resource_thread.clone(),
                                                   Some(page.pipeline()),
                                                   Some(incomplete.url.clone()));

        let content_type = Some(DOMString::from("text/html"));
        let is_html_document = IsHTMLDocument::HTMLDocument;

        let document = Document::new(window.r(),
                                     Some(&browsing_context),
                                     Some(incomplete.url.clone()),
                                     is_html_document,
                                     content_type,
                                     None,
                                     DocumentSource::NotFromParser,
                                     loader);

        browsing_context.init(&document);

        let htmlel = create_element_simple(
            atom!("html"),
            &document,
            ElementCreator::ParserCreated);
        assert!(document.upcast::<Node>().InsertBefore(htmlel.upcast::<Node>(), None).is_ok());

        let bodyel = create_element_simple(
            atom!("body"),
            &document,
            ElementCreator::ParserCreated);
        assert!(htmlel.upcast::<Node>().InsertBefore(bodyel.upcast::<Node>(), None).is_ok());

        let text = Text::new(DOMString::from("Hello World!"), &document);
        assert!(bodyel.upcast::<Node>().InsertBefore(text.upcast(), None).is_ok());

        document.set_ready_state(DocumentReadyState::Complete);

        // Create the root frame
        page.set_frame(Some(Frame {
            document: JS::from_rooted(&document),
            window: JS::from_rooted(&window),
        }));

        let ConstellationChan(ref chan) = self.constellation_chan;
        chan.send(ConstellationMsg::ActivateDocument(incomplete.pipeline_id)).unwrap();

        // Notify devtools that a new script global exists.
        self.notify_devtools(document.Title(), incomplete.url.clone(), (page.pipeline(), None));

        document.content_changed(document.upcast(), NodeDamage::OtherNodeDamage);
        window.reflow(ReflowGoal::ForDisplay, ReflowQueryType::NoQuery, ReflowReason::FirstLoad);
    }

    fn notify_devtools(&self, title: DOMString, url: Url, ids: (PipelineId, Option<WorkerId>)) {
        if let Some(ref chan) = self.devtools_chan {
            let page_info = DevtoolsPageInfo {
                title: String::from(title),
                url: url,
            };
            chan.send(ScriptToDevtoolsControlMsg::NewGlobal(
                        ids,
                        self.devtools_sender.clone(),
                        page_info)).unwrap();
        }
    }

    fn scroll_fragment_point(&self, pipeline_id: PipelineId, element: &Element) {
        // FIXME(#8275, pcwalton): This is pretty bogus when multiple layers are involved.
        // Really what needs to happen is that this needs to go through layout to ask which
        // layer the element belongs to, and have it send the scroll message to the
        // compositor.
        let rect = element.upcast::<Node>().get_bounding_content_box();

        // In order to align with element edges, we snap to unscaled pixel boundaries, since the
        // paint thread currently does the same for drawing elements. This is important for pages
        // that require pixel perfect scroll positioning for proper display (like Acid2). Since we
        // don't have the device pixel ratio here, this might not be accurate, but should work as
        // long as the ratio is a whole number. Once #8275 is fixed this should actually take into
        // account the real device pixel ratio.
        let point = Point2D::new(rect.origin.x.to_nearest_px() as f32,
                                 rect.origin.y.to_nearest_px() as f32);

        self.compositor.borrow_mut().send(ScriptToCompositorMsg::ScrollFragmentPoint(
                                                 pipeline_id, LayerId::null(), point, false)).unwrap();
    }

    /// Reflows non-incrementally, rebuilding the entire layout tree in the process.
    fn rebuild_and_force_reflow(&self, page: &Page, reason: ReflowReason) {
        let document = page.document();
        document.dirty_all_nodes();
        let window = window_from_node(document.r());
        window.reflow(ReflowGoal::ForDisplay, ReflowQueryType::NoQuery, reason);
    }

    /// This is the main entry point for receiving and dispatching DOM events.
    ///
    /// TODO: Actually perform DOM event dispatch.
    fn handle_event(&self, pipeline_id: PipelineId, event: CompositorEvent) {

        // DOM events can only be handled if there's a root page.
        if !self.root_page_exists() {
            return;
        }

        match event {
            ResizeEvent(new_size) => {
                self.handle_resize_event(pipeline_id, new_size);
            }

            MouseButtonEvent(event_type, button, point) => {
                self.handle_mouse_event(pipeline_id, event_type, button, point);
            }

            MouseMoveEvent(point) => {
                let page = get_page(&self.root_page(), pipeline_id);
                let document = page.document();

                // Get the previous target temporarily
                let prev_mouse_over_target = self.topmost_mouse_over_target.get();

                document.handle_mouse_move_event(point,
                                                 &self.topmost_mouse_over_target);

                // Short-circuit if nothing changed
                if self.topmost_mouse_over_target.get() == prev_mouse_over_target {
                    return;
                }

                let mut state_already_changed = false;

                // Notify Constellation about the topmost anchor mouse over target.
                if let Some(target) = self.topmost_mouse_over_target.get() {
                    if let Some(anchor) = target.upcast::<Node>()
                                                .inclusive_ancestors()
                                                .filter_map(Root::downcast::<HTMLAnchorElement>)
                                                .next() {
                        let status = anchor.upcast::<Element>()
                                           .get_attribute(&ns!(), &atom!("href"))
                                           .and_then(|href| {
                                               let value = href.value();
                                               let url = document.url();
                                               url.join(&value).map(|url| url.serialize()).ok()
                                           });

                        let event = ConstellationMsg::NodeStatus(status);
                        let ConstellationChan(ref chan) = self.constellation_chan;
                        chan.send(event).unwrap();

                        state_already_changed = true;
                    }
                }

                // We might have to reset the anchor state
                if !state_already_changed {
                    if let Some(target) = prev_mouse_over_target {
                        if let Some(_) = target.upcast::<Node>()
                                               .inclusive_ancestors()
                                               .filter_map(Root::downcast::<HTMLAnchorElement>)
                                               .next() {
                            let event = ConstellationMsg::NodeStatus(None);
                            let ConstellationChan(ref chan) = self.constellation_chan;
                            chan.send(event).unwrap();
                        }
                    }
                }
            }
            TouchEvent(event_type, identifier, point) => {
                let handled = self.handle_touch_event(pipeline_id, event_type, identifier, point);
                match event_type {
                    TouchEventType::Down => {
                        if handled {
                            // TODO: Wait to see if preventDefault is called on the first touchmove event.
                            self.compositor.borrow_mut()
                                .send(ScriptToCompositorMsg::TouchEventProcessed(
                                        EventResult::DefaultAllowed)).unwrap();
                        } else {
                            self.compositor.borrow_mut()
                                .send(ScriptToCompositorMsg::TouchEventProcessed(
                                        EventResult::DefaultPrevented)).unwrap();
                        }
                    }
                    _ => {
                        // TODO: Calling preventDefault on a touchup event should prevent clicks.
                    }
                }
            }

            KeyEvent(key, state, modifiers) => {
                let page = get_page(&self.root_page(), pipeline_id);
                let document = page.document();
                document.dispatch_key_event(
                    key, state, modifiers, &mut self.compositor.borrow_mut());
            }
        }
    }

    fn handle_mouse_event(&self,
                          pipeline_id: PipelineId,
                          mouse_event_type: MouseEventType,
                          button: MouseButton,
                          point: Point2D<f32>) {
        let page = get_page(&self.root_page(), pipeline_id);
        let document = page.document();
        document.handle_mouse_event(button, point, mouse_event_type);
    }

    fn handle_touch_event(&self,
                          pipeline_id: PipelineId,
                          event_type: TouchEventType,
                          identifier: TouchId,
                          point: Point2D<f32>)
                          -> bool {
        let page = get_page(&self.root_page(), pipeline_id);
        let document = page.document();
        document.handle_touch_event(event_type, identifier, point)
    }

    /// https://html.spec.whatwg.org/multipage/#navigating-across-documents
    /// The entry point for content to notify that a new load has been requested
    /// for the given pipeline (specifically the "navigate" algorithm).
    fn handle_navigate(&self, pipeline_id: PipelineId, subpage_id: Option<SubpageId>, load_data: LoadData) {
        // Step 8.
        {
            let nurl = &load_data.url;
            if let Some(ref fragment) = nurl.fragment {
                let page = get_page(&self.root_page(), pipeline_id);
                let document = page.document();
                let document = document.r();
                let url = document.url();
                if url.scheme == nurl.scheme && url.scheme_data == nurl.scheme_data &&
                    url.query == nurl.query && load_data.method == Method::Get {
                    match document.find_fragment_node(&*fragment) {
                        Some(ref node) => {
                            self.scroll_fragment_point(pipeline_id, node.r());
                        }
                        None => {}
                    }
                    return;
                }
            }
        }

        match subpage_id {
            Some(_) => {},
            None => {
                let ConstellationChan(ref const_chan) = self.constellation_chan;
                const_chan.send(ConstellationMsg::LoadUrl(pipeline_id, load_data)).unwrap();
            }
        }
    }

    fn handle_resize_event(&self, pipeline_id: PipelineId, new_size: WindowSizeData) {
        let page = get_page(&self.root_page(), pipeline_id);
        let window = page.window();
        window.set_window_size(new_size);
        window.force_reflow(ReflowGoal::ForDisplay,
                            ReflowQueryType::NoQuery,
                            ReflowReason::WindowResize);

        let document = page.document();
        let fragment_node = window.steal_fragment_name()
                                  .and_then(|name| document.find_fragment_node(&*name));
        match fragment_node {
            Some(ref node) => self.scroll_fragment_point(pipeline_id, node.r()),
            None => {}
        }

        // http://dev.w3.org/csswg/cssom-view/#resizing-viewports
        // https://dvcs.w3.org/hg/dom3events/raw-file/tip/html/DOM3-Events.html#event-type-resize
        let uievent = UIEvent::new(window.r(),
                                   DOMString::from("resize"), EventBubbles::DoesNotBubble,
                                   EventCancelable::NotCancelable, Some(window.r()),
                                   0i32);
        uievent.upcast::<Event>().fire(window.upcast());
    }


    fn handle_parsing_complete(&self, id: PipelineId) {
        let parent_page = self.root_page();
        let page = match parent_page.find(id) {
            Some(page) => page,
            None => return,
        };

        let document = page.document();
        let final_url = document.url();

        // https://html.spec.whatwg.org/multipage/#the-end step 1
        document.set_ready_state(DocumentReadyState::Interactive);

        // TODO: Execute step 2 here.

        // Kick off the initial reflow of the page.
        debug!("kicking off initial reflow of {:?}", final_url);
        document.disarm_reflow_timeout();
        document.content_changed(document.upcast(),
                                 NodeDamage::OtherNodeDamage);
        let window = window_from_node(document.r());
        window.reflow(ReflowGoal::ForDisplay, ReflowQueryType::NoQuery, ReflowReason::FirstLoad);

        // No more reflow required
        page.set_reflow_status(false);

        window.set_fragment_name(final_url.fragment.clone());
    }

    fn handle_css_error_reporting(&self, pipeline_id: PipelineId, filename: String,
                                  line: usize, column: usize, msg: String) {
        let parent_page = self.root_page();
        let page = match parent_page.find(pipeline_id) {
            Some(page) => page,
            None => return,
        };

        let document = page.document();
        let css_error = CSSError {
            filename: filename,
            line: line,
            column: column,
            msg: msg
        };

        document.report_css_error(css_error.clone());
        let window = page.window();

        if window.live_devtools_updates() {
            if let Some(ref chan) = self.devtools_chan {
                chan.send(ScriptToDevtoolsControlMsg::ReportCSSError(
                    pipeline_id,
                    css_error)).unwrap();
             }
        }
    }
}

impl Drop for ScriptThread {
    fn drop(&mut self) {
        SCRIPT_THREAD_ROOT.with(|root| {
            *root.borrow_mut() = None;
        });
    }
}

/// Shuts down layout for the given page tree.
fn shut_down_layout(page_tree: &Rc<Page>) {
    let mut channels = vec!();

    for page in page_tree.iter() {
        // Tell the layout thread to begin shutting down, and wait until it
        // processed this message.
        let (response_chan, response_port) = channel();
        let window = page.window();
        let LayoutChan(chan) = window.layout_chan();
        if chan.send(layout_interface::Msg::PrepareToExit(response_chan)).is_ok() {
            channels.push(chan);
            response_port.recv().unwrap();
        }
    }

    // Drop our references to the JSContext and DOM objects.
    for page in page_tree.iter() {
        let window = page.window();
        window.clear_js_runtime();
        // Sever the connection between the global and the DOM tree
        page.set_frame(None);
    }

    // Destroy the layout thread. If there were node leaks, layout will now crash safely.
    for chan in channels {
        chan.send(layout_interface::Msg::ExitNow).ok();
    }
}

pub fn get_page(page: &Rc<Page>, pipeline_id: PipelineId) -> Rc<Page> {
    page.find(pipeline_id).expect("ScriptThread: received an event \
        message for a layout channel that is not associated with this script thread.\
         This is a bug.")
}
