/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::error::Fallible;
use dom::bindings::global::GlobalRef;
use dom::bindings::inheritance::CharacterDataTypeId;
use dom::bindings::js::Root;
use dom::characterdata::CharacterData;
use dom::document::Document;
use util::str::DOMString;

/// An HTML comment.

pub struct Comment {
    characterdata: CharacterData,
}

impl Comment {
    fn new_inherited(id: u64, text: DOMString, document: &Document) -> Comment {
        Comment {
            characterdata: CharacterData::new_inherited(CharacterDataTypeId::Comment, id, text, document),
        }
    }

    pub fn new(id: u64, text: DOMString, document: &Document) -> Root<Comment> {
        Root::new_box(box Comment::new_inherited(id, text, document))
    }
}
