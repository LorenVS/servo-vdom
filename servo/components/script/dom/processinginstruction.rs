/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */


use dom::bindings::codegen::Bindings::ProcessingInstructionBinding::ProcessingInstructionMethods;
use dom::bindings::js::Root;
use dom::bindings::inheritance::CharacterDataTypeId;
use dom::characterdata::CharacterData;
use dom::document::Document;

use util::str::DOMString;

/// An HTML processing instruction node.

pub struct ProcessingInstruction {
    characterdata: CharacterData,
    target: DOMString,
}

impl ProcessingInstruction {
    fn new_inherited(id: u64, target: DOMString, data: DOMString, document: &Document) -> ProcessingInstruction {
        ProcessingInstruction {
            characterdata: CharacterData::new_inherited(CharacterDataTypeId::ProcessingInstruction, id, data, document),
            target: target
        }
    }

    pub fn new(id: u64, target: DOMString, data: DOMString, document: &Document) -> Root<ProcessingInstruction> {
        Root::new_box(box ProcessingInstruction::new_inherited(id, target, data, document))
    }

    pub fn target(&self) -> &DOMString {
        &self.target
    }

    // https://dom.spec.whatwg.org/#dom-processinginstruction-target
    pub fn Target(&self) -> DOMString {
        self.target.clone()
    }
}
