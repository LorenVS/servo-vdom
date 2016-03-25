/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::error::{Error, Fallible};
use dom::bindings::inheritance::{Castable, CharacterDataTypeId};
use dom::bindings::js::Root;
use dom::bindings::js::{RootedReference};
use dom::characterdata::CharacterData;
use dom::document::Document;
use dom::node::Node;
use util::str::DOMString;

/// An HTML text node.

pub struct Text {
    characterdata: CharacterData,
}

impl Text {
    fn new_inherited(id: u64, text: DOMString, document: &Document) -> Text {
        Text {
            characterdata: CharacterData::new_inherited(CharacterDataTypeId::Text, id, text, document)
        }
    }

    pub fn new(id: u64, text: DOMString, document: &Document) -> Root<Text> {
        Root::new_box(box Text::new_inherited(id, text, document))
    }

    // https://dom.spec.whatwg.org/#dom-text-wholetext
    fn WholeText(&self) -> DOMString {
        let first = self.upcast::<Node>().inclusively_preceding_siblings()
                                         .take_while(|node| node.is::<Text>())
                                         .last().unwrap();
        let nodes = first.inclusively_following_siblings()
                         .take_while(|node| node.is::<Text>());
        let mut text = String::new();
        for ref node in nodes {
            let cdata = node.downcast::<CharacterData>().unwrap();
            text.push_str(&cdata.data());
        }
        DOMString::from(text)
    }
}
