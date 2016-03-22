/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! The `USVString` struct.

/// A string that is constructed from a UCS-2 buffer by replacing invalid code
/// points with the replacement character.
pub struct USVString(pub String);