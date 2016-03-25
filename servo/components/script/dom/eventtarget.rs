/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::cell::DOMRefCell;
use dom::bindings::eventhandler::{EventHandlerNonNull, OnErrorEventHandlerNonNull};
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

pub struct EventTarget {
    #[ignore_heap_size_of = "type_ids are new"]
    type_id: EventTargetTypeId,
}

impl EventTarget {
    pub fn new_inherited(type_id: EventTargetTypeId) -> EventTarget {
        EventTarget {
            type_id: type_id
        }
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

    /// Store the raw uncompiled event handler for on-demand compilation later.
    /// https://html.spec.whatwg.org/multipage/#event-handler-attributes:event-handler-content-attributes-3
    pub fn set_event_handler_uncompiled(&self,
                                        _url: Url,
                                        _line: usize,
                                        _ty: &str,
                                        _source: DOMString) {
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
