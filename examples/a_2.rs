#![feature(allocator_api)]

use std::{
    alloc::{AllocError, Allocator, Layout, System},
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
        System.deallocate(ptr, layout)
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
            let ptr = System.allocate(layout).unwrap();
            self.ptr = Some(ptr);
        }
        self.ptr.unwrap()
    }
}

fn main() {
    // allocate vector and push some elements, all looks nice so far
    // NOTE: uses with_capacity to avoid reallocation
    let mut a = Vec::with_capacity_in(10, &MY_ALLOC);
    for i in 0..10 {
        a.push(i);
    }
    dbg!(&a, a.as_ptr());
    separator();

    // allocate another vector and push some elements
    // this overwrites the earlier vector
    let mut b = Vec::with_capacity_in(5, &MY_ALLOC);
    for i in 0..5 {
        b.push(i * 10);
    }
    dbg!(&a, a.as_ptr(), &b, b.as_ptr());
    separator();

    // allocate another vector, this time push more elements than we have capacity to trigger reallocation
    let mut c = Vec::with_capacity_in(5, &MY_ALLOC);
    for i in 0..10 {
        c.push(i * 10);
    }
    dbg!(&a, a.as_ptr(), &b, b.as_ptr(), &c, c.as_ptr());
}

fn separator() {
    println!("{}", "-".repeat(50));
}
