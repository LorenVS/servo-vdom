/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::attr::Attr;
use dom::bindings::cell::DOMRefCell;
use dom::bindings::codegen::Bindings::HTMLObjectElementBinding::HTMLObjectElementMethods;
use dom::bindings::inheritance::{Castable, HTMLElementTypeId};
use dom::bindings::js::Root;
use dom::document::Document;
use dom::element::{AttributeMutation, Element};
use dom::htmlelement::HTMLElement;
use dom::htmlformelement::{FormControl, HTMLFormElement};
use dom::virtualmethods::VirtualMethods;
use net_traits::image::base::Image;
use std::sync::Arc;
use string_cache::Atom;
use util::str::DOMString;


pub struct HTMLObjectElement {
    htmlelement: HTMLElement,
    image: DOMRefCell<Option<Arc<Image>>>,
}

impl HTMLObjectElement {
    fn new_inherited(localName: Atom,
                     prefix: Option<DOMString>,
                     document: &Document) -> HTMLObjectElement {
        HTMLObjectElement {
            htmlelement:
                HTMLElement::new_inherited(HTMLElementTypeId::HTMLObjectElement, localName, prefix, document),
            image: DOMRefCell::new(None),
        }
    }

    
    pub fn new(localName: Atom,
               prefix: Option<DOMString>,
               document: &Document) -> Root<HTMLObjectElement> {
        let element = HTMLObjectElement::new_inherited(localName, prefix, document);
        Root::new_box(box element)
    }

    // https://html.spec.whatwg.org/multipage/#dom-object-type
    make_getter!(Type, "type");

    // https://html.spec.whatwg.org/multipage/#dom-object-type
    make_setter!(SetType, "type");

    // https://html.spec.whatwg.org/multipage/#dom-fae-form
    fn GetForm(&self) -> Option<Root<HTMLFormElement>> {
        self.form_owner()
    }
}

trait ProcessDataURL {
    fn process_data_url(&self);
}

impl<'a> ProcessDataURL for &'a HTMLObjectElement {
    // Makes the local `data` member match the status of the `data` attribute and starts
    /// prefetching the image. This method must be called after `data` is changed.
    fn process_data_url(&self) {
        let elem = self.upcast::<Element>();

        // TODO: support other values
        match (elem.get_attribute(&ns!(), &atom!("type")),
               elem.get_attribute(&ns!(), &atom!("data"))) {
            (None, Some(_uri)) => {
                // TODO(gw): Prefetch the image here.
            }
            _ => { }
        }
    }
}

pub fn is_image_data(uri: &str) -> bool {
    static TYPES: &'static [&'static str] = &["data:image/png", "data:image/gif", "data:image/jpeg"];
    TYPES.iter().any(|&type_| uri.starts_with(type_))
}


impl VirtualMethods for HTMLObjectElement {
    fn super_type(&self) -> Option<&VirtualMethods> {
        Some(self.upcast::<HTMLElement>() as &VirtualMethods)
    }

    fn attribute_mutated(&self, attr: &Attr, mutation: AttributeMutation) {
        self.super_type().unwrap().attribute_mutated(attr, mutation);
        match attr.local_name() {
            &atom!("data") => {
                if let AttributeMutation::Set(_) = mutation {
                    self.process_data_url();
                }
            },
            _ => {},
        }
    }
}

impl FormControl for HTMLObjectElement {}
