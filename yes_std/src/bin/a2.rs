#![feature(allocator_api)]

use core::slice;
use std::{
    alloc::{AllocError, Allocator, Global, Layout},
    ptr::{self, NonNull},
};

static MY_ALLOC: MyAllocator = MyAllocator;

struct MyAllocator;

unsafe impl Allocator for MyAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        unsafe {
            let ptr = slice::from_raw_parts_mut(ptr::null_mut(), layout.size()) as *mut _;
            let non_null = NonNull::new_unchecked(ptr);
            dbg!(layout, non_null);
            Ok(non_null)
        }
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        Global.deallocate(ptr, layout)
    }
}

fn main() {
    let mut a = Vec::<i32, _>::new_in(&MY_ALLOC);
    dbg!(&a, a.as_ptr());

    for i in 0..5 {
        a.push(i);
    }
    dbg!(&a, a.as_ptr());
}
