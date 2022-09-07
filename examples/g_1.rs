#![feature(allocator_api)]

use std::{
    alloc::{AllocError, Allocator, Layout, System},
    ptr::NonNull,
    thread,
    time::Duration,
};

static MY_ALLOC: MyAllocator = MyAllocator;

struct MyAllocator;

unsafe impl Allocator for MyAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        thread::sleep(Duration::from_secs(2));
        println!("Allocate!");
        System.allocate(layout)
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        System.deallocate(ptr, layout)
    }
}

#[derive(Debug)]
struct SlowDrop(u32);

impl Drop for SlowDrop {
    fn drop(&mut self) {
        thread::sleep(Duration::from_secs(1));
        println!("Drop!");
    }
}

fn main() {
    let mut a = Vec::new_in(&MY_ALLOC);
    (0..3).for_each(|i| a.push(SlowDrop(i)));
    dbg!(&a, a.as_ptr());

    let mut b = Vec::new_in(&MY_ALLOC);
    (0..5).for_each(|i| b.push(SlowDrop(i)));
    dbg!(&b, b.as_ptr());
}
