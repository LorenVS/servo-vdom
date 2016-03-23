/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */


use dom::bindings::js::Root;
use dom::bindings::inheritance::HTMLElementTypeId;
use dom::document::Document;
use dom::htmlelement::HTMLElement;

use string_cache::Atom;
use util::str::DOMString;


pub enum HeadingLevel {
    Heading1,
    Heading2,
    Heading3,
    Heading4,
    Heading5,
    Heading6,
}


pub struct HTMLHeadingElement {
    htmlelement: HTMLElement,
    level: HeadingLevel,
}

impl HTMLHeadingElement {
    fn new_inherited(id: u64,
                     localName: Atom,
                     prefix: Option<DOMString>,
                     document: &Document,
                     level: HeadingLevel) -> HTMLHeadingElement {
        HTMLHeadingElement {
            htmlelement:
                HTMLElement::new_inherited(HTMLElementTypeId::HTMLHeadingElement, id, localName, prefix, document),
            level: level,
        }
    }

    
    pub fn new(id: u64,
               localName: Atom,
               prefix: Option<DOMString>,
               document: &Document,
               level: HeadingLevel) -> Root<HTMLHeadingElement> {
        let element = HTMLHeadingElement::new_inherited(id, localName, prefix, document, level);
        Root::new_box(box element)
    }
}
