/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::cell::DOMRefCell;
use dom::bindings::eventhandler::EventHandlerNonNull;
use dom::bindings::error::{Error, Fallible};
use dom::bindings::global::GlobalRef;
use dom::bindings::inheritance::EventTargetTypeId;
use dom::bindings::js::Root;

use dom::eventtarget::EventTarget;
use std::cell::Cell;
use url::Url;
use util::str::DOMString;

#[derive(PartialEq, Copy, Clone, Debug)]
enum EventSourceReadyState {
    Connecting = 0,
    #[allow(dead_code)]
    Open = 1,
    Closed = 2
}


pub struct EventSource {
    eventtarget: EventTarget,
    url: Url,
    ready_state: Cell<EventSourceReadyState>,
    with_credentials: bool,
    last_event_id: DOMRefCell<DOMString>
}

impl EventSource {
    fn new_inherited(url: Url, with_credentials: bool) -> EventSource {
        EventSource {
            eventtarget: EventTarget::new_inherited(EventTargetTypeId::EventSource),
            url: url,
            ready_state: Cell::new(EventSourceReadyState::Connecting),
            with_credentials: with_credentials,
            last_event_id: DOMRefCell::new(DOMString::from(""))
        }
    }

    fn new(url: Url, with_credentials: bool) -> Root<EventSource> {
        Root::new_box(box EventSource::new_inherited(url, with_credentials))
    }
    
    // https://html.spec.whatwg.org/multipage/#handler-eventsource-onopen
    event_handler!(open, GetOnopen, SetOnopen);

    // https://html.spec.whatwg.org/multipage/#handler-eventsource-onmessage
    event_handler!(message, GetOnmessage, SetOnmessage);

    // https://html.spec.whatwg.org/multipage/#handler-eventsource-onerror
    event_handler!(error, GetOnerror, SetOnerror);

    // https://html.spec.whatwg.org/multipage/#dom-eventsource-url
    fn Url(&self) -> DOMString {
        DOMString::from(self.url.serialize())
    }

    // https://html.spec.whatwg.org/multipage/#dom-eventsource-withcredentials
    fn WithCredentials(&self) -> bool {
        self.with_credentials
    }

    // https://html.spec.whatwg.org/multipage/#dom-eventsource-readystate
    fn ReadyState(&self) -> u16 {
        self.ready_state.get() as u16
    }

    // https://html.spec.whatwg.org/multipage/#dom-eventsource-close
    fn Close(&self) {
        self.ready_state.set(EventSourceReadyState::Closed);
        // TODO: Terminate ongoing fetch
    }
}
