/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::attr::AttrValue;
use dom::bindings::inheritance::Castable;
use dom::bindings::js::Root;
use dom::bindings::inheritance::HTMLElementTypeId;
use dom::document::Document;
use dom::htmlelement::HTMLElement;
use dom::virtualmethods::VirtualMethods;
use string_cache::Atom;
use util::str::DOMString;


pub struct HTMLAppletElement {
    htmlelement: HTMLElement
}

impl HTMLAppletElement {
    fn new_inherited(id: u64,
                     localName: Atom,
                     prefix: Option<DOMString>,
                     document: &Document) -> HTMLAppletElement {
        HTMLAppletElement {
            htmlelement:
                HTMLElement::new_inherited(HTMLElementTypeId::HTMLAppletElement, id, localName, prefix, document)
        }
    }

    
    pub fn new(id: u64,
               localName: Atom,
               prefix: Option<DOMString>,
               document: &Document) -> Root<HTMLAppletElement> {
        let element = HTMLAppletElement::new_inherited(id, localName, prefix, document);
        Root::new_box(box element)
    }
    
    // https://html.spec.whatwg.org/multipage/#the-applet-element:dom-applet-name
    make_getter!(Name, "name");

    // https://html.spec.whatwg.org/multipage/#the-applet-element:dom-applet-name
    make_atomic_setter!(SetName, "name");
}

impl VirtualMethods for HTMLAppletElement {
    fn super_type(&self) -> Option<&VirtualMethods> {
        Some(self.upcast::<HTMLElement>() as &VirtualMethods)
    }

    fn parse_plain_attribute(&self, name: &Atom, value: DOMString) -> AttrValue {
        match name {
            &atom!("name") => AttrValue::from_atomic(value),
            _ => self.super_type().unwrap().parse_plain_attribute(name, value),
        }
    }
}
