/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */


use dom::bindings::codegen::Bindings::HTMLDataListElementBinding::HTMLDataListElementMethods;
use dom::bindings::inheritance::{Castable,HTMLElementTypeId};
use dom::bindings::js::Root;
use dom::document::Document;
use dom::element::Element;
use dom::htmlcollection::{CollectionFilter, HTMLCollection};
use dom::htmlelement::HTMLElement;
use dom::htmloptionelement::HTMLOptionElement;
use dom::node::{Node};
use string_cache::Atom;
use util::str::DOMString;


pub struct HTMLDataListElement {
    htmlelement: HTMLElement
}

impl HTMLDataListElement {
    fn new_inherited(localName: Atom,
                     prefix: Option<DOMString>,
                     document: &Document) -> HTMLDataListElement {
        HTMLDataListElement {
            htmlelement:
                HTMLElement::new_inherited(HTMLElementTypeId::HTMLDataListElement, localName, prefix, document)
        }
    }

    
    pub fn new(localName: Atom,
               prefix: Option<DOMString>,
               document: &Document) -> Root<HTMLDataListElement> {
        let element = HTMLDataListElement::new_inherited(localName, prefix, document);
        Root::new_box(box element)
    }
    
    // https://html.spec.whatwg.org/multipage/#dom-datalist-options
    fn Options(&self) -> Root<HTMLCollection> {
        
        struct HTMLDataListOptionsFilter;
        impl CollectionFilter for HTMLDataListOptionsFilter {
            fn filter(&self, elem: &Element, _root: &Node) -> bool {
                elem.is::<HTMLOptionElement>()
            }
        }
        let filter = box HTMLDataListOptionsFilter;
        HTMLCollection::create(self.upcast(), filter)
    }
}
