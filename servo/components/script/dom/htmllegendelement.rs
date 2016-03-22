/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */


use dom::bindings::js::Root;
use dom::bindings::inheritance::HTMLElementTypeId;
use dom::document::Document;
use dom::htmlelement::HTMLElement;

use string_cache::Atom;
use util::str::DOMString;


pub struct HTMLLegendElement {
    htmlelement: HTMLElement,
}

impl HTMLLegendElement {
    fn new_inherited(localName: Atom,
                     prefix: Option<DOMString>,
                     document: &Document) -> HTMLLegendElement {
        HTMLLegendElement {
            htmlelement:
                HTMLElement::new_inherited(HTMLElementTypeId::HTMLLegendElement, localName, prefix, document)
        }
    }

    
    pub fn new(localName: Atom,
               prefix: Option<DOMString>,
               document: &Document) -> Root<HTMLLegendElement> {
        let element = HTMLLegendElement::new_inherited(localName, prefix, document);
        Root::new_box(box element)
    }
}
