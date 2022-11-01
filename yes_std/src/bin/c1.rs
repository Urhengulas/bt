#![feature(allocator_api)]

use std::{
    alloc::{Allocator, Global, Layout},
    fmt,
    ptr::{self, NonNull},
    slice,
};

#[derive(Debug)]
struct LeakyVec<T, A: Allocator> {
    alloc: A,
    _cap: usize,
    len: usize,
    ptr: NonNull<T>,
}

impl<T, A: Allocator> LeakyVec<T, A> {
    fn with_capacity_in(cap: usize, alloc: A) -> Self {
        let ptr = alloc.allocate(Layout::array::<T>(cap).unwrap()).unwrap();
        Self {
            alloc,
            _cap: cap,
            len: 0,
            ptr: ptr.cast(),
        }
    }

    fn push(&mut self, value: T) {
        // NOTE: does _not_ grow capacity, since this is not needed for the experiment

        unsafe {
            let end = self.ptr.as_ptr().add(self.len);
            ptr::write(end, value);
            self.len += 1;
        }
    }
}

impl<T: fmt::Debug, A: Allocator> fmt::Display for LeakyVec<T, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = unsafe { slice::from_raw_parts(self.ptr.as_ptr(), self.len) };
        f.debug_list().entries(a.iter()).finish()
    }
}

impl<T, A: Allocator> Drop for LeakyVec<T, A> {
    fn drop(&mut self) {
        // drop values
        unsafe { ptr::drop_in_place(ptr::slice_from_raw_parts_mut(self.ptr.as_ptr(), self.len)) };

        // deallocate memory
        let n = self.len; // BUG: should use `self.cap` instead of `self.len`
        let layout = Layout::array::<T>(n).unwrap();
        unsafe { self.alloc.deallocate(self.ptr.cast(), layout) }
        println!("Deallocate {} bytes!", n);
    }
}

fn main() {
    let mut a = LeakyVec::with_capacity_in(10, &Global);
    for i in 0..5 {
        a.push(i * i);
    }
    println!("{:?} = {}", &a, &a);
}
