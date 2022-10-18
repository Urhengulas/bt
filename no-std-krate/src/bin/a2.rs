#![feature(allocator_api)]
#![feature(default_alloc_error_handler)]
#![no_main]
#![no_std]

extern crate alloc;

use core::ptr::NonNull;

use alloc::{
    alloc::{AllocError, Allocator, Layout},
    vec::Vec,
};
use linked_list_allocator::LockedHeap;
use no_std_krate as _; // global logger + panicking-behavior + memory layout

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();
static mut GLOBAL_PTR: GlobalPtr = GlobalPtr::new();
static mut HEAP: [u8; 1024] = [0; 1024];
static MY_ALLOC: MyAllocator = MyAllocator;

struct MyAllocator;

unsafe impl Allocator for MyAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        Ok(unsafe { GLOBAL_PTR.get(layout) })
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        // if deallocation is omitted the example just runs through without any complaints
        ALLOCATOR.deallocate(ptr, layout)
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

#[cortex_m_rt::entry]
fn main() -> ! {
    init_heap();

    let mut a = Vec::with_capacity_in(5, &MY_ALLOC);
    defmt::dbg!(a.as_slice());
    for i in 0..20 {
        a.push(i);
        defmt::dbg!(a.as_slice());
    }

    defmt::println!("{}", a.as_slice());

    no_std_krate::exit()
}

pub fn init_heap() {
    unsafe {
        let heap_start = HEAP.as_mut_ptr();
        let heap_size = HEAP.len();
        ALLOCATOR.lock().init(heap_start, heap_size);
    }
}
