/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::cell::DOMRefCell;
use dom::bindings::codegen::Bindings::FormDataBinding;
use dom::bindings::reflector::{Reflector, reflect_dom_object};
use dom::bindings::error::{Fallible};
use dom::bindings::global::GlobalRef;
use dom::bindings::js::{JS, Root};
use dom::htmlformelement::HTMLFormElement;
use std::collections::HashMap;
use string_cache::Atom;

#[derive(JSTraceable, Clone)]
#[must_root]
#[derive(HeapSizeOf)]
pub enum FormDatum {
    StringData(String),
    BlobData(String)
}

#[dom_struct]
pub struct FormData {
    reflector_: Reflector,
    data: DOMRefCell<HashMap<Atom, Vec<FormDatum>>>,
    form: Option<JS<HTMLFormElement>>
}

impl FormData {
    fn new_inherited(form: Option<&HTMLFormElement>) -> FormData {
        FormData {
            reflector_: Reflector::new(),
            data: DOMRefCell::new(HashMap::new()),
            form: form.map(|f| JS::from_ref(f)),
        }
    }

    pub fn new(form: Option<&HTMLFormElement>, global: GlobalRef) -> Root<FormData> {
        Root::new_box(box FormData::new_inherited(form))
    }

    pub fn Constructor(global: GlobalRef, form: Option<&HTMLFormElement>) -> Fallible<Root<FormData>> {
        // TODO: Construct form data set for form if it is supplied
        Ok(FormData::new(form, global))
    }
}