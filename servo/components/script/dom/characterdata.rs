/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! DOM bindings for `CharacterData`.

use dom::bindings::cell::DOMRefCell;
use dom::bindings::uniontypes::NodeOrString;
use dom::bindings::error::{Error, ErrorResult, Fallible};
use dom::bindings::inheritance::{Castable, CharacterDataTypeId, NodeTypeId};
use dom::bindings::js::{LayoutJS, Root};
use dom::comment::Comment;
use dom::document::Document;
use dom::element::Element;
use dom::node::{Node, NodeDamage};
use dom::processinginstruction::ProcessingInstruction;
use dom::text::Text;
use std::cell::Ref;
use util::str::DOMString;

// https://dom.spec.whatwg.org/#characterdata

pub struct CharacterData {
    node: Node,
    data: DOMRefCell<DOMString>,
}

impl CharacterData {
    pub fn new_inherited(type_id: CharacterDataTypeId, id: u64, data: DOMString, document: &Document) -> CharacterData {
        CharacterData {
            node: Node::new_inherited(NodeTypeId::CharacterData(type_id), id, document),
            data: DOMRefCell::new(data),
        }
    }

    #[inline]
    pub fn data(&self) -> Ref<DOMString> {
        self.data.borrow()
    }

    #[inline]
    pub fn append_data(&self, data: &str) {
        self.data.borrow_mut().push_str(data);
        self.content_changed();
    }

    fn content_changed(&self) {
        let node = self.upcast::<Node>();
        node.owner_doc().content_changed(node, NodeDamage::OtherNodeDamage);
    }

    // https://dom.spec.whatwg.org/#dom-characterdata-data
    pub fn Data(&self) -> DOMString {
        self.data.borrow().clone()
    }

    // https://dom.spec.whatwg.org/#dom-characterdata-length
    pub fn Length(&self) -> u32 {
        self.data.borrow().encode_utf16().count() as u32
    }

    // https://dom.spec.whatwg.org/#dom-characterdata-substringdata
    pub fn SubstringData(&self, offset: u32, count: u32) -> Fallible<DOMString> {
        let data = self.data.borrow();
        // Step 1.
        let data_from_offset = match find_utf16_code_unit_offset(&data, offset) {
            Some(offset_bytes) => &data[offset_bytes..],
            // Step 2.
            None => return Err(Error::IndexSize),
        };
        let substring = match find_utf16_code_unit_offset(data_from_offset, count) {
            // Steps 3.
            None => data_from_offset,
            // Steps 4.
            Some(count_bytes) => &data_from_offset[..count_bytes],
        };
        Ok(DOMString::from(substring))
    }

    // https://dom.spec.whatwg.org/#dom-characterdata-appenddatadata
    pub fn AppendData(&self, data: DOMString) {
        // FIXME(ajeffrey): Efficient append on DOMStrings?
        self.append_data(&*data);
    }

    // https://dom.spec.whatwg.org/#dom-nondocumenttypechildnode-previouselementsibling
    fn GetPreviousElementSibling(&self) -> Option<Root<Element>> {
        self.upcast::<Node>().preceding_siblings().filter_map(Root::downcast).next()
    }

    // https://dom.spec.whatwg.org/#dom-nondocumenttypechildnode-nextelementsibling
    fn GetNextElementSibling(&self) -> Option<Root<Element>> {
        self.upcast::<Node>().following_siblings().filter_map(Root::downcast).next()
    }
}

#[allow(unsafe_code)]
pub trait LayoutCharacterDataHelpers {
    unsafe fn data_for_layout(&self) -> &str;
}

#[allow(unsafe_code)]
impl LayoutCharacterDataHelpers for LayoutJS<CharacterData> {
    #[inline]
    unsafe fn data_for_layout(&self) -> &str {
        &(*self.unsafe_get()).data.borrow_for_layout()
    }
}

/// Given a number of UTF-16 code units from the start of the given string,
/// return the corresponding number of UTF-8 bytes.
///
/// s[find_utf16_code_unit_offset(s, o).unwrap()..] == s.to_utf16()[o..].to_utf8()
fn find_utf16_code_unit_offset(s: &str, offset: u32) -> Option<usize> {
    let mut code_units = 0;
    for (i, c) in s.char_indices() {
        if code_units == offset {
            return Some(i);
        }
        code_units += 1;
        if c > '\u{FFFF}' {
            if code_units == offset {
                panic!("\n\n\
                    Would split a surrogate pair in CharacterData API.\n\
                    If you see this in real content, please comment with the URL\n\
                    on https://github.com/servo/servo/issues/6873\n\
                \n");
            }
            code_units += 1;
        }
    }
    if code_units == offset {
        Some(s.len())
    } else {
        None
    }
}
