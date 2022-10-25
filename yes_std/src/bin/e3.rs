#![feature(allocator_api)]

use std::{
    alloc::{AllocError, Allocator, Layout, System},
    ptr::NonNull,
};

static MY_ALLOC: MyAllocator = MyAllocator;

struct MyAllocator;

unsafe impl Allocator for MyAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let ptr = System.allocate(layout).unwrap();
        unsafe { System.deallocate(ptr.cast(), layout) };
        Ok(ptr)
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        System.deallocate(ptr, layout)
    }
}

fn main() {
    let mut a = Vec::new_in(&MY_ALLOC);
    (0..3).for_each(|i| a.push(i));
    log(&a);

    let mut b = Vec::new_in(&MY_ALLOC);
    (0..3).for_each(|i| b.push(i * 2));
    log(&b);
    log(&a);
}

fn log(a: &[u32]) {
    dbg!(&a, a.as_ptr());
}
