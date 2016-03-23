/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! Smart pointers for the JS-managed DOM objects.
//!
//! The DOM is made up of DOM objects whose lifetime is entirely controlled by
//! the whims of the SpiderMonkey garbage collector. The types in this module
//! are designed to ensure that any interactions with said Rust types only
//! occur on values that will remain alive the entire time.
//!
//! Here is a brief overview of the important types:
//!
//! - `Root<T>`: a stack-based reference to a rooted DOM object.
//! - `JS<T>`: a reference to a DOM object that can automatically be traced by
//!   the GC when encountered as a field of a Rust structure.
//!
//! `JS<T>` does not allow access to their inner value without explicitly
//! creating a stack-based root via the `root` method. This returns a `Root<T>`,
//! which causes the JS-owned value to be uncollectable for the duration of the
//! `Root` object's lifetime. A reference to the object can then be obtained
//! from the `Root` object. These references are not allowed to outlive their
//! originating `Root<T>`.
//!

use core::nonzero::NonZero;
use dom::bindings::conversions::DerivedFrom;
use dom::bindings::inheritance::Castable;
use dom::node::Node;
use heapsize::HeapSizeOf;
use layout_interface::TrustedNodeAddress;
use std::cell::UnsafeCell;
use std::default::Default;
use std::hash::{Hash, Hasher};
use std::mem;
use std::ops::Deref;
use std::ptr;
use util::thread_state;

/// A traced reference to a DOM object
///
/// This type is critical to making garbage collection work with the DOM,
/// but it is very dangerous; if garbage collection happens with a `JS<T>`
/// on the stack, the `JS<T>` can point to freed memory.
///
/// This should only be used as a field in other DOM objects.
pub struct JS<T> {
    ptr: NonZero<*const T>,
}

// JS<T> is similar to Rc<T>, in that it's not always clear how to avoid double-counting.
// For now, we choose not to follow any such pointers.
impl<T> HeapSizeOf for JS<T> {
    fn heap_size_of_children(&self) -> usize {
        0
    }
}

impl<T> JS<T> {
    /// Returns `LayoutJS<T>` containing the same pointer.
    pub unsafe fn to_layout(&self) -> LayoutJS<T> {
        debug_assert!(thread_state::get().is_layout());
        LayoutJS {
            ptr: self.ptr.clone(),
        }
    }
}

impl<T> JS<T> {
    /// Create a JS<T> from a Root<T>
    /// XXX Not a great API. Should be a call on Root<T> instead
    
    pub fn from_rooted(root: &Root<T>) -> JS<T> {
        debug_assert!(thread_state::get().is_script());
        JS {
            ptr: unsafe { NonZero::new(&**root) },
        }
    }
    /// Create a JS<T> from a &T
    
    pub fn from_ref(obj: &T) -> JS<T> {
        debug_assert!(thread_state::get().is_script());
        JS {
            ptr: unsafe { NonZero::new(&*obj) },
        }
    }
}

impl<T> Deref for JS<T> {
    type Target = T;

    fn deref(&self) -> &T {
        debug_assert!(thread_state::get().is_script());
        // We can only have &JS<T> from a rooted thing, so it's safe to deref
        // it to &T.
        unsafe { &**self.ptr }
    }
}

/// An unrooted reference to a DOM object for use in layout. `Layout*Helpers`
/// traits must be implemented on this.

pub struct LayoutJS<T> {
    ptr: NonZero<*const T>,
}

impl<T: Castable> LayoutJS<T> {
    /// Cast a DOM object root upwards to one of the interfaces it derives from.
    pub fn upcast<U>(&self) -> LayoutJS<U>
        where U: Castable,
              T: DerivedFrom<U>
    {
        debug_assert!(thread_state::get().is_layout());
        unsafe { mem::transmute_copy(self) }
    }

    /// Cast a DOM object downwards to one of the interfaces it might implement.
    pub fn downcast<U>(&self) -> Option<LayoutJS<U>>
        where U: DerivedFrom<T>
    {
        debug_assert!(thread_state::get().is_layout());
        unsafe {
            if (*self.unsafe_get()).is::<U>() {
                Some(mem::transmute_copy(self))
            } else {
                None
            }
        }
    }
}

impl<T> Copy for LayoutJS<T> {}

impl<T> PartialEq for JS<T> {
    fn eq(&self, other: &JS<T>) -> bool {
        self.ptr == other.ptr
    }
}

impl<T> Eq for JS<T> {}

impl<T> PartialEq for LayoutJS<T> {
    fn eq(&self, other: &LayoutJS<T>) -> bool {
        self.ptr == other.ptr
    }
}

impl<T> Eq for LayoutJS<T> {}

impl<T> Hash for JS<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ptr.hash(state)
    }
}

impl<T> Hash for LayoutJS<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ptr.hash(state)
    }
}

