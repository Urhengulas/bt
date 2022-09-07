#![feature(allocator_api)]

use std::{
    alloc::{AllocError, Allocator, Layout, System},
    ptr::NonNull,
};

static MY_ALLOC: MyAllocator = MyAllocator;

struct MyAllocator;

unsafe impl Allocator for MyAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        System.allocate(layout)
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        System.deallocate(ptr, layout)
    }
}

fn main() {
    let mut a = Vec::with_capacity_in(3, &MY_ALLOC);
    (0..3).for_each(|i| a.push(i));
    dbg!(&a, a.as_ptr());

    let mut b = Vec::with_capacity_in(3, &MY_ALLOC);
    (0..3).for_each(|i| b.push(i * 2));
    dbg!(&b, b.as_ptr());
}
