/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::error::Fallible;
use dom::bindings::global::GlobalRef;
use dom::bindings::inheritance::{Castable,EventTypeId};
use dom::bindings::js::{Root};
use dom::event::Event;
use string_cache::Atom;
use util::str::DOMString;

// https://dom.spec.whatwg.org/#interface-customevent

pub struct CustomEvent {
    event: Event
}

impl CustomEvent {
    fn new_inherited() -> CustomEvent {
        CustomEvent {
            event: Event::new_inherited(EventTypeId::CustomEvent)
        }
    }

    pub fn new_uninitialized() -> Root<CustomEvent> {
        Root::new_box(box CustomEvent::new_inherited())
    }
    pub fn new(type_: Atom,
               bubbles: bool,
               cancelable: bool)
               -> Root<CustomEvent> {
        let ev = CustomEvent::new_uninitialized();
        ev.init_custom_event(type_, bubbles, cancelable);
        ev
    }

    fn init_custom_event(&self,
                         type_: Atom,
                         can_bubble: bool,
                         cancelable: bool) {
        let event = self.upcast::<Event>();
        if event.dispatching() {
            return;
        }

        event.init_event(type_, can_bubble, cancelable);
    }
    
    // https://dom.spec.whatwg.org/#dom-customevent-initcustomevent
    fn InitCustomEvent(&self,
                       type_: DOMString,
                       can_bubble: bool,
                       cancelable: bool) {
        self.init_custom_event(Atom::from(type_), can_bubble, cancelable)
    }

    // https://dom.spec.whatwg.org/#dom-event-istrusted
    fn IsTrusted(&self) -> bool {
        self.event.IsTrusted()
    }
}
