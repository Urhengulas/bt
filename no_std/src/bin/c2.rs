#![feature(allocator_api)]
#![feature(default_alloc_error_handler)]
#![feature(ptr_metadata)]
#![no_main]
#![no_std]

extern crate alloc;

use alloc::boxed::Box;

use defmt::println;
use linked_list_allocator::LockedHeap;
use no_std as _; // global logger + panicking-behavior + memory layout

#[global_allocator]
static GLOBAL_ALLOC: LockedHeap = LockedHeap::empty();
static mut HEAP: [u8; 1024] = [0; 1024];

fn notmain() {
    let before = GLOBAL_ALLOC.lock().free();

    let a = Box::new(42);
    Box::leak(a);

    let b = Box::new([0, 1, 2, 3, 4]);
    core::mem::forget(b);

    let after = GLOBAL_ALLOC.lock().free();
    assert_eq!(before, after);
}

#[cortex_m_rt::entry]
fn main() -> ! {
    init_heap();
    notmain();
    no_std::exit()
}

fn init_heap() {
    unsafe { GLOBAL_ALLOC.lock().init(HEAP.as_mut_ptr(), HEAP.len()) };
    println!("Initialized heap!");
}
