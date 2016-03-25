/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::error::Fallible;
use dom::bindings::inheritance::{Castable, EventTypeId};
use dom::bindings::js::Root;
use dom::event::Event;
use dom::eventtarget::EventTarget;
use std::default::Default;
use string_cache::Atom;
use util::str::DOMString;


pub struct MessageEvent {
    event: Event,
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
        let ev = box MessageEvent {
            event: Event::new_inherited(EventTypeId::MessageEvent),
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
