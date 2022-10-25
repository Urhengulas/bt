#![feature(allocator_api)]

use yes_std::{BumpAllocator, Locked};

static mut MEMORY: [u8; 300] = [0; 300];
static ALLOC: Locked<BumpAllocator> = Locked::new(BumpAllocator::new());

fn main() {
    init_heap();

    let mut a = Vec::new_in(&ALLOC);
    for i in 0.. {
        a.push(i);
    }
}

fn init_heap() {
    unsafe {
        let mut allocator = ALLOC.lock();
        allocator.init(MEMORY.as_ptr() as usize, MEMORY.len());
    }
}
