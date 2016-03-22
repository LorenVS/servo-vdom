/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

///! Miscellaneous Code which depends on large libraries that we don't
///  depend on in GeckoLib builds.

/// Behavior for stringification of `JSVal`s.
#[derive(PartialEq, Clone)]
pub enum StringificationBehavior {
    /// Convert `null` to the string `"null"`.
    Default,
    /// Convert `null` to the empty string.
    Empty,
}
