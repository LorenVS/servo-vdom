/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::EventBinding::EventMethods;
use dom::bindings::codegen::Bindings::MessageEventBinding;
use dom::bindings::codegen::Bindings::MessageEventBinding::MessageEventMethods;
use dom::bindings::error::Fallible;
use dom::bindings::global::GlobalRef;
use dom::bindings::inheritance::{Castable, EventTypeId};
use dom::bindings::js::Root;
use dom::bindings::reflector::reflect_dom_object;
use dom::event::Event;
use dom::eventtarget::EventTarget;
use js::jsapi::{RootedValue, HandleValue, Heap, JSContext};
use js::jsval::JSVal;
use std::default::Default;
use string_cache::Atom;
use util::str::DOMString;

#[dom_struct]
pub struct MessageEvent {
    event: Event,
    data: Heap<JSVal>,
    origin: DOMString,
    lastEventId: DOMString,
}

impl MessageEvent {
    pub fn new_uninitialized() -> Root<MessageEvent> {
        MessageEvent::new_initialized(DOMString::new(),
                                      DOMString::new())
    }

    pub fn new_initialized(origin: DOMString,
                           lastEventId: DOMString) -> Root<MessageEvent> {
        let mut ev = box MessageEvent {
            event: Event::new_inherited(EventTypeId::MessageEvent),
            data: Heap::default(),
            origin: origin,
            lastEventId: lastEventId,
        };
        Root::new_box(ev)
    }

    pub fn new(type_: Atom,
               bubbles: bool, cancelable: bool,
               origin: DOMString, lastEventId: DOMString)
               -> Root<MessageEvent> {
        let ev = MessageEvent::new_initialized(origin, lastEventId);
        {
            let event = ev.upcast::<Event>();
            event.init_event(type_, bubbles, cancelable);
        }
        ev
    }

    pub fn Constructor(_global: GlobalRef,
                       type_: DOMString,
                       init: &MessageEventBinding::MessageEventInit)
                       -> Fallible<Root<MessageEvent>> {
        // Dictionaries need to be rooted
        // https://github.com/servo/servo/issues/6381
        let ev = MessageEvent::new(Atom::from(type_), init.parent.bubbles, init.parent.cancelable,
                                   init.origin.clone(), init.lastEventId.clone());
        Ok(ev)
    }
}

impl MessageEvent {
    pub fn dispatch_jsval(target: &EventTarget,
                          _scope: GlobalRef,
                          _message: HandleValue) {
        let messageevent = MessageEvent::new(
            atom!("message"), false, false,
            DOMString::new(), DOMString::new());
        messageevent.upcast::<Event>().fire(target);
    }
}

impl MessageEventMethods for MessageEvent {
    // https://html.spec.whatwg.org/multipage/#dom-messageevent-data
    fn Data(&self, _cx: *mut JSContext) -> JSVal {
        self.data.get()
    }

    // https://html.spec.whatwg.org/multipage/#dom-messageevent-origin
    fn Origin(&self) -> DOMString {
        self.origin.clone()
    }

    // https://html.spec.whatwg.org/multipage/#dom-messageevent-lasteventid
    fn LastEventId(&self) -> DOMString {
        self.lastEventId.clone()
    }

    // https://dom.spec.whatwg.org/#dom-event-istrusted
    fn IsTrusted(&self) -> bool {
        self.event.IsTrusted()
    }
}
