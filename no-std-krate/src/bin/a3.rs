#![feature(allocator_api)]
#![feature(default_alloc_error_handler)]
#![no_main]
#![no_std]

extern crate alloc;

use alloc::{
    alloc::{AllocError, Allocator, Layout},
    vec::Vec,
};
use core::{
    ptr::{self, NonNull},
    slice,
};

use defmt::{dbg, println};
use linked_list_allocator::LockedHeap;
use no_std_krate as _; // global logger + panicking-behavior + memory layout

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();
static mut HEAP: [u8; 1024] = [0; 1024];
static MY_ALLOC: MyAllocator = MyAllocator;

struct MyAllocator;

unsafe impl Allocator for MyAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        unsafe {
            let ptr = slice::from_raw_parts_mut(ptr::null_mut(), layout.size()) as *mut _;
            let non_null = NonNull::new_unchecked(ptr);
            println!("Allocated!");
            Ok(non_null)
        }
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        ALLOCATOR.deallocate(ptr, layout)
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    init_heap();

    let mut a = Vec::<i32, _>::new_in(&MY_ALLOC);
    dbg!(a.as_slice(), a.as_ptr());

    for i in 0..5 {
        a.push(i);
    }
    dbg!(a.as_slice(), a.as_ptr());

    no_std_krate::exit()
}

pub fn init_heap() {
    unsafe {
        let heap_start = HEAP.as_mut_ptr();
        let heap_size = HEAP.len();
        ALLOCATOR.lock().init(heap_start, heap_size);
    };
    println!("Initialized heap!");
}
