#![feature(allocator_api)]
#![feature(default_alloc_error_handler)]
#![feature(ptr_metadata)]
#![no_main]
#![no_std]

extern crate alloc;

use alloc::{string::ToString, vec::Vec};

use defmt::{error, info, println};
use linked_list_allocator::LockedHeap;
use no_std as _; // global logger + panicking-behavior + memory layout

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();
static mut HEAP: [u8; 100 * KB] = [0; 100 * KB];

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

const KB: usize = 1024;

fn notmain() {
    let mut a = Vec::new();
    loop {
        push_one_kb(&mut a);
    }
}

fn push_one_kb(a: &mut Vec<[u8; KB]>) {
    match a.try_reserve_exact(1) {
        Ok(_) => {
            a.push([0; KB]);
        }
        Err(e) => {
            error!("AllocationError: {}.", e.to_string().as_str());
            info!("Handle allocation error gracefully.");
            no_std::exit();
        }
    }
}
