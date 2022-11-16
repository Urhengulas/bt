#![feature(ptr_as_uninit)]
#![no_main]

use std::{
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

/// A pointer which indicates that the possessor owns the `T`.
///
/// It is non-null, well-aligned and may or may not be initialised.
///
/// Because the `Unique` owns `T`, `Unique` does not alias and therefore is neither `Clone` nor `Copy`.
pub struct Unique<T: ?Sized>(NonNull<T>);

impl<T: ?Sized> Unique<T> {
    /// Create a new `Unique<T>`.
    ///
    /// # Safety
    /// - the `ptr` must be considered consumed
    pub unsafe fn new(ptr: NonNull<T>) -> Self {
        Self { 0: ptr }
    }

    pub fn into_inner(self) -> NonNull<T> {
        self.0
    }
}

impl<T> Deref for Unique<T> {
    type Target = MaybeUninit<T>;
    fn deref(&self) -> &MaybeUninit<T> {
        unsafe { self.0.as_uninit_ref() }
    }
}

impl<T> DerefMut for Unique<T> {
    fn deref_mut(&mut self) -> &mut MaybeUninit<T> {
        unsafe { self.0.as_uninit_mut() }
    }
}
