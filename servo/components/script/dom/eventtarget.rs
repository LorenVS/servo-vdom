/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::callback::{CallbackContainer, ExceptionHandling, CallbackFunction};
use dom::bindings::cell::DOMRefCell;
use dom::bindings::codegen::Bindings::ErrorEventBinding::ErrorEventMethods;
use dom::bindings::codegen::Bindings::EventBinding::EventMethods;
use dom::bindings::codegen::Bindings::EventHandlerBinding::EventHandlerNonNull;
use dom::bindings::codegen::Bindings::EventHandlerBinding::OnErrorEventHandlerNonNull;
use dom::bindings::codegen::Bindings::EventListenerBinding::EventListener;
use dom::bindings::codegen::Bindings::EventTargetBinding::EventTargetMethods;
use dom::bindings::codegen::Bindings::WindowBinding::WindowMethods;
use dom::bindings::codegen::UnionTypes::EventOrString;
use dom::bindings::error::{Error, Fallible, report_pending_exception};
use dom::bindings::inheritance::{Castable, EventTargetTypeId, TopTypeId};
use dom::bindings::js::Root;
use dom::bindings::reflector::{Reflectable, Reflector};
use dom::bindings::typed::Typed;
use dom::element::Element;
use dom::errorevent::ErrorEvent;
use dom::event::{Event, EventBubbles, EventCancelable};
use dom::node::document_from_node;
use dom::virtualmethods::VirtualMethods;
use dom::window::Window;
use fnv::FnvHasher;
use heapsize::HeapSizeOf;
use js::jsapi::{CompileFunction, JS_GetFunctionObject, RootedValue, RootedFunction};
use js::jsapi::{JSAutoCompartment, JSAutoRequest};
use js::rust::{AutoObjectVectorWrapper, CompileOptionsWrapper};
use libc::{c_char, size_t};
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::default::Default;
use std::ffi::CString;
use std::hash::BuildHasherDefault;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::{intrinsics, ptr};
use string_cache::Atom;
use url::Url;
use util::str::DOMString;

#[derive(PartialEq, Clone, JSTraceable)]
pub enum CommonEventHandler {
    EventHandler(Rc<EventHandlerNonNull>),
    ErrorEventHandler(Rc<OnErrorEventHandlerNonNull>),
}

impl CommonEventHandler {
    fn parent(&self) -> &CallbackFunction {
        match *self {
            CommonEventHandler::EventHandler(ref handler) => &handler.parent,
            CommonEventHandler::ErrorEventHandler(ref handler) => &handler.parent,
        }
    }
}

#[derive(JSTraceable, Copy, Clone, PartialEq, HeapSizeOf)]
pub enum ListenerPhase {
    Capturing,
    Bubbling,
}

impl PartialEq for EventTargetTypeId {
    #[inline]
    fn eq(&self, other: &EventTargetTypeId) -> bool {
        match (*self, *other) {
            (EventTargetTypeId::Node(this_type), EventTargetTypeId::Node(other_type)) => {
                this_type == other_type
            }
            _ => self.eq_slow(other)
        }
    }
}

impl EventTargetTypeId {
    #[allow(unsafe_code)]
    fn eq_slow(&self, other: &EventTargetTypeId) -> bool {
        match (*self, *other) {
            (EventTargetTypeId::Node(this_type), EventTargetTypeId::Node(other_type)) => {
                this_type == other_type
            }
            (_, _) => {
                unsafe {
                    intrinsics::discriminant_value(self) == intrinsics::discriminant_value(other)
                }
            }
        }
    }
}

/// https://html.spec.whatwg.org/multipage/#internal-raw-uncompiled-handler
#[derive(JSTraceable, Clone, PartialEq)]
pub struct InternalRawUncompiledHandler {
    source: DOMString,
    url: Url,
    line: usize,
}

