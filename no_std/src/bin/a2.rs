#![feature(allocator_api)]
#![feature(default_alloc_error_handler)]
#![no_main]
#![no_std]

extern crate alloc;

use alloc::{
    alloc::{AllocError, Allocator, Global, Layout},
    vec::Vec,
};
use core::ptr::NonNull;

use defmt::println;
use linked_list_allocator::LockedHeap;
use no_std as _; // global logger + panicking-behavior + memory layout

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();
static mut GLOBAL_PTR: GlobalPtr = GlobalPtr::new();
static mut HEAP: [u8; 1024] = [0; 1024];
static MY_ALLOC: MyAllocator = MyAllocator;

struct MyAllocator;

unsafe impl Allocator for MyAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        println!("allocate");
        Ok(unsafe { GLOBAL_PTR.get(layout) })
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        println!("deallocate");
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
            let ptr = ALLOCATOR.allocate(layout).unwrap();
            self.ptr = Some(ptr);
        }
        self.ptr.unwrap()
    }
}

fn not_main() {
    // allocate vector and push some elements, all looks nice so far
    // NOTE: uses with_capacity to avoid reallocation
    let mut a = Vec::with_capacity_in(6, &MY_ALLOC);
    for i in 0..6 {
        a.push(i);
    }
    println!("a: {:?}", a.as_slice());
    separator();

    // allocate another vector and push some elements
    // this overwrites the earlier vector
    let mut b = Vec::with_capacity_in(3, &MY_ALLOC);
    for i in 0..3 {
        b.push(i * 10);
    }
    assert_eq!(a.as_ptr(), b.as_ptr());
    println!("a: {:?}\nb: {:?}", a.as_slice(), b.as_slice());
    separator();

    // push more elements in second vector to force reallocation
    for i in 0..3 {
        b.push(i * 100);
    }
    println!("a: {:?}\nb: {:?}", a.as_slice(), b.as_slice());
}

#[cortex_m_rt::entry]
fn main() -> ! {
    init_heap();
    not_main();
    no_std::exit()
}

pub fn init_heap() {
    unsafe {
        let heap_start = HEAP.as_mut_ptr();
        let heap_size = HEAP.len();
        ALLOCATOR.lock().init(heap_start, heap_size);
    }
}

fn separator() {
    println!("{}", "-".repeat(70).as_str());
}
