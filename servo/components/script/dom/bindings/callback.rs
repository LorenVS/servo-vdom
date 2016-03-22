/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! Base classes to work with IDL callbacks.

/// A common base class for representing IDL callback function types.
#[derive(JSTraceable, PartialEq)]
pub struct CallbackFunction {
}

impl CallbackFunction {
    /// Create a new `CallbackFunction` for this object.
    pub fn new() -> CallbackFunction {
        CallbackFunction {}
    }
}

/// A common base class for representing IDL callback interface types.
#[derive(JSTraceable, PartialEq)]
pub struct CallbackInterface {
}


impl CallbackInterface {
    /// Create a new CallbackInterface object for the given `JSObject`.
    pub fn new() -> CallbackInterface {
        CallbackInterface {}
    }
}

