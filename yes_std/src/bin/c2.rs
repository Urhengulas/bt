#![feature(allocator_api)]

use std::{
    alloc::{Allocator, Global, Layout},
    ptr::NonNull,
};

#[derive(Debug)]
struct LeakyVec<T, A: Allocator> {
    alloc: A,
    cap: usize,
    len: usize,
    ptr: NonNull<T>,
}

impl<T, A: Allocator> LeakyVec<T, A> {
    fn with_capacity_in(cap: usize, alloc: A) -> Self {
        let ptr = alloc.allocate(Layout::array::<T>(cap).unwrap()).unwrap();
        Self {
            alloc,
            cap,
            len: 0,
            ptr: ptr.cast(),
        }
    }
}

impl<T, A: Allocator> Drop for LeakyVec<T, A> {
    fn drop(&mut self) {
        let layout = Layout::array::<T>(self.len).unwrap(); // bug: should use `self.cap` instead of `self.len`
        unsafe { self.alloc.deallocate(self.ptr.cast(), layout) }
    }
}

fn main() {
    let a = LeakyVec::<u32, _>::with_capacity_in(10, &Global);
    dbg!(a);
}
