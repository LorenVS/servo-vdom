/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */


use dom::bindings::js::Root;
use dom::bindings::inheritance::HTMLElementTypeId;
use dom::document::Document;
use dom::htmlelement::HTMLElement;

use string_cache::Atom;
use util::str::DOMString;


pub struct HTMLUListElement {
    htmlelement: HTMLElement
}

impl HTMLUListElement {
    fn new_inherited(id: u64, localName: Atom, prefix: Option<DOMString>, document: &Document) -> HTMLUListElement {
        HTMLUListElement {
            htmlelement: HTMLElement::new_inherited(HTMLElementTypeId::HTMLUListElement, id, localName, prefix, document)
        }
    }

    
    pub fn new(id: u64,
               localName: Atom,
               prefix: Option<DOMString>,
               document: &Document) -> Root<HTMLUListElement> {
        let element = HTMLUListElement::new_inherited(id, localName, prefix, document);
        Root::new_box(box element)
    }
}
