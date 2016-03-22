/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */


use dom::bindings::js::Root;
use dom::bindings::inheritance::HTMLTableCellElementTypeId;
use dom::document::Document;
use dom::htmltablecellelement::HTMLTableCellElement;

use string_cache::Atom;
use util::str::DOMString;


pub struct HTMLTableDataCellElement {
    htmltablecellelement: HTMLTableCellElement,
}

impl HTMLTableDataCellElement {
    fn new_inherited(localName: Atom,
                     prefix: Option<DOMString>,
                     document: &Document) -> HTMLTableDataCellElement {
        HTMLTableDataCellElement {
            htmltablecellelement:
                HTMLTableCellElement::new_inherited(HTMLTableCellElementTypeId::HTMLTableDataCellElement, localName, prefix, document)
        }
    }

    
    pub fn new(localName: Atom, prefix: Option<DOMString>, document: &Document)
               -> Root<HTMLTableDataCellElement> {
        Root::new_box(box HTMLTableDataCellElement::new_inherited(localName, prefix, document))
    }
}