impl <T> Clone for JS<T> {
    #[inline]
    
    fn clone(&self) -> JS<T> {
        debug_assert!(thread_state::get().is_script());
        JS {
            ptr: self.ptr.clone(),
        }
    }
}

impl <T> Clone for LayoutJS<T> {
    #[inline]
    fn clone(&self) -> LayoutJS<T> {
        debug_assert!(thread_state::get().is_layout());
        LayoutJS {
            ptr: self.ptr.clone(),
        }
    }
}

impl LayoutJS<Node> {
    /// Create a new JS-owned value wrapped from an address known to be a
    /// `Node` pointer.
    pub unsafe fn from_trusted_node_address(inner: TrustedNodeAddress) -> LayoutJS<Node> {
        debug_assert!(thread_state::get().is_layout());
        let TrustedNodeAddress(addr) = inner;
        LayoutJS {
            ptr: NonZero::new(addr as *const Node),
        }
    }
}

/// A holder that provides interior mutability for GC-managed values such as
/// `JS<T>`.  Essentially a `Cell<JS<T>>`, but safer.
///
/// This should only be used as a field in other DOM objects; see warning
/// on `JS<T>`.

pub struct MutHeap<T> {
    val: UnsafeCell<T>,
}

impl<T> MutHeap<JS<T>> {
    /// Create a new `MutHeap`.
    pub fn new(initial: &T) -> MutHeap<JS<T>> {
        debug_assert!(thread_state::get().is_script());
        MutHeap {
            val: UnsafeCell::new(JS::from_ref(initial)),
        }
    }

    /// Set this `MutHeap` to the given value.
    pub fn set(&self, val: &T) {
        debug_assert!(thread_state::get().is_script());
        unsafe {
            *self.val.get() = JS::from_ref(val);
        }
    }

    /// Get the value in this `MutHeap`.
    pub fn get(&self) -> Root<T> {
        debug_assert!(thread_state::get().is_script());
        unsafe {
            Root::from_ref(&*ptr::read(self.val.get()))
        }
    }
}

impl<T> HeapSizeOf for MutHeap<T> {
    fn heap_size_of_children(&self) -> usize {
        // See comment on HeapSizeOf for JS<T>.
        0
    }
}

impl<T> PartialEq for MutHeap<JS<T>> {
   fn eq(&self, other: &Self) -> bool {
        unsafe {
            *self.val.get() == *other.val.get()
        }
    }
}

impl<T: PartialEq> PartialEq<T> for MutHeap<JS<T>> {
    fn eq(&self, other: &T) -> bool {
        unsafe {
            **self.val.get() == *other
        }
    }
}

/// A holder that provides interior mutability for GC-managed values such as
/// `JS<T>`, with nullability represented by an enclosing Option wrapper.
/// Essentially a `Cell<Option<JS<T>>>`, but safer.
///
/// This should only be used as a field in other DOM objects; see warning
/// on `JS<T>`.

pub struct MutNullableHeap<T> {
    ptr: UnsafeCell<Option<T>>,
}

impl<T> MutNullableHeap<JS<T>> {
    /// Create a new `MutNullableHeap`.
    pub fn new(initial: Option<&T>) -> MutNullableHeap<JS<T>> {
        debug_assert!(thread_state::get().is_script());
        MutNullableHeap {
            ptr: UnsafeCell::new(initial.map(JS::from_ref)),
        }
    }

    /// Retrieve a copy of the current inner value. If it is `None`, it is
    /// initialized with the result of `cb` first.
    pub fn or_init<F>(&self, cb: F) -> Root<T>
        where F: FnOnce() -> Root<T>
    {
        debug_assert!(thread_state::get().is_script());
        match self.get() {
            Some(inner) => inner,
            None => {
                let inner = cb();
                self.set(Some(&inner));
                inner
            },
        }
    }

    /// Retrieve a copy of the inner optional `JS<T>` as `LayoutJS<T>`.
    /// For use by layout, which can't use safe types like Temporary.
    
    pub unsafe fn get_inner_as_layout(&self) -> Option<LayoutJS<T>> {
        debug_assert!(thread_state::get().is_layout());
        ptr::read(self.ptr.get()).map(|js| js.to_layout())
    }

    /// Get a rooted value out of this object
    
    pub fn get(&self) -> Option<Root<T>> {
        debug_assert!(thread_state::get().is_script());
        unsafe {
            ptr::read(self.ptr.get()).map(|o| Root::from_ref(&*o))
        }
    }

    /// Set this `MutNullableHeap` to the given value.
    pub fn set(&self, val: Option<&T>) {
        debug_assert!(thread_state::get().is_script());
        unsafe {
            *self.ptr.get() = val.map(|p| JS::from_ref(p));
        }
    }

}

impl<T> PartialEq for MutNullableHeap<JS<T>> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            *self.ptr.get() == *other.ptr.get()
        }
    }
}

