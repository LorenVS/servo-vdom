/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! Abstractions for global scopes.
//!
//! This module contains smart pointers to global scopes, to simplify writing
//! code that works in workers as well as window scopes.

use devtools_traits::{ScriptToDevtoolsControlMsg, WorkerId};
use dom::bindings::codegen::Bindings::WindowBinding::WindowMethods;
use dom::bindings::js::Root;
use dom::bindings::reflector::{Reflectable, Reflector};
use dom::window::{self};
use ipc_channel::ipc::IpcSender;
use js::jsapi::GetGlobalForObjectCrossCompartment;
use js::jsapi::{JSObject, JS_GetClass};
use js::{JSCLASS_IS_DOMJSCLASS, JSCLASS_IS_GLOBAL};
use msg::constellation_msg::{ConstellationChan, PipelineId};
use net_traits::ResourceThread;
use profile_traits::mem;
use script_thread::{CommonScriptMsg, MainThreadScriptChan, ScriptChan, ScriptPort, ScriptThread};
use script_traits::{MsDuration, ScriptMsg as ConstellationMsg, TimerEventRequest};
use task_source::TaskSource;
use task_source::dom_manipulation::DOMManipulationTask;
use timers::{OneshotTimerCallback, OneshotTimerHandle};
use url::Url;

/// A freely-copyable reference to a rooted global object.
#[derive(Copy, Clone)]
pub enum GlobalRef<'a> {
    /// A reference to a `Window` object.
    Window(&'a window::Window)
}

/// A stack-based rooted reference to a global object.
pub enum GlobalRoot {
    /// A root for a `Window` object.
    Window(Root<window::Window>)
}

impl<'a> GlobalRef<'a> {

    /// Extract a `Window`, causing thread failure if the global object is not
    /// a `Window`.
    pub fn as_window(&self) -> &window::Window {
        match *self {
            GlobalRef::Window(window) => window
        }
    }

    /// Get the `PipelineId` for this global scope.
    pub fn pipeline(&self) -> PipelineId {
        match *self {
            GlobalRef::Window(window) => window.pipeline()
        }
    }

    /// Get a `mem::ProfilerChan` to send messages to the memory profiler thread.
    pub fn mem_profiler_chan(&self) -> mem::ProfilerChan {
        match *self {
            GlobalRef::Window(window) => window.mem_profiler_chan()
        }
    }

    /// Get a `ConstellationChan` to send messages to the constellation channel when available.
    pub fn constellation_chan(&self) -> ConstellationChan<ConstellationMsg> {
        match *self {
            GlobalRef::Window(window) => window.constellation_chan()
        }
    }

    /// Get the scheduler channel to request timer events.
    pub fn scheduler_chan(&self) -> IpcSender<TimerEventRequest> {
        match *self {
            GlobalRef::Window(window) => window.scheduler_chan()
        }
    }

    /// Get an `IpcSender<ScriptToDevtoolsControlMsg>` to send messages to Devtools
    /// thread when available.
    pub fn devtools_chan(&self) -> Option<IpcSender<ScriptToDevtoolsControlMsg>> {
        match *self {
            GlobalRef::Window(window) => window.devtools_chan()
        }
    }

    /// Get the `ResourceThread` for this global scope.
    pub fn resource_thread(&self) -> ResourceThread {
        match *self {
            GlobalRef::Window(ref window) => {
                let doc = window.Document();
                let doc = doc.r();
                let loader = doc.loader();
                (*loader.resource_thread).clone()
            }
        }
    }

    /// Get the worker's id.
    pub fn get_worker_id(&self) -> Option<WorkerId> {
        match *self {
            GlobalRef::Window(_) => None
        }
    }

    /// Get next worker id.
    pub fn get_next_worker_id(&self) -> WorkerId {
        match *self {
            GlobalRef::Window(ref window) => window.get_next_worker_id()
        }
    }

