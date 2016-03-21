/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */


use dom::bindings::codegen::Bindings::WindowBinding::WindowMethods;
use dom::bindings::error::Fallible;
use dom::bindings::global::GlobalRef;
use dom::bindings::inheritance::CharacterDataTypeId;
use dom::bindings::js::Root;
use dom::characterdata::CharacterData;
use dom::document::Document;
use dom::node::Node;
use util::str::DOMString;

/// An HTML comment.
#[dom_struct]
pub struct Comment {
    characterdata: CharacterData,
}

impl Comment {
    fn new_inherited(text: DOMString, document: &Document) -> Comment {
        Comment {
            characterdata: CharacterData::new_inherited(CharacterDataTypeId::Comment, text, document),
        }
    }

    pub fn new(text: DOMString, document: &Document) -> Root<Comment> {
        Root::new_box(box Comment::new_inherited(text, document))
    }

    pub fn Constructor(global: GlobalRef, data: DOMString) -> Fallible<Root<Comment>> {
        let document = global.as_window().Document();
        Ok(Comment::new(data, document.r()))
    }
}
