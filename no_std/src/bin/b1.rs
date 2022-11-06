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
static ALLOCATOR: LockedHeap = LockedHeap::empty();
static mut HEAP: [u8; 64] = [0; 64];

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

// ----------------------------------------------------------------------------

const SMALL: [u8; 16] = [0; 16];
const BIG: [u8; 32] = [0; 32];

fn notmain() {
    assert_eq!(ALLOCATOR.lock().free(), 64);

    // allocate four small objects
    let _a = Box::try_new_in(SMALL, &ALLOCATOR).unwrap();
    let _b = Box::try_new_in(SMALL, &ALLOCATOR).unwrap();
    let _c = Box::try_new_in(SMALL, &ALLOCATOR).unwrap();
    let _d = Box::try_new_in(SMALL, &ALLOCATOR).unwrap();
    assert_eq!(ALLOCATOR.lock().free(), 0);

    // deallocate two small objects; they are _not_ consecutive
    drop(_a);
    drop(_c);
    assert_eq!(ALLOCATOR.lock().free(), 32);

    // allocating one big object fails, although there is theoretically enough memory available
    let _e = Box::try_new_in(BIG, &ALLOCATOR).unwrap_err();

    // allocating two small objects succeeds; combined they have the same size as the big object
    let _f = Box::try_new_in(SMALL, &ALLOCATOR).unwrap();
    let _g = Box::try_new_in(SMALL, &ALLOCATOR).unwrap();
}
