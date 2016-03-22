/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::HTMLMeterElementBinding::{self, HTMLMeterElementMethods};
use dom::bindings::inheritance::{Castable, HTMLElementTypeId};
use dom::bindings::js::Root;
use dom::document::Document;
use dom::htmlelement::HTMLElement;

use dom::nodelist::NodeList;
use string_cache::Atom;
use util::str::DOMString;

#[dom_struct]
pub struct HTMLMeterElement {
    htmlelement: HTMLElement
}

impl HTMLMeterElement {
    fn new_inherited(localName: Atom,
                     prefix: Option<DOMString>,
                     document: &Document) -> HTMLMeterElement {
        HTMLMeterElement {
            htmlelement: HTMLElement::new_inherited(HTMLElementTypeId::HTMLMeterElement, localName, prefix, document)
        }
    }

    #[allow(unrooted_must_root)]
    pub fn new(localName: Atom,
               prefix: Option<DOMString>,
               document: &Document) -> Root<HTMLMeterElement> {
        let element = HTMLMeterElement::new_inherited(localName, prefix, document);
        Root::new_box(box element)
    }
}

impl HTMLMeterElementMethods for HTMLMeterElement {
    // https://html.spec.whatwg.org/multipage/#dom-lfe-labels
    fn Labels(&self) -> Root<NodeList> {
        self.upcast::<HTMLElement>().labels()
    }
}
