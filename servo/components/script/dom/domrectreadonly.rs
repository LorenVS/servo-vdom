/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::DOMRectReadOnlyBinding::{DOMRectReadOnlyMethods};
use dom::bindings::inheritance::{DOMRectReadOnlyTypeId,TopTypeId};
use dom::bindings::typed::Typed;
use dom::bindings::error::Fallible;
use dom::bindings::js::Root;
use dom::bindings::reflector::{Reflector};
use std::cell::Cell;


pub struct DOMRectReadOnly {
    reflector_: Reflector,
    #[ignore_heap_size_of = "type_ids are new"]
    type_id: DOMRectReadOnlyTypeId,
    x: Cell<f64>,
    y: Cell<f64>,
    width: Cell<f64>,
    height: Cell<f64>,
}

impl DOMRectReadOnly {
    pub fn new_inherited(type_id: DOMRectReadOnlyTypeId, x: f64, y: f64, width: f64, height: f64) -> DOMRectReadOnly {
        DOMRectReadOnly {
            type_id: type_id,
            x: Cell::new(x),
            y: Cell::new(y),
            width: Cell::new(width),
            height: Cell::new(height),
            reflector_: Reflector::new(),
        }
    }

    pub fn new(x: f64,
               y: f64,
               width: f64,
               height: f64)
               -> Root<DOMRectReadOnly> {
        Root::new_box(box DOMRectReadOnly::new_inherited(DOMRectReadOnlyTypeId::DOMRectReadOnly, x, y, width, height))
    }

    pub fn Constructor(x: f64,
                       y: f64,
                       width: f64,
                       height: f64)
                       -> Fallible<Root<DOMRectReadOnly>> {
        Ok(DOMRectReadOnly::new(x, y, width, height))
    }

    pub fn set_x(&self, value: f64) {
        self.x.set(value);
    }

    pub fn set_y(&self, value: f64) {
        self.y.set(value);
    }

    pub fn set_width(&self, value: f64) {
        self.width.set(value);
    }

    pub fn set_height(&self, value: f64) {
        self.height.set(value);
    }

    // https://drafts.fxtf.org/geometry/#dom-domrectreadonly-x
    pub fn X(&self) -> f64 {
        self.x.get()
    }

    // https://drafts.fxtf.org/geometry/#dom-domrectreadonly-y
    pub fn Y(&self) -> f64 {
        self.y.get()
    }

    // https://drafts.fxtf.org/geometry/#dom-domrectreadonly-width
    pub fn Width(&self) -> f64 {
        self.width.get()
    }

    // https://drafts.fxtf.org/geometry/#dom-domrectreadonly-height
    pub fn Height(&self) -> f64 {
        self.height.get()
    }

    // https://drafts.fxtf.org/geometry/#dom-domrectreadonly-top
    pub fn Top(&self) -> f64 {
        let height = self.height.get();
        if height >= 0f64 {
            self.y.get()
        } else {
            self.y.get() + height
        }
    }

    // https://drafts.fxtf.org/geometry/#dom-domrectreadonly-right
    pub fn Right(&self) -> f64 {
        let width = self.width.get();
        if width < 0f64 {
            self.x.get()
        } else {
            self.x.get() + width
        }
    }

    // https://drafts.fxtf.org/geometry/#dom-domrectreadonly-bottom
    pub fn Bottom(&self) -> f64 {
        let height = self.height.get();
        if height < 0f64 {
            self.y.get()
        } else {
            self.y.get() + height
        }
    }

    // https://drafts.fxtf.org/geometry/#dom-domrectreadonly-left
    pub fn Left(&self) -> f64 {
        let width = self.width.get();
        if width >= 0f64 {
            self.x.get()
        } else {
            self.x.get() + width
        }
    }
}

impl Typed for DOMRectReadOnly {
    fn get_type(&self) -> TopTypeId {
        TopTypeId::DOMRectReadOnly(self.type_id)
    }

    fn is_subtype(ty : &TopTypeId) -> bool{
        match ty {
            &TopTypeId::DOMRectReadOnly(_) => true,
            _ => false
        }
    }
}