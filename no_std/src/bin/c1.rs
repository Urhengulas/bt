#![feature(allocator_api)]
#![feature(default_alloc_error_handler)]
#![feature(ptr_metadata)]
#![no_main]
#![no_std]

extern crate alloc;

use alloc::alloc::{Allocator, Global, Layout};
use core::{
    fmt,
    ptr::{self, NonNull},
    slice,
};

use defmt::{println, Debug2Format, Display2Format};
use linked_list_allocator::LockedHeap;
use no_std as _; // global logger + panicking-behavior + memory layout

#[global_allocator]
static GLOBAL_ALLOC: LockedHeap = LockedHeap::empty();
static mut HEAP_1: [u8; 1024] = [0; 1024];

#[derive(Debug)]
struct LeakyVec<T, A: Allocator> {
    alloc: A,
    _cap: usize,
    len: usize,
    ptr: NonNull<T>,
}

impl<T, A: Allocator> LeakyVec<T, A> {
    fn with_capacity_in(cap: usize, alloc: A) -> Self {
        let ptr = alloc.allocate(Layout::array::<T>(cap).unwrap()).unwrap();
        Self {
            alloc,
            _cap: cap,
            len: 0,
            ptr: ptr.cast(),
        }
    }

    fn push(&mut self, value: T) {
        // NOTE: does _not_ grow capacity, since this is not needed for the experiment

        unsafe {
            let end = self.ptr.as_ptr().add(self.len);
            ptr::write(end, value);
            self.len += 1;
        }
    }
}

impl<T: fmt::Debug, A: Allocator> fmt::Display for LeakyVec<T, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = unsafe { slice::from_raw_parts(self.ptr.as_ptr(), self.len) };
        f.debug_list().entries(a.iter()).finish()
    }
}

impl<T, A: Allocator> Drop for LeakyVec<T, A> {
    fn drop(&mut self) {
        // drop values
        unsafe { ptr::drop_in_place(ptr::slice_from_raw_parts_mut(self.ptr.as_ptr(), self.len)) };

        // deallocate memory
        let n = self.len; // BUG: should use `self.cap` instead of `self.len`
        let layout = Layout::array::<T>(n).unwrap();
        unsafe { self.alloc.deallocate(self.ptr.cast(), layout) }
        println!("Deallocate {} bytes!", n);
    }
}

fn notmain() {
    let before = GLOBAL_ALLOC.lock().free();

    let mut leak = LeakyVec::<u32, _>::with_capacity_in(10, &Global);
    for i in 0..5 {
        leak.push(i * i);
    }
    println!("{:?} = {}", Debug2Format(&leak), Display2Format(&leak));
    drop(leak);

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
    unsafe { GLOBAL_ALLOC.lock().init(HEAP_1.as_mut_ptr(), HEAP_1.len()) };
    println!("Initialized heap!");
}
