#![feature(allocator_api)]
#![feature(default_alloc_error_handler)]
#![feature(ptr_metadata)]
#![no_main]
#![no_std]

extern crate alloc;

use alloc::{
    alloc::{AllocError, Allocator, Global, Layout},
    vec::Vec,
};
use core::{
    ptr::{self, NonNull},
    slice,
    sync::atomic::AtomicU32,
};

use defmt::{dbg, println};
use linked_list_allocator::LockedHeap;
use no_std as _; // global logger + panicking-behavior + memory layout

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();
static mut HEAP: [u8; 1024] = [0; 1024];
static MY_ALLOC: MyAllocator = MyAllocator;

struct MyAllocator;

unsafe impl Allocator for MyAllocator {
    fn allocate(&self, _layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let ptr = unsafe {
            let a = ALLOCATOR.lock().top().add(4 * _layout.align());
            slice::from_raw_parts_mut(a as *mut u8, 10) as *mut _
        };
        Ok(NonNull::new(ptr).unwrap())
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        Global.deallocate(ptr, layout)
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    init_heap();
    notmain();
    no_std::exit()
}

fn notmain() {
    let mut a = Vec::<i32, _>::new_in(&MY_ALLOC);
    for i in 0..5 {
        a.push(i);
    }
    dbg!(&a.as_slice(), a.as_ptr());

    let mut b = Vec::<i32, _>::new_in(&MY_ALLOC);
    for i in 0..5 {
        b.push(i * 2);
    }
    dbg!(&b.as_slice(), b.as_ptr());
}

pub fn init_heap() {
    unsafe {
        let heap_start = HEAP.as_mut_ptr();
        let heap_size = HEAP.len();
        ALLOCATOR.lock().init(heap_start, heap_size);
    };
    println!("Initialized heap!");
}
