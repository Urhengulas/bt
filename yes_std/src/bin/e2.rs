#![feature(allocator_api)]

use std::{
    alloc::{AllocError, Allocator, Global, Layout},
    ptr::NonNull,
};

static MY_ALLOC: MyAllocator = MyAllocator;

struct MyAllocator;

unsafe impl Allocator for MyAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let ptr = Global.allocate(layout).unwrap();
        unsafe { Global.deallocate(ptr.cast(), layout) };
        Ok(ptr)
    }

    unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
        // Global.deallocate(ptr, layout)
    }
}

fn main() {
    println!("allocate A");
    let mut a = Vec::new_in(&MY_ALLOC);
    (0..4).for_each(|i| a.push(i));
    log(&a, "A");

    println!("allocate B");
    let mut b = Vec::new_in(&MY_ALLOC);
    (0..4).for_each(|i| b.push(i * 2));
    log(&b, "B");
    log(&a, "A");
}

fn log(x: &[i32], name: &str) {
    println!("{}: {:?} (ptr={:?})", name, &x, x.as_ptr());
}
