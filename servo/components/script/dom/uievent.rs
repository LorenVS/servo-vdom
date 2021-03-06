/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::error::Fallible;
use dom::bindings::inheritance::{Castable, EventTypeId, UIEventTypeId};
use dom::bindings::js::Root;
use dom::bindings::js::{JS, MutNullableHeap, RootedReference};
use dom::event::{Event, EventBubbles, EventCancelable};
use dom::window::Window;
use std::cell::Cell;
use std::default::Default;
use string_cache::Atom;
use util::str::DOMString;

// https://dvcs.w3.org/hg/dom3events/raw-file/tip/html/DOM3-Events.html#interface-UIEvent

pub struct UIEvent {
    event: Event,
    view: MutNullableHeap<JS<Window>>,
    detail: Cell<i32>
}

impl UIEvent {
    pub fn new_inherited(type_id: UIEventTypeId) -> UIEvent {
        UIEvent {
            event: Event::new_inherited(EventTypeId::UIEvent(type_id)),
            view: Default::default(),
            detail: Cell::new(0),
        }
    }

    pub fn new_uninitialized() -> Root<UIEvent> {
        Root::new_box(box UIEvent::new_inherited(UIEventTypeId::UIEvent))
    }

    pub fn new(type_: DOMString,
               can_bubble: EventBubbles,
               cancelable: EventCancelable,
               view: Option<&Window>,
               detail: i32) -> Root<UIEvent> {
        let ev = UIEvent::new_uninitialized();
        ev.InitUIEvent(type_, can_bubble == EventBubbles::Bubbles,
                       cancelable == EventCancelable::Cancelable, view, detail);
        ev
    }

    // https://w3c.github.io/uievents/#widl-UIEvent-view
    fn GetView(&self) -> Option<Root<Window>> {
        self.view.get()
    }

    // https://w3c.github.io/uievents/#widl-UIEvent-detail
    fn Detail(&self) -> i32 {
        self.detail.get()
    }

    // https://w3c.github.io/uievents/#widl-UIEvent-initUIEvent
    pub fn InitUIEvent(&self,
                   type_: DOMString,
                   can_bubble: bool,
                   cancelable: bool,
                   view: Option<&Window>,
                   detail: i32) {
        let event = self.upcast::<Event>();
        if event.dispatching() {
            return;
        }

        event.init_event(Atom::from(type_), can_bubble, cancelable);
        self.view.set(view);
        self.detail.set(detail);
    }

    // https://dom.spec.whatwg.org/#dom-event-istrusted
    pub fn IsTrusted(&self) -> bool {
        self.event.IsTrusted()
    }
}
