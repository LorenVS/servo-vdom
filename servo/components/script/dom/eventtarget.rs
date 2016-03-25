/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::cell::DOMRefCell;
use dom::bindings::codegen::Bindings::EventHandlerBinding::EventHandlerNonNull;
use dom::bindings::codegen::Bindings::EventHandlerBinding::OnErrorEventHandlerNonNull;
use dom::bindings::codegen::Bindings::EventListenerBinding::EventListener;
use dom::bindings::error::{Error, Fallible};
use dom::bindings::inheritance::{EventTargetTypeId, TopTypeId};
use dom::bindings::js::Root;
use dom::bindings::typed::Typed;
use dom::event::{Event, EventBubbles, EventCancelable};
use dom::virtualmethods::VirtualMethods;
use fnv::FnvHasher;
use heapsize::HeapSizeOf;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::default::Default;
use std::hash::BuildHasherDefault;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::{intrinsics};
use string_cache::Atom;
use url::Url;
use util::str::DOMString;

#[derive(PartialEq, Clone)]
pub enum CommonEventHandler {
    EventHandler(Rc<EventHandlerNonNull>),
    ErrorEventHandler(Rc<OnErrorEventHandlerNonNull>),
}

#[derive(Copy, Clone, PartialEq)]
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
#[derive(Clone, PartialEq)]
pub struct InternalRawUncompiledHandler {
    source: DOMString,
    url: Url,
    line: usize,
}

/// A representation of an event handler, either compiled or uncompiled raw source, or null.
#[derive(PartialEq, Clone)]
pub enum InlineEventListener {
    Uncompiled(InternalRawUncompiledHandler),
    Compiled(CommonEventHandler),
    Null,
}

#[derive(Clone, PartialEq)]
enum EventListenerType {
    Additive(Rc<EventListener>)
}

impl HeapSizeOf for EventListenerType {
    fn heap_size_of_children(&self) -> usize {
        // FIXME: Rc<T> isn't HeapSizeOf and we can't ignore it due to #6870 and #6871
        0
    }
}

/// A representation of an EventListener/EventHandler object that has previously
/// been compiled successfully, if applicable.
pub enum CompiledEventListener {
    Listener(Rc<EventListener>),
    Handler(CommonEventHandler),
}

#[derive(Clone, PartialEq)]
#[privatize]
/// A listener in a collection of event listeners.
struct EventListenerEntry {
    phase: ListenerPhase,
    listener: EventListenerType
}


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



pub struct EventTarget {
    #[ignore_heap_size_of = "type_ids are new"]
    type_id: EventTargetTypeId,
    handlers: DOMRefCell<HashMap<Atom, EventListeners, BuildHasherDefault<FnvHasher>>>,
}

impl EventTarget {
    pub fn new_inherited(type_id: EventTargetTypeId) -> EventTarget {
        EventTarget {
            type_id: type_id,
            handlers: DOMRefCell::new(Default::default()),
        }
    }

    pub fn get_listeners_for(&self,
                             _type_: &Atom,
                             _specific_phase: Option<ListenerPhase>)
                             -> Vec<CompiledEventListener> {
        Vec::new()
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
                                     _ty: Atom,
                                     _listener: Option<InlineEventListener>) {
    }

    /// Store the raw uncompiled event handler for on-demand compilation later.
    /// https://html.spec.whatwg.org/multipage/#event-handler-attributes:event-handler-content-attributes-3
    pub fn set_event_handler_uncompiled(&self,
                                        _url: Url,
                                        _line: usize,
                                        _ty: &str,
                                        _source: DOMString) {
    }

    // https://html.spec.whatwg.org/multipage/#getting-the-current-value-of-the-event-handler
    #[allow(unsafe_code)]
    pub fn get_compiled_event_handler(&self,
                                      _handler: InternalRawUncompiledHandler,
                                      _ty: &Atom)
                                      -> Option<CommonEventHandler> {
        None
    }

    pub fn set_event_handler_common<T>(
        &self, _ty: &str, _listener: Option<Rc<T>>) {
    }

    pub fn set_error_event_handler<T>(
        &self, _ty: &str, _listener: Option<Rc<T>>) {
    }

    pub fn get_event_handler_common<T>(&self, _ty: &str) -> Option<Rc<T>> {
        None
    }

    pub fn has_handlers(&self) -> bool {
        false
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
