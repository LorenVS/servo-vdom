/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! Union types previously from IDL codegen

use dom::bindings::js::Root;
use dom::bindings::str::USVString;
use dom::types::*;
use util::str::DOMString;

pub enum EventOrString {
    Event(Root<Event>),
    String(DOMString),
}


pub enum HTMLElementOrLong {
    HTMLElement(Root<HTMLElement>),
    Long(i32),
}


pub enum HTMLOptionElementOrHTMLOptGroupElement {
    HTMLOptionElement(Root<HTMLOptionElement>),
    HTMLOptGroupElement(Root<HTMLOptGroupElement>),
}


pub enum NodeOrString {
    Node(Root<Node>),
    String(DOMString),
}
