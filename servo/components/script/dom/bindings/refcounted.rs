/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! A generic, safe mechanism by which DOM objects can be pinned and transferred
//! between threads (or intra-thread for asynchronous events). Akin to Gecko's
//! nsMainThreadPtrHandle, this uses thread-safe reference counting and ensures
//! that the actual SpiderMonkey GC integration occurs on the script thread via
//! message passing. Ownership of a `Trusted<T>` object means the DOM object of
//! type T to which it points remains alive. Any other behaviour is undefined.
//! To guarantee the lifetime of a DOM object when performing asynchronous operations,
//! obtain a `Trusted<T>` from that object and pass it along with each operation.
//! A usable pointer to the original DOM object can be obtained on the script thread
//! from a `Trusted<T>` via the `to_temporary` method.
//!
//! The implementation of Trusted<T> is as follows:
//! A hashtable resides in the script thread, keyed on the pointer to the Rust DOM object.
//! The values in this hashtable are atomic reference counts. When a Trusted<T> object is
//! created or cloned, this count is increased. When a Trusted<T> is dropped, the count
//! decreases. If the count hits zero, a message is dispatched to the script thread to remove
//! the entry from the hashmap if the count is still zero. The JS reflector for the DOM object
//! is rooted when a hashmap entry is first created, and unrooted when the hashmap entry
//! is removed.

use core::nonzero::NonZero;
use dom::bindings::js::Root;
use dom::bindings::reflector::{Reflectable, Reflector};
use dom::bindings::trace::trace_reflector;
use js::jsapi::JSTracer;
use libc;
use script_thread::{CommonScriptMsg, ScriptChan};
use std::cell::RefCell;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::hash_map::HashMap;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};


/// A pointer to a Rust DOM object that needs to be destroyed.
pub struct TrustedReference(*const libc::c_void);
unsafe impl Send for TrustedReference {}

/// A safe wrapper around a raw pointer to a DOM object that can be
/// shared among threads for use in asynchronous operations. The underlying
/// DOM object is guaranteed to live at least as long as the last outstanding
/// `Trusted<T>` instance.

pub struct Trusted<T> {
    /// A pointer to the Rust DOM object of type T, but void to allow
    /// sending `Trusted<T>` between threads, regardless of T's sendability.
    ptr: *const libc::c_void,
    script_chan: Box<ScriptChan + Send>,
    phantom: PhantomData<T>,
}

unsafe impl<T> Send for Trusted<T> {}

impl<T> Trusted<T> {
    /// Create a new `Trusted<T>` instance from an existing DOM pointer. The DOM object will
    /// be prevented from being GCed for the duration of the resulting `Trusted<T>` object's
    /// lifetime.
    pub fn new(ptr: &T, script_chan: Box<ScriptChan + Send>) -> Trusted<T> {
        Trusted {
            ptr: &*ptr as *const T as *const libc::c_void,
            script_chan: script_chan.clone(),
            phantom: PhantomData,
        }
    }

    /// Obtain a usable DOM pointer from a pinned `Trusted<T>` value. Fails if used on
    /// a different thread than the original value from which this `Trusted<T>` was
    /// obtained.
    pub fn root(&self) -> Root<T> {
        unsafe {
            Root::new(NonZero::new(self.ptr as *const T))
        }
    }
}

impl<T> Clone for Trusted<T> {
    fn clone(&self) -> Trusted<T> {
        Trusted {
            ptr: self.ptr,
            script_chan: self.script_chan.clone(),
            phantom: PhantomData,
        }
    }
}

impl<T> Drop for Trusted<T> {
    fn drop(&mut self) {
    }
}
