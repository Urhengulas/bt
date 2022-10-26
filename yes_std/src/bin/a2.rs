#![feature(allocator_api)]

use std::{
    alloc::{AllocError, Allocator, Global, Layout},
    ptr::NonNull,
};

static MY_ALLOC: MyAllocator = MyAllocator;
static mut GLOBAL_PTR: GlobalPtr = GlobalPtr::new();

struct MyAllocator;

unsafe impl Allocator for MyAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        Ok(unsafe { GLOBAL_PTR.get(layout) })
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        Global.deallocate(ptr, layout)
    }
}

struct GlobalPtr {
    ptr: Option<NonNull<[u8]>>,
}

impl GlobalPtr {
    const fn new() -> Self {
        Self { ptr: None }
    }

    fn get(&mut self, layout: Layout) -> NonNull<[u8]> {
        if self.ptr.is_none() {
            let ptr = Global.allocate(layout).unwrap();
            self.ptr = Some(ptr);
        }
        self.ptr.unwrap()
    }
}

fn main() {
    // allocate vector and push some elements, all looks nice so far
    // NOTE: uses with_capacity to avoid reallocation
    let mut a = Vec::with_capacity_in(6, &MY_ALLOC);
    for i in 0..6 {
        a.push(i);
    }
    println!("a: {a:?}");
    separator();

    // allocate another vector and push some elements
    // this overwrites the earlier vector
    let mut b = Vec::with_capacity_in(3, &MY_ALLOC);
    for i in 0..3 {
        b.push(i * 10);
    }
    assert_eq!(a.as_ptr(), b.as_ptr());
    println!("a: {a:?}\nb: {b:?}");
    separator();

    // push more elements in second vector to force reallocation
    for i in 0..3 {
        b.push(i * 100);
    }
    println!("a: {a:?}\nb: {b:?}");
}

fn separator() {
    println!("{}", "-".repeat(70));
}
