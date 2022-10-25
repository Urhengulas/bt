#![feature(allocator_api)]

use std::{
    alloc::{AllocError, Allocator, Layout, System},
    fmt::Debug,
    ptr::NonNull,
    thread,
    time::Duration,
};

static MY_ALLOC: MyAllocator = MyAllocator;
const DUR: Duration = Duration::from_secs(1);

struct MyAllocator;

unsafe impl Allocator for MyAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        thread::sleep(DUR);
        println!("Allocate!");
        System.allocate(layout)
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        thread::sleep(DUR);
        println!("Deallocate!");
        System.deallocate(ptr, layout)
    }
}

#[derive(Debug)]
struct SlowDrop(u32);

impl Drop for SlowDrop {
    fn drop(&mut self) {
        thread::sleep(DUR);
        println!("Drop!");
    }
}

fn main() {
    let mut a = Vec::new_in(&MY_ALLOC);
    (0..10).for_each(|i| a.push(SlowDrop(i)));
    println!("{:?}, {:?}", &a, a.as_ptr());
}
