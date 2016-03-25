/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::inheritance::{Castable, HTMLElementTypeId};
use dom::bindings::js::{JS, MutNullableHeap, Root};
use dom::document::Document;
use dom::documentfragment::DocumentFragment;
use dom::htmlelement::HTMLElement;
use dom::node::{CloneChildrenFlag, Node, document_from_node};
use dom::virtualmethods::VirtualMethods;
use string_cache::Atom;
use util::str::DOMString;


pub struct HTMLTemplateElement {
    htmlelement: HTMLElement,

    /// https://html.spec.whatwg.org/multipage/#template-contents
    contents: MutNullableHeap<JS<DocumentFragment>>,
}

impl HTMLTemplateElement {
    fn new_inherited(id: u64,
                     localName: Atom,
                     prefix: Option<DOMString>,
                     document: &Document) -> HTMLTemplateElement {
        HTMLTemplateElement {
            htmlelement:
                HTMLElement::new_inherited(HTMLElementTypeId::HTMLTemplateElement, id, localName, prefix, document),
            contents: MutNullableHeap::new(None),
        }
    }

    
    pub fn new(id: u64,
               localName: Atom,
               prefix: Option<DOMString>,
               document: &Document) -> Root<HTMLTemplateElement> {
        let element = HTMLTemplateElement::new_inherited(id, localName, prefix, document);
        Root::new_box(box element)
    }
}

impl VirtualMethods for HTMLTemplateElement {
    fn super_type(&self) -> Option<&VirtualMethods> {
        Some(self.upcast::<HTMLElement>() as &VirtualMethods)
    }

    /// https://html.spec.whatwg.org/multipage/#template-adopting-steps
    fn adopting_steps(&self, old_doc: &Document) {
        self.super_type().unwrap().adopting_steps(old_doc);
    }
}
