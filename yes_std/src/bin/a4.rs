#![feature(allocator_api)]
#![feature(ptr_metadata)]

use std::{
    alloc::{AllocError, Allocator, Global, Layout},
    ptr::NonNull,
};

static MY_ALLOC: MyAllocator = MyAllocator;

struct MyAllocator;

unsafe impl Allocator for MyAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let layout =
            unsafe { Layout::from_size_align_unchecked(layout.size(), layout.align() - 1) };
        let ptr = Global.allocate(layout).unwrap();
        dbg!(layout, ptr.to_raw_parts());
        Ok(ptr)
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        Global.deallocate(ptr, layout)
    }
}

fn main() {
    let mut a = Vec::new_in(&MY_ALLOC);
    for i in 0..5 {
        a.push(i);
    }
    dbg!(&a, a.as_ptr());

    let mut b = Vec::new_in(&MY_ALLOC);
    for i in 0..5 {
        b.push(i);
    }
    dbg!(&b, b.as_ptr());
}
