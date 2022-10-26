#![feature(allocator_api)]

use std::{
    alloc::{AllocError, Allocator, Global, Layout},
    ptr::NonNull,
    slice,
};

static MY_ALLOC: MyAllocator = MyAllocator;

struct MyAllocator;

unsafe impl Allocator for MyAllocator {
    fn allocate(&self, _layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        unsafe {
            let a = slice::from_raw_parts_mut(0x1234 as *mut u8, 10) as *mut _;
            Ok(NonNull::new(a).unwrap())
        }
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        Global.deallocate(ptr, layout)
    }
}

fn main() {
    let mut a = Vec::<i32, _>::new_in(&MY_ALLOC);
    for i in 0..5 {
        a.push(i);
    }
    dbg!(&a, a.as_ptr());

    let mut b = Vec::<i32, _>::new_in(&MY_ALLOC);
    for i in 0..5 {
        b.push(i * 2);
    }
    dbg!(&b, b.as_ptr());
}
