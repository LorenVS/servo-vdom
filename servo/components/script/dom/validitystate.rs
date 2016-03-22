/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::js::Root;
use dom::bindings::reflector::{Reflector};

// https://html.spec.whatwg.org/multipage/#validitystate
#[dom_struct]
pub struct ValidityState {
    reflector_: Reflector,
    state: u8,
}

impl ValidityState {
    fn new_inherited() -> ValidityState {
        ValidityState {
            reflector_: Reflector::new(),
            state: 0,
        }
    }

    pub fn new() -> Root<ValidityState> {
        Root::new_box(box ValidityState::new_inherited())
    }
}
