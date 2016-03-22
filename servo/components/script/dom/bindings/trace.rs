/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! Utilities for tracing JS-managed values.
//!
//! The lifetime of DOM objects is managed by the SpiderMonkey Garbage
//! Collector. A rooted DOM object implementing the interface `Foo` is traced
//! as follows:
//!
//! 1. The GC calls `_trace` defined in `FooBinding` during the marking
//!    phase. (This happens through `JSClass.trace` for non-proxy bindings, and
//!    through `ProxyTraps.trace` otherwise.)
//! 2. `_trace` calls `Foo::trace()` (an implementation of `JSTraceable`).
//!    This is typically derived via a ``
//!    (implies ``) annotation.
//!    Non-JS-managed types have an empty inline `trace()` method,
//!    achieved via `no_jsmanaged_fields!` or similar.
//! 3. For all fields, `Foo::trace()`
//!    calls `trace()` on the field.
//!    For example, for fields of type `JS<T>`, `JS<T>::trace()` calls
//!    `trace_reflector()`.
//! 4. `trace_reflector()` calls `JS_CallUnbarrieredObjectTracer()` with a
//!    pointer to the `JSObject` for the reflector. This notifies the GC, which
//!    will add the object to the graph, and will trace that object as well.
//! 5. When the GC finishes tracing, it [`finalizes`](../index.html#destruction)
//!    any reflectors that were not reachable.
//!
//! The `no_jsmanaged_fields!()` macro adds an empty implementation of `JSTraceable` to
//! a datatype.

use canvas_traits::WebGLError;
use canvas_traits::{CanvasGradientStop, LinearGradientStyle, RadialGradientStyle};
use canvas_traits::{CompositionOrBlending, LineCapStyle, LineJoinStyle, RepetitionStyle};
use cssparser::RGBA;
use devtools_traits::CSSError;
use devtools_traits::WorkerId;
use dom::bindings::inheritance::{DOMPointReadOnlyTypeId, DOMRectReadOnlyTypeId, EventTypeId,EventTargetTypeId, HTMLCollectionTypeId, NodeListTypeId};
use dom::bindings::js::{JS, Root};
use dom::bindings::refcounted::Trusted;
use encoding::types::EncodingRef;
use euclid::length::Length as EuclidLength;
use euclid::matrix2d::Matrix2D;
use euclid::rect::Rect;
use euclid::size::Size2D;
use html5ever::tree_builder::QuirksMode;
use hyper::header::Headers;
use hyper::method::Method;
use hyper::mime::Mime;
use ipc_channel::ipc::{IpcReceiver, IpcSender};
use layout_interface::{LayoutChan, LayoutRPC};
use libc;
use msg::constellation_msg::ConstellationChan;
use msg::constellation_msg::{PipelineId, SubpageId, WindowSizeData};
use net_traits::Metadata;
use net_traits::image::base::{Image, ImageMetadata};
use net_traits::image_cache_thread::{ImageCacheChan, ImageCacheThread};
use net_traits::response::HttpsState;
use net_traits::storage_thread::StorageType;
use profile_traits::mem::ProfilerChan as MemProfilerChan;
use profile_traits::time::ProfilerChan as TimeProfilerChan;
use script_thread::ScriptChan;
use script_traits::{LayoutMsg, ScriptMsg, TimerEventId, TimerSource, UntrustedNodeAddress};
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::boxed::FnBox;
use std::cell::{Cell, UnsafeCell};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::ffi::CString;
use std::hash::{BuildHasher, Hash};
use std::intrinsics::return_address;
use std::iter::{FromIterator, IntoIterator};
use std::mem;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize};
use std::sync::mpsc::{Receiver, Sender};
use string_cache::{Atom, Namespace, QualName};
use style::attr::{AttrIdentifier, AttrValue};
use style::element_state::*;
use style::properties::PropertyDeclarationBlock;
use style::restyle_hints::ElementSnapshot;
use style::selector_impl::PseudoElement;
use style::values::specified::Length;
use url::Url;
use util::str::{DOMString, LengthOrPercentageOrAuto};
use uuid::Uuid;



/// A vector of items that are rooted for the lifetime of this struct.

#[no_move]

pub struct RootedVec<T> {
    v: Vec<T>,
}


impl<T> RootedVec<T> {
    /// Create a vector of items of type T that is rooted for
    /// the lifetime of this struct
    pub fn new() -> RootedVec<T> {
        let addr = unsafe { return_address() as *const libc::c_void };

        unsafe { RootedVec::new_with_destination_address(addr) }
    }

    /// Create a vector of items of type T. This constructor is specific
    /// for RootTraceableSet.
    pub unsafe fn new_with_destination_address(addr: *const libc::c_void) -> RootedVec<T> {
        RootedVec::<T> {
            v: vec![],
        }
    }
}

impl<T> RootedVec<JS<T>> {
    /// Obtain a safe slice of references that can't outlive that RootedVec.
    pub fn r(&self) -> &[&T] {
        unsafe { mem::transmute(&self.v[..]) }
    }
}

impl<T> Deref for RootedVec<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Vec<T> {
        &self.v
    }
}

impl<T> DerefMut for RootedVec<T> {
    fn deref_mut(&mut self) -> &mut Vec<T> {
        &mut self.v
    }
}

impl<A> FromIterator<Root<A>> for RootedVec<JS<A>> {
    #[allow(moved_no_move)]
    fn from_iter<T>(iterable: T) -> RootedVec<JS<A>>
        where T: IntoIterator<Item = Root<A>>
    {
        let mut vec = unsafe {
            RootedVec::new_with_destination_address(return_address() as *const libc::c_void)
        };
        vec.extend(iterable.into_iter().map(|item| JS::from_rooted(&item)));
        vec
    }
}