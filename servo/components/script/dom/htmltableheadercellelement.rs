/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */


use dom::bindings::js::Root;
use dom::bindings::inheritance::HTMLTableCellElementTypeId;
use dom::document::Document;
use dom::htmltablecellelement::HTMLTableCellElement;

use string_cache::Atom;
use util::str::DOMString;


pub struct HTMLTableHeaderCellElement {
    htmltablecellelement: HTMLTableCellElement,
}

impl HTMLTableHeaderCellElement {
    fn new_inherited(id: u64,
                    localName: Atom,
                     prefix: Option<DOMString>,
                     document: &Document) -> HTMLTableHeaderCellElement {
        HTMLTableHeaderCellElement {
            htmltablecellelement:
                HTMLTableCellElement::new_inherited(HTMLTableCellElementTypeId::HTMLTableHeaderCellElement, id, localName, prefix, document)
        }
    }

    
    pub fn new(id: u64,
               localName: Atom,
               prefix: Option<DOMString>,
               document: &Document) -> Root<HTMLTableHeaderCellElement> {
        let element = HTMLTableHeaderCellElement::new_inherited(id, localName, prefix, document);
        Root::new_box(box element)
    }
}
