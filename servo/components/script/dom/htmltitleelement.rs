/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::inheritance::{Castable, HTMLElementTypeId};
use dom::bindings::js::Root;
use dom::characterdata::CharacterData;
use dom::document::Document;
use dom::htmlelement::HTMLElement;
use dom::node::{ChildrenMutation, Node};
use dom::text::Text;
use dom::virtualmethods::VirtualMethods;
use string_cache::Atom;
use util::str::DOMString;

pub struct HTMLTitleElement {
    htmlelement: HTMLElement,
}

impl HTMLTitleElement {
    fn new_inherited(id: u64, localName: Atom, prefix: Option<DOMString>, document: &Document) -> HTMLTitleElement {
        HTMLTitleElement {
            htmlelement: HTMLElement::new_inherited(HTMLElementTypeId::HTMLTitleElement, id, localName, prefix, document)
        }
    }

    
    pub fn new(id: u64,
               localName: Atom,
               prefix: Option<DOMString>,
               document: &Document) -> Root<HTMLTitleElement> {
        let element = HTMLTitleElement::new_inherited(id, localName, prefix, document);
        Root::new_box(box element)
    }
    
    // https://html.spec.whatwg.org/multipage/#dom-title-text
    fn Text(&self) -> DOMString {
        let mut content = String::new();
        for child in self.upcast::<Node>().children() {
            if let Some(text) = child.downcast::<Text>() {
                content.push_str(&text.upcast::<CharacterData>().data());
            }
        }
        DOMString::from(content)
    }

}

impl VirtualMethods for HTMLTitleElement {
    fn super_type(&self) -> Option<&VirtualMethods> {
        Some(self.upcast::<HTMLElement>() as &VirtualMethods)
    }

    fn children_changed(&self, mutation: &ChildrenMutation) {
        if let Some(ref s) = self.super_type() {
            s.children_changed(mutation);
        }
        let node = self.upcast::<Node>();
        if node.is_in_doc() {
            node.owner_doc().title_changed();
        }
    }

    fn bind_to_tree(&self, is_in_doc: bool) {
        let node = self.upcast::<Node>();
        if is_in_doc {
            node.owner_doc().title_changed();
        }
    }
}
