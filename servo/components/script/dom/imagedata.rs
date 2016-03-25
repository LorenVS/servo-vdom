/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::js::Root;
use euclid::size::Size2D;
use std::vec::Vec;

pub struct ImageData {
    width: u32,
    height: u32,
    data: Vec<u8>
}

impl ImageData {
    #[allow(unsafe_code)]
    pub fn new(width: u32, height: u32, data: Option<Vec<u8>>) -> Root<ImageData> {
        let imagedata = box ImageData {
            width: width,
            height: height,
            data: data.unwrap_or(Vec::new())
        };

        Root::new_box(imagedata)
    }

    #[allow(unsafe_code)]
    pub fn get_data_array(&self) -> Vec<u8> {
        self.data.clone()
    }

    pub fn get_size(&self) -> Size2D<i32> {
        Size2D::new(self.Width() as i32, self.Height() as i32)
    }

    // https://html.spec.whatwg.org/multipage/#dom-imagedata-width
    fn Width(&self) -> u32 {
        self.width
    }

    // https://html.spec.whatwg.org/multipage/#dom-imagedata-height
    fn Height(&self) -> u32 {
        self.height
    }
}
