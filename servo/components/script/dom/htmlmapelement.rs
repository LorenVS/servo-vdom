/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */


use dom::bindings::js::Root;
use dom::bindings::inheritance::HTMLElementTypeId;
use dom::document::Document;
use dom::htmlelement::HTMLElement;

use string_cache::Atom;
use util::str::DOMString;

#[dom_struct]
pub struct HTMLMapElement {
    htmlelement: HTMLElement
}

impl HTMLMapElement {
    fn new_inherited(localName: Atom,
                     prefix: Option<DOMString>,
                     document: &Document) -> HTMLMapElement {
        HTMLMapElement {
            htmlelement: HTMLElement::new_inherited(HTMLElementTypeId::HTMLMapElement, localName, prefix, document)
        }
    }

    #[allow(unrooted_must_root)]
    pub fn new(localName: Atom,
               prefix: Option<DOMString>,
               document: &Document) -> Root<HTMLMapElement> {
        let element = HTMLMapElement::new_inherited(localName, prefix, document);
        Root::new_box(box element)
    }
}
