/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */


use dom::bindings::js::Root;
use dom::bindings::inheritance::HTMLElementTypeId;
use dom::document::Document;
use dom::htmlelement::HTMLElement;

use string_cache::Atom;
use util::str::DOMString;


pub struct HTMLLIElement {
    htmlelement: HTMLElement,
}

impl HTMLLIElement {
    fn new_inherited(localName: Atom, prefix: Option<DOMString>, document: &Document) -> HTMLLIElement {
        HTMLLIElement {
            htmlelement: HTMLElement::new_inherited(HTMLElementTypeId::HTMLLIElement, localName, prefix, document)
        }
    }

    
    pub fn new(localName: Atom,
               prefix: Option<DOMString>,
               document: &Document) -> Root<HTMLLIElement> {
        let element = HTMLLIElement::new_inherited(localName, prefix, document);
        Root::new_box(box element)
    }
}
