/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::error::Fallible;
use dom::bindings::js::{Root, JS};
use dom::dompoint::DOMPoint;
use dom::domrect::DOMRect;

// https://drafts.fxtf.org/geometry/#DOMQuad

pub struct DOMQuad {
    p1: JS<DOMPoint>,
    p2: JS<DOMPoint>,
    p3: JS<DOMPoint>,
    p4: JS<DOMPoint>,
}

impl DOMQuad {
    fn new_inherited(p1: &DOMPoint,
                     p2: &DOMPoint,
                     p3: &DOMPoint,
                     p4: &DOMPoint)
                     -> DOMQuad {

        DOMQuad {
            p1: JS::from_ref(p1),
            p2: JS::from_ref(p2),
            p3: JS::from_ref(p3),
            p4: JS::from_ref(p4),
        }
    }

    pub fn new(p1: &DOMPoint,
               p2: &DOMPoint,
               p3: &DOMPoint,
               p4: &DOMPoint) -> Root<DOMQuad> {
        Root::new_box(box DOMQuad::new_inherited(p1, p2, p3, p4))
    }

    // https://drafts.fxtf.org/geometry/#dom-domquad-p1
    fn P1(&self) -> Root<DOMPoint> {
        Root::from_ref(&self.p1)
    }

    // https://drafts.fxtf.org/geometry/#dom-domquad-p2
    fn P2(&self) -> Root<DOMPoint> {
        Root::from_ref(&self.p2)
    }

    // https://drafts.fxtf.org/geometry/#dom-domquad-p3
    fn P3(&self) -> Root<DOMPoint> {
        Root::from_ref(&self.p3)
    }

    // https://drafts.fxtf.org/geometry/#dom-domquad-p4
    fn P4(&self) -> Root<DOMPoint> {
        Root::from_ref(&self.p4)
    }

    // https://drafts.fxtf.org/geometry/#dom-domquad-getbounds
    fn GetBounds(&self) -> Root<DOMRect> {
        let left = self.p1.X().min(self.p2.X()).min(self.p3.X()).min(self.p4.X());
        let top = self.p1.Y().min(self.p2.Y()).min(self.p3.Y()).min(self.p4.Y());
        let right = self.p1.X().max(self.p2.X()).max(self.p3.X()).max(self.p4.X());
        let bottom = self.p1.Y().max(self.p2.Y()).max(self.p3.Y()).max(self.p4.Y());

        DOMRect::new(left,
                     top,
                     right - left,
                     bottom - top)
    }
}