/// A representation of an event handler, either compiled or uncompiled raw source, or null.
#[derive(JSTraceable, PartialEq, Clone)]
pub enum InlineEventListener {
    Uncompiled(InternalRawUncompiledHandler),
    Compiled(CommonEventHandler),
    Null,
}

impl InlineEventListener {
    /// Get a compiled representation of this event handler, compiling it from its
    /// raw source if necessary.
    /// https://html.spec.whatwg.org/multipage/#getting-the-current-value-of-the-event-handler
    fn get_compiled_handler(&mut self, owner: &EventTarget, ty: &Atom)
                            -> Option<CommonEventHandler> {
        match mem::replace(self, InlineEventListener::Null) {
            InlineEventListener::Null => None,
            InlineEventListener::Uncompiled(handler) => {
                let result = owner.get_compiled_event_handler(handler, ty);
                if let Some(ref compiled) = result {
                    *self = InlineEventListener::Compiled(compiled.clone());
                }
                result
            }
            InlineEventListener::Compiled(handler) => {
                *self = InlineEventListener::Compiled(handler.clone());
                Some(handler)
            }
        }
    }
}

#[derive(JSTraceable, Clone, PartialEq)]
enum EventListenerType {
    Additive(Rc<EventListener>),
    Inline(InlineEventListener),
}

impl HeapSizeOf for EventListenerType {
    fn heap_size_of_children(&self) -> usize {
        // FIXME: Rc<T> isn't HeapSizeOf and we can't ignore it due to #6870 and #6871
        0
    }
}

impl EventListenerType {
    fn get_compiled_listener(&mut self, owner: &EventTarget, ty: &Atom)
                             -> Option<CompiledEventListener> {
        match self {
            &mut EventListenerType::Inline(ref mut inline) =>
                inline.get_compiled_handler(owner, ty)
                      .map(CompiledEventListener::Handler),
            &mut EventListenerType::Additive(ref listener) =>
                Some(CompiledEventListener::Listener(listener.clone())),
        }
    }
}

/// A representation of an EventListener/EventHandler object that has previously
/// been compiled successfully, if applicable.
pub enum CompiledEventListener {
    Listener(Rc<EventListener>),
    Handler(CommonEventHandler),
}

impl CompiledEventListener {
    // https://html.spec.whatwg.org/multipage/#the-event-handler-processing-algorithm
    pub fn call_or_handle_event<T: Reflectable>(&self,
                                                object: &T,
                                                event: &Event,
                                                exception_handle: ExceptionHandling) {
    }
}

#[derive(JSTraceable, Clone, PartialEq, HeapSizeOf)]
#[privatize]
/// A listener in a collection of event listeners.
struct EventListenerEntry {
    phase: ListenerPhase,
    listener: EventListenerType
}

#[derive(JSTraceable, HeapSizeOf)]
/// A mix of potentially uncompiled and compiled event listeners of the same type.
struct EventListeners(Vec<EventListenerEntry>);

impl Deref for EventListeners {
    type Target = Vec<EventListenerEntry>;
    fn deref(&self) -> &Vec<EventListenerEntry> {
        &self.0
    }
}

impl DerefMut for EventListeners {
    fn deref_mut(&mut self) -> &mut Vec<EventListenerEntry> {
        &mut self.0
    }
}

impl EventListeners {
    // https://html.spec.whatwg.org/multipage/#getting-the-current-value-of-the-event-handler
    fn get_inline_listener(&mut self, owner: &EventTarget, ty: &Atom) -> Option<CommonEventHandler> {
        for entry in &mut self.0 {
            if let EventListenerType::Inline(ref mut inline) = entry.listener {
                // Step 1.1-1.8 and Step 2
                return inline.get_compiled_handler(owner, ty);
            }
        }

        // Step 2
        None
    }

    // https://html.spec.whatwg.org/multipage/#getting-the-current-value-of-the-event-handler
    fn get_listeners(&mut self, phase: Option<ListenerPhase>, owner: &EventTarget, ty: &Atom)
                     -> Vec<CompiledEventListener> {
        self.0.iter_mut().filter_map(|entry| {
            if phase.is_none() || Some(entry.phase) == phase {
                // Step 1.1-1.8, 2
                entry.listener.get_compiled_listener(owner, ty)
            } else {
                None
            }
        }).collect()
    }
}

#[dom_struct]
pub struct EventTarget {
    reflector_: Reflector,
    #[ignore_heap_size_of = "type_ids are new"]
    type_id: EventTargetTypeId,
    handlers: DOMRefCell<HashMap<Atom, EventListeners, BuildHasherDefault<FnvHasher>>>,
}

impl EventTarget {
    pub fn new_inherited(type_id: EventTargetTypeId) -> EventTarget {
        EventTarget {
            type_id: type_id,
            reflector_: Reflector::new(),
            handlers: DOMRefCell::new(Default::default()),
        }
    }

    pub fn get_listeners_for(&self,
                             type_: &Atom,
                             specific_phase: Option<ListenerPhase>)
                             -> Vec<CompiledEventListener> {
        self.handlers.borrow_mut().get_mut(type_).map_or(vec![], |listeners| {
            listeners.get_listeners(specific_phase, self, type_)
        })
    }

    pub fn dispatch_event_with_target(&self,
                                      _target: &EventTarget,
                                      _event: &Event) -> bool {
        true
        //dispatch_event(self, Some(target), event)
    }

    pub fn dispatch_event(&self, _event: &Event) -> bool {
        true
        //dispatch_event(self, None, event)
    }

    /// https://html.spec.whatwg.org/multipage/#event-handler-attributes:event-handlers-11
    pub fn set_inline_event_listener(&self,
                                     ty: Atom,
                                     listener: Option<InlineEventListener>) {
        let mut handlers = self.handlers.borrow_mut();
        let entries = match handlers.entry(ty) {
            Occupied(entry) => entry.into_mut(),
            Vacant(entry) => entry.insert(EventListeners(vec!())),
        };

        let idx = entries.iter().position(|ref entry| {
            match entry.listener {
                EventListenerType::Inline(_) => true,
                _ => false,
            }
        });

        match idx {
            Some(idx) => {
                entries[idx].listener =
                    EventListenerType::Inline(listener.unwrap_or(InlineEventListener::Null));
            }
            None => {
                if let Some(listener) = listener {
                    entries.push(EventListenerEntry {
                        phase: ListenerPhase::Bubbling,
                        listener: EventListenerType::Inline(listener),
                    });
                }
            }
        }
    }

    fn get_inline_event_listener(&self, ty: &Atom) -> Option<CommonEventHandler> {
        let mut handlers = self.handlers.borrow_mut();
        handlers.get_mut(ty).and_then(|entry| entry.get_inline_listener(self, ty))
    }

    /// Store the raw uncompiled event handler for on-demand compilation later.
    /// https://html.spec.whatwg.org/multipage/#event-handler-attributes:event-handler-content-attributes-3
    pub fn set_event_handler_uncompiled(&self,
                                        url: Url,
                                        line: usize,
                                        ty: &str,
                                        source: DOMString) {
        let handler = InternalRawUncompiledHandler {
            source: source,
            line: line,
            url: url,
        };
        self.set_inline_event_listener(Atom::from(ty),
                                       Some(InlineEventListener::Uncompiled(handler)));
    }

    // https://html.spec.whatwg.org/multipage/#getting-the-current-value-of-the-event-handler
    #[allow(unsafe_code)]
    pub fn get_compiled_event_handler(&self,
                                      handler: InternalRawUncompiledHandler,
                                      ty: &Atom)
                                      -> Option<CommonEventHandler> {
        None
    }

    pub fn set_event_handler_common<T: CallbackContainer>(
        &self, ty: &str, listener: Option<Rc<T>>)
    {
        let event_listener = listener.map(|listener|
                                          InlineEventListener::Compiled(
                                              CommonEventHandler::EventHandler(
                                                  EventHandlerNonNull::new(listener.callback()))));
        self.set_inline_event_listener(Atom::from(ty), event_listener);
    }

