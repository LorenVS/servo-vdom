/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! The `Reflector` struct.

use std::cell::UnsafeCell;
use std::ptr;

/// A struct to store a reference to the reflector of a DOM object.


#[servo_lang = "reflector"]

// If you're renaming or moving this field, update the path in plugins::reflector as well
pub struct Reflector {
}


impl PartialEq for Reflector {
    fn eq(&self, other: &Reflector) -> bool {
        true
    }
}

impl Reflector {

    /// Create an uninitialized `Reflector`.
    pub fn new() -> Reflector {
        Reflector {}
    }
}

/// A trait to provide access to the `Reflector` for a DOM object.
pub trait Reflectable {
    /// Returns the receiver's reflector.
    fn reflector(&self) -> &Reflector;
}
