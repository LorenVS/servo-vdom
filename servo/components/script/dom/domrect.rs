/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::error::Fallible;
use dom::bindings::inheritance::DOMRectReadOnlyTypeId;
use dom::bindings::js::Root;
use dom::domrectreadonly::DOMRectReadOnly;

pub struct DOMRect {
    rect: DOMRectReadOnly,
}

impl DOMRect {
    fn new_inherited(x: f64, y: f64, width: f64, height: f64) -> DOMRect {
        DOMRect {
            rect: DOMRectReadOnly::new_inherited(DOMRectReadOnlyTypeId::DOMRect, x, y, width, height),
        }
    }

    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Root<DOMRect> {
        Root::new_box(box DOMRect::new_inherited(x, y, width, height))
    }

    pub fn Constructor(x: f64,
                       y: f64,
                       width: f64,
                       height: f64)
                       -> Fallible<Root<DOMRect>> {
        Ok(DOMRect::new(x, y, width, height))
    }

    // https://drafts.fxtf.org/geometry/#dom-domrect-x
    pub fn X(&self) -> f64 {
        self.rect.X()
    }

    // https://drafts.fxtf.org/geometry/#dom-domrect-x
    pub fn SetX(&self, value: f64) {
        self.rect.set_x(value);
    }

    // https://drafts.fxtf.org/geometry/#dom-domrect-y
    pub fn Y(&self) -> f64 {
        self.rect.Y()
    }

    // https://drafts.fxtf.org/geometry/#dom-domrect-y
    pub fn SetY(&self, value: f64) {
        self.rect.set_y(value);
    }

    // https://drafts.fxtf.org/geometry/#dom-domrect-width
    pub fn Width(&self) -> f64 {
        self.rect.Width()
    }

    // https://drafts.fxtf.org/geometry/#dom-domrect-width
    pub fn SetWidth(&self, value: f64) {
        self.rect.set_width(value);
    }

    // https://drafts.fxtf.org/geometry/#dom-domrect-height
    pub fn Height(&self) -> f64 {
        self.rect.Height()
    }

    // https://drafts.fxtf.org/geometry/#dom-domrect-height
    pub fn SetHeight(&self, value: f64) {
        self.rect.set_height(value);
    }
}