    pub fn set_error_event_handler<T: CallbackContainer>(
        &self, ty: &str, listener: Option<Rc<T>>)
    {
        let event_listener = listener.map(|listener|
                                          InlineEventListener::Compiled(
                                              CommonEventHandler::ErrorEventHandler(
                                                  OnErrorEventHandlerNonNull::new(listener.callback()))));
        self.set_inline_event_listener(Atom::from(ty), event_listener);
    }

    pub fn get_event_handler_common<T: CallbackContainer>(&self, ty: &str) -> Option<Rc<T>> {
        let listener = self.get_inline_event_listener(&Atom::from(ty));
        listener.map(|listener| CallbackContainer::new(listener.parent().callback()))
    }

    pub fn has_handlers(&self) -> bool {
        !self.handlers.borrow().is_empty()
    }

    // https://html.spec.whatwg.org/multipage/#fire-a-simple-event
    pub fn fire_simple_event(&self, name: &str) -> Root<Event> {
        self.fire_event(name, EventBubbles::DoesNotBubble,
                        EventCancelable::NotCancelable)
    }

    // https://dom.spec.whatwg.org/#concept-event-fire
    pub fn fire_event(&self, name: &str,
                      bubbles: EventBubbles,
                      cancelable: EventCancelable)
                      -> Root<Event> {
        let event = Event::new(Atom::from(name), bubbles, cancelable);

        event.fire(self);

        event
    }
}

impl EventTargetMethods for EventTarget {
    // https://dom.spec.whatwg.org/#dom-eventtarget-addeventlistener
    fn AddEventListener(&self,
                        ty: DOMString,
                        listener: Option<Rc<EventListener>>,
                        capture: bool) {
        if let Some(listener) = listener {
            let mut handlers = self.handlers.borrow_mut();
            let entry = match handlers.entry(Atom::from(ty)) {
                Occupied(entry) => entry.into_mut(),
                Vacant(entry) => entry.insert(EventListeners(vec!())),
            };

            let phase = if capture { ListenerPhase::Capturing } else { ListenerPhase::Bubbling };
            let new_entry = EventListenerEntry {
                phase: phase,
                listener: EventListenerType::Additive(listener)
            };
            if !entry.contains(&new_entry) {
                entry.push(new_entry);
            }
        }
    }

    // https://dom.spec.whatwg.org/#dom-eventtarget-removeeventlistener
    fn RemoveEventListener(&self,
                           ty: DOMString,
                           listener: Option<Rc<EventListener>>,
                           capture: bool) {
        if let Some(ref listener) = listener {
            let mut handlers = self.handlers.borrow_mut();
            let entry = handlers.get_mut(&Atom::from(ty));
            for entry in entry {
                let phase = if capture { ListenerPhase::Capturing } else { ListenerPhase::Bubbling };
                let old_entry = EventListenerEntry {
                    phase: phase,
                    listener: EventListenerType::Additive(listener.clone())
                };
                if let Some(position) = entry.iter().position(|e| *e == old_entry) {
                    entry.remove(position);
                }
            }
        }
    }

    // https://dom.spec.whatwg.org/#dom-eventtarget-dispatchevent
    fn DispatchEvent(&self, event: &Event) -> Fallible<bool> {
        if event.dispatching() || !event.initialized() {
            return Err(Error::InvalidState);
        }
        event.set_trusted(false);
        Ok(self.dispatch_event(event))
    }
}

impl Typed for EventTarget {
    fn get_type(&self) -> TopTypeId { TopTypeId::EventTarget(self.type_id) }
    fn is_subtype(ty: &TopTypeId) -> bool {
        match ty {
            &TopTypeId::EventTarget(_) => true,
            _ => false
        }
    }
}

impl VirtualMethods for EventTarget {
    fn super_type(&self) -> Option<&VirtualMethods> {
        None
    }
}
