/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::error::Fallible;
use dom::bindings::js::Root;
use dom::bindings::inheritance::DOMPointReadOnlyTypeId;
use dom::dompointreadonly::{DOMPointReadOnly, DOMPointWriteMethods};

// http://dev.w3.org/fxtf/geometry/Overview.html#dompoint

pub struct DOMPoint {
    point: DOMPointReadOnly,
}

impl DOMPoint {
    fn new_inherited(x: f64, y: f64, z: f64, w: f64) -> DOMPoint {
        DOMPoint {
            point: DOMPointReadOnly::new_inherited(DOMPointReadOnlyTypeId::DOMPoint, x, y, z, w),
        }
    }

    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Root<DOMPoint> {
        Root::new_box(box DOMPoint::new_inherited(x, y, z, w))
    }

    pub fn Constructor(x: f64,
                       y: f64,
                       z: f64,
                       w: f64)
                       -> Fallible<Root<DOMPoint>> {
        Ok(DOMPoint::new(x, y, z, w))
    }

    // https://dev.w3.org/fxtf/geometry/Overview.html#dom-dompointreadonly-x
    pub fn X(&self) -> f64 {
        self.point.X()
    }

    // https://dev.w3.org/fxtf/geometry/Overview.html#dom-dompointreadonly-x
    pub fn SetX(&self, value: f64) {
        self.point.SetX(value);
    }

    // https://dev.w3.org/fxtf/geometry/Overview.html#dom-dompointreadonly-y
    pub fn Y(&self) -> f64 {
        self.point.Y()
    }

    // https://dev.w3.org/fxtf/geometry/Overview.html#dom-dompointreadonly-y
    pub fn SetY(&self, value: f64) {
        self.point.SetY(value);
    }

    // https://dev.w3.org/fxtf/geometry/Overview.html#dom-dompointreadonly-z
    pub fn Z(&self) -> f64 {
        self.point.Z()
    }

    // https://dev.w3.org/fxtf/geometry/Overview.html#dom-dompointreadonly-z
    pub fn SetZ(&self, value: f64) {
        self.point.SetZ(value);
    }

    // https://dev.w3.org/fxtf/geometry/Overview.html#dom-dompointreadonly-w
    pub fn W(&self) -> f64 {
        self.point.W()
    }

    // https://dev.w3.org/fxtf/geometry/Overview.html#dom-dompointreadonly-w
    pub fn SetW(&self, value: f64) {
        self.point.SetW(value);
    }
}
