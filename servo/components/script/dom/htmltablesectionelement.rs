/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use cssparser::RGBA;
use dom::attr::AttrValue;
use dom::bindings::error::{ErrorResult, Fallible};
use dom::bindings::inheritance::{Castable, HTMLElementTypeId};
use dom::bindings::js::{LayoutJS, Root, RootedReference};
use dom::document::Document;
use dom::element::{Element, RawLayoutElementHelpers};
use dom::htmlcollection::{CollectionFilter, HTMLCollection};
use dom::htmlelement::HTMLElement;
use dom::htmltablerowelement::HTMLTableRowElement;
use dom::node::{Node};
use dom::virtualmethods::VirtualMethods;
use string_cache::Atom;
use util::str::DOMString;


pub struct HTMLTableSectionElement {
    htmlelement: HTMLElement,
}

impl HTMLTableSectionElement {
    fn new_inherited(id: u64, localName: Atom, prefix: Option<DOMString>, document: &Document)
                     -> HTMLTableSectionElement {
        HTMLTableSectionElement {
            htmlelement: HTMLElement::new_inherited(HTMLElementTypeId::HTMLTableSectionElement, id, localName, prefix, document),
        }
    }

    
    pub fn new(id: u64, localName: Atom, prefix: Option<DOMString>, document: &Document)
               -> Root<HTMLTableSectionElement> {
        let element = HTMLTableSectionElement::new_inherited(id, localName, prefix, document);
        Root::new_box(box element)
    }

    // https://html.spec.whatwg.org/multipage/#dom-tbody-rows
    fn Rows(&self) -> Root<HTMLCollection> {
        HTMLCollection::create(self.upcast(), box RowsFilter)
    }

    // https://html.spec.whatwg.org/multipage/#dom-tbody-deleterow
    fn DeleteRow(&self, index: i32) -> ErrorResult {
        let node = self.upcast::<Node>();
        node.delete_cell_or_row(
            index,
            || self.Rows(),
            |n| n.is::<HTMLTableRowElement>())
    }
}
struct RowsFilter;
impl CollectionFilter for RowsFilter {
    fn filter(&self, elem: &Element, root: &Node) -> bool {
        elem.is::<HTMLTableRowElement>()
            && elem.upcast::<Node>().GetParentNode().r() == Some(root)
    }
}

pub trait HTMLTableSectionElementLayoutHelpers {
    fn get_background_color(&self) -> Option<RGBA>;
}

#[allow(unsafe_code)]
impl HTMLTableSectionElementLayoutHelpers for LayoutJS<HTMLTableSectionElement> {
    fn get_background_color(&self) -> Option<RGBA> {
        unsafe {
            (&*self.upcast::<Element>().unsafe_get())
                .get_attr_for_layout(&ns!(), &atom!("bgcolor"))
                .and_then(AttrValue::as_color)
                .cloned()
        }
    }
}

impl VirtualMethods for HTMLTableSectionElement {
    fn super_type(&self) -> Option<&VirtualMethods> {
        Some(self.upcast::<HTMLElement>() as &VirtualMethods)
    }

    fn parse_plain_attribute(&self, local_name: &Atom, value: DOMString) -> AttrValue {
        match *local_name {
            atom!("bgcolor") => AttrValue::from_legacy_color(value),
            _ => self.super_type().unwrap().parse_plain_attribute(local_name, value),
        }
    }
}