impl<'a, T> PartialEq<Option<&'a T>> for MutNullableHeap<JS<T>> {
    fn eq(&self, other: &Option<&T>) -> bool {
        unsafe {
            *self.ptr.get() == other.map(JS::from_ref)
        }
    }
}

impl<T> Default for MutNullableHeap<T> {
    
    fn default() -> MutNullableHeap<T> {
        debug_assert!(thread_state::get().is_script());
        MutNullableHeap {
            ptr: UnsafeCell::new(None),
        }
    }
}

impl<T> HeapSizeOf for MutNullableHeap<T> {
    fn heap_size_of_children(&self) -> usize {
        // See comment on HeapSizeOf for JS<T>.
        0
    }
}

impl<T> LayoutJS<T> {
    /// Returns an unsafe pointer to the interior of this JS object. This is
    /// the only method that be safely accessed from layout. (The fact that
    /// this is unsafe is what necessitates the layout wrappers.)
    pub unsafe fn unsafe_get(&self) -> *const T {
        debug_assert!(thread_state::get().is_layout());
        *self.ptr
    }
}

/// Get an `Option<&T>` out of an `Option<Root<T>>`
pub trait RootedReference<T> {
    /// Obtain a safe optional reference to the wrapped JS owned-value that
    /// cannot outlive the lifetime of this root.
    fn r(&self) -> Option<&T>;
}

impl<T> RootedReference<T> for Option<Root<T>> {
    fn r(&self) -> Option<&T> {
        self.as_ref().map(|root| root.r())
    }
}

/// Get an `Option<&T> out of an `Option<JS<T>>`
impl<T> RootedReference<T> for Option<JS<T>> {
    fn r(&self) -> Option<&T> {
        self.as_ref().map(|inner| &**inner)
    }
}

/// Get an `Option<Option<&T>>` out of an `Option<Option<Root<T>>>`
pub trait OptionalRootedReference<T> {
    /// Obtain a safe optional optional reference to the wrapped JS owned-value
    /// that cannot outlive the lifetime of this root.
    fn r(&self) -> Option<Option<&T>>;
}

impl<T> OptionalRootedReference<T> for Option<Option<Root<T>>> {
    fn r(&self) -> Option<Option<&T>> {
        self.as_ref().map(|inner| inner.r())
    }
}

/// A rooted reference to a DOM object.
///
/// The JS value is pinned for the duration of this object's lifetime; roots
/// are additive, so this object's destruction will not invalidate other roots
/// for the same JS value. `Root`s cannot outlive the associated
/// `RootCollection` object.

#[Derive(Clone)]
pub struct Root<T> {
    /// Reference to rooted value that must not outlive this container
    ptr: NonZero<*const T>
}

impl<T: Castable> Root<T> {
    /// Cast a DOM object root upwards to one of the interfaces it derives from.
    pub fn upcast<U>(root: Root<T>) -> Root<U>
        where U: Castable,
              T: DerivedFrom<U>
    {
        unsafe { mem::transmute(root) }
    }

    /// Cast a DOM object root downwards to one of the interfaces it might implement.
    pub fn downcast<U>(root: Root<T>) -> Option<Root<U>>
        where U: DerivedFrom<T>
    {
        if root.is::<U>() {
            Some(unsafe { mem::transmute(root) })
        } else {
            None
        }
    }
}

impl<T> Root<T> {

    /// Create a new stack-bounded root for the provided JS-owned value.
    /// It cannot not outlive its associated `RootCollection`, and it gives
    /// out references which cannot outlive this new `Root`.
    pub fn new_box(value: Box<T>) -> Root<T> {
        unsafe {
            Root::new(mem::transmute(Box::into_raw(value)))
        }
    }

    /// Create a new stack-bounded root for the provided JS-owned value.
    /// It cannot not outlive its associated `RootCollection`, and it gives
    /// out references which cannot outlive this new `Root`.
    pub fn new(unrooted: NonZero<*const T>) -> Root<T> {
        debug_assert!(thread_state::get().is_script());
        Root {
            ptr: unrooted
        }
    }

    /// Generate a new root from a reference
    pub fn from_ref(unrooted: &T) -> Root<T> {
        Root::new(unsafe { NonZero::new(&*unrooted) })
    }

    /// Obtain a safe reference to the wrapped JS owned-value that cannot
    /// outlive the lifetime of this root.
    pub fn r(&self) -> &T {
        &**self
    }
}

impl<T> Deref for Root<T> {
    type Target = T;
    fn deref(&self) -> &T {
        debug_assert!(thread_state::get().is_script());
        unsafe { &**self.ptr.deref() }
    }
}

impl<T> PartialEq for Root<T> {
    fn eq(&self, other: &Root<T>) -> bool {
        self.ptr == other.ptr
    }
}
