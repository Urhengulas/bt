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
    alloc::LayoutError,
    ptr::{self, NonNull},
    slice,
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
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let ptr = Global.allocate(layout).unwrap();
        let ptr =
            unsafe { slice::from_raw_parts_mut(ptr.as_ptr().cast::<u8>().offset(1), ptr.len()) };
        Ok(NonNull::new(ptr as *mut _).unwrap())
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        Global.deallocate(ptr, layout)
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    init_heap();
    not_main();
    no_std::exit()
}

fn not_main() {
    let mut a = Vec::new_in(&MY_ALLOC);
    for i in 0..5 {
        a.push(i);
    }
    dbg!(&a.as_slice(), a.as_ptr());

    let mut b = Vec::new_in(&MY_ALLOC);
    for i in 0..5 {
        b.push(i);
    }
    dbg!(&b.as_slice(), b.as_ptr());
}

fn init_heap() {
    unsafe {
        let heap_start = HEAP.as_mut_ptr();
        let heap_size = HEAP.len();
        ALLOCATOR.lock().init(heap_start, heap_size);
    };
    println!("Initialized heap!");
}
