/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! IDLInterface and DerivedFrom

use dom::bindings::inheritance::Castable;

/// A trait to check whether a given `JSObject` implements an IDL interface.
pub trait IDLInterface {}

/// A trait to mark an IDL interface as deriving from another one.
#[rustc_on_unimplemented = "The IDL interface `{Self}` is not derived from `{T}`."]
pub trait DerivedFrom<T: Castable>: Castable {}
