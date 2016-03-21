/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::DOMPointReadOnlyBinding::{DOMPointReadOnlyMethods, Wrap};
use dom::bindings::error::Fallible;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::Root;
use dom::bindings::reflector::{Reflector, reflect_dom_object};
use dom::bindings::typed::Typed;
use dom::bindings::inheritance::{TopTypeId,DOMPointReadOnlyTypeId};
use std::cell::Cell;

// http://dev.w3.org/fxtf/geometry/Overview.html#dompointreadonly
#[dom_struct]
pub struct DOMPointReadOnly {
    reflector_: Reflector,
    #[ignore_heap_size_of = "type_ids are new"]
    type_id: DOMPointReadOnlyTypeId,
    x: Cell<f64>,
    y: Cell<f64>,
    z: Cell<f64>,
    w: Cell<f64>,
}

impl DOMPointReadOnly {
    pub fn new_inherited(type_id: DOMPointReadOnlyTypeId, x: f64, y: f64, z: f64, w: f64) -> DOMPointReadOnly {
        DOMPointReadOnly {
            type_id: type_id,
            x: Cell::new(x),
            y: Cell::new(y),
            z: Cell::new(z),
            w: Cell::new(w),
            reflector_: Reflector::new(),
        }
    }

    pub fn new(global: GlobalRef, x: f64, y: f64, z: f64, w: f64) -> Root<DOMPointReadOnly> {
        reflect_dom_object(box DOMPointReadOnly::new_inherited(DOMPointReadOnlyTypeId::DOMPointReadOnly, x, y, z, w),
                           global,
                           Wrap)
    }

    pub fn Constructor(global: GlobalRef,
                       x: f64,
                       y: f64,
                       z: f64,
                       w: f64)
                       -> Fallible<Root<DOMPointReadOnly>> {
        Ok(DOMPointReadOnly::new(global, x, y, z, w))
    }
}

impl DOMPointReadOnlyMethods for DOMPointReadOnly {
    // https://dev.w3.org/fxtf/geometry/Overview.html#dom-dompointreadonly-x
    fn X(&self) -> f64 {
        self.x.get()
    }

    // https://dev.w3.org/fxtf/geometry/Overview.html#dom-dompointreadonly-y
    fn Y(&self) -> f64 {
        self.y.get()
    }

    // https://dev.w3.org/fxtf/geometry/Overview.html#dom-dompointreadonly-z
    fn Z(&self) -> f64 {
        self.z.get()
    }

    // https://dev.w3.org/fxtf/geometry/Overview.html#dom-dompointreadonly-w
    fn W(&self) -> f64 {
        self.w.get()
    }
}

impl Typed for DOMPointReadOnly {
    fn get_type(&self) -> TopTypeId {
        TopTypeId::DOMPointReadOnly(self.type_id)
    }

    fn is_subtype(ty : TopTypeId) -> bool {
        match ty {
            TopTypeId::DOMPointReadOnly(_) => true,
            _ => false
        }
    }
}

pub trait DOMPointWriteMethods {
    fn SetX(&self, value: f64);
    fn SetY(&self, value: f64);
    fn SetZ(&self, value: f64);
    fn SetW(&self, value: f64);
}

impl DOMPointWriteMethods for DOMPointReadOnly {
    fn SetX(&self, value: f64) {
        self.x.set(value);
    }

    fn SetY(&self, value: f64) {
        self.y.set(value);
    }

    fn SetZ(&self, value: f64) {
        self.z.set(value);
    }

    fn SetW(&self, value: f64) {
        self.w.set(value);
    }
}
