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
use core::ptr::NonNull;

use defmt::println;
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
        unsafe { Global.deallocate(ptr.cast(), layout) };
        Ok(ptr)
    }

    unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
        // Global.deallocate(ptr, layout)
    }
}

fn notmain() {
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

#[derive(Debug)]
struct Dropper(u8);

impl Drop for Dropper {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    init_heap();
    notmain();
    no_std::exit()
}

fn init_heap() {
    unsafe {
        let heap_start = HEAP.as_mut_ptr();
        let heap_size = HEAP.len();
        ALLOCATOR.lock().init(heap_start, heap_size);
    };
    println!("Initialized heap!");
}