    /// Get the URL for this global scope.
    pub fn get_url(&self) -> Url {
        match *self {
            GlobalRef::Window(ref window) => window.get_url()
        }
    }

    /// `ScriptChan` used to send messages to the event loop of this global's
    /// thread.
    pub fn script_chan(&self) -> Box<ScriptChan + Send> {
        match *self {
            GlobalRef::Window(ref window) => {
                MainThreadScriptChan(window.main_thread_script_chan().clone()).clone()
            }
        }
    }

    /// `TaskSource` used to queue DOM manipulation messages to the event loop of this global's
    /// thread.
    pub fn dom_manipulation_task_source(&self) -> Box<TaskSource<DOMManipulationTask> + Send> {
        match *self {
            GlobalRef::Window(ref window) => window.dom_manipulation_task_source()
        }
    }

    /// `ScriptChan` used to send messages to the event loop of this global's
    /// thread.
    pub fn user_interaction_task_source(&self) -> Box<ScriptChan + Send> {
        match *self {
            GlobalRef::Window(ref window) => window.user_interaction_task_source()
        }
    }

    /// `ScriptChan` used to send messages to the event loop of this global's
    /// thread.
    pub fn networking_task_source(&self) -> Box<ScriptChan + Send> {
        match *self {
            GlobalRef::Window(ref window) => window.networking_task_source()
        }
    }

    /// `ScriptChan` used to send messages to the event loop of this global's
    /// thread.
    pub fn history_traversal_task_source(&self) -> Box<ScriptChan + Send> {
        match *self {
            GlobalRef::Window(ref window) => window.history_traversal_task_source()
        }
    }

    /// `ScriptChan` used to send messages to the event loop of this global's
    /// thread.
    pub fn file_reading_task_source(&self) -> Box<ScriptChan + Send> {
        match *self {
            GlobalRef::Window(ref window) => window.file_reading_task_source()
        }
    }

    /// Create a new sender/receiver pair that can be used to implement an on-demand
    /// event loop. Used for implementing web APIs that require blocking semantics
    /// without resorting to nested event loops.
    pub fn new_script_pair(&self) -> (Box<ScriptChan + Send>, Box<ScriptPort + Send>) {
        match *self {
            GlobalRef::Window(ref window) => window.new_script_pair()
        }
    }

    /// Process a single event as if it were the next event in the thread queue for
    /// this global.
    pub fn process_event(&self, msg: CommonScriptMsg) {
        match *self {
            GlobalRef::Window(_) => ScriptThread::process_event(msg)
        }
    }

    /// Set the `bool` value to indicate whether developer tools has requested
    /// updates from the global
    pub fn set_devtools_wants_updates(&self, send_updates: bool) {
        match *self {
            GlobalRef::Window(window) => window.set_devtools_wants_updates(send_updates)
        }
    }

    /// Schedule the given `callback` to be invoked after at least `duration` milliseconds have
    /// passed.
    pub fn schedule_callback(&self,
                             callback: OneshotTimerCallback,
                             duration: MsDuration)
                             -> OneshotTimerHandle {
        match *self {
            GlobalRef::Window(window) => window.schedule_callback(callback, duration)
        }
    }

    /// Unschedule a previously-scheduled callback.
    pub fn unschedule_callback(&self, handle: OneshotTimerHandle) {
        match *self {
            GlobalRef::Window(window) => window.unschedule_callback(handle)
        }
    }

    /// Returns the receiver's reflector.
    pub fn reflector(&self) -> &Reflector {
        match *self {
            GlobalRef::Window(ref window) => window.reflector()
        }
    }
}

impl GlobalRoot {
    /// Obtain a safe reference to the global object that cannot outlive the
    /// lifetime of this root.
    pub fn r(&self) -> GlobalRef {
        match *self {
            GlobalRoot::Window(ref window) => GlobalRef::Window(window.r())
        }
    }
}

