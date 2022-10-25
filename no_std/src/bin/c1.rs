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
    alloc::GlobalAlloc,
    ptr::{self, NonNull},
    slice,
};

use defmt::{dbg, println};
use no_std as _; // global logger + panicking-behavior + memory layout

static mut MEMORY: [u8; 300] = [0; 300];
#[global_allocator]
static ALLOC: Locked<BumpAllocator> = Locked::new(BumpAllocator::new());

#[cortex_m_rt::entry]
fn main() -> ! {
    init_heap();
    notmain();
    no_std::exit()
}

fn notmain() {
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

    println!("Initialized heap!");
}

#[derive(Debug)]
pub struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
    next: usize,
    allocations: usize,
}

impl BumpAllocator {
    /// Creates a new empty bump allocator.
    pub const fn new() -> Self {
        BumpAllocator {
            heap_start: 0,
            heap_end: 0,
            next: 0,
            allocations: 0,
        }
    }

    /// Initializes the bump allocator with the given heap bounds.
    ///
    /// This method is unsafe because the caller must ensure that the given
    /// memory range is unused. Also, this method must be called only once.
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.heap_start = heap_start;
        self.heap_end = heap_start + heap_size;
        self.next = heap_start;

        dbg!(self);
    }
}

/// A wrapper around spin::Mutex to permit trait implementations.
#[derive(Debug)]
pub struct Locked<T> {
    inner: spin::Mutex<T>,
}

impl<T> Locked<T> {
    pub const fn new(inner: T) -> Self {
        Locked {
            inner: spin::Mutex::new(inner),
        }
    }

    pub fn lock(&self) -> spin::MutexGuard<T> {
        self.inner.lock()
    }
}

unsafe impl Allocator for Locked<BumpAllocator> {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let mut bump = self.lock(); // get a mutable reference

        let alloc_start = align_up(bump.next, layout.align());
        let alloc_end = match alloc_start.checked_add(layout.size()) {
            Some(end) => end,
            None => return Err(AllocError),
        };

        if alloc_end > bump.heap_end {
            Err(AllocError)
        } else {
            bump.next = alloc_end;
            bump.allocations += 1;
            let ptr = unsafe {
                slice::from_raw_parts_mut(alloc_start as *mut u8, layout.size()) as *mut _
            };
            println!("allocate:   ptr={:?}, layout={:?}", ptr, layout);
            dbg!(&bump);
            Ok(NonNull::new(ptr).unwrap())
        }
    }

    unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
        let mut bump = self.lock(); // get a mutable reference

        bump.allocations -= 1;
        if bump.allocations == 0 {
            println!("reset!");
            bump.next = bump.heap_start;
        }

        println!("deallocate: ptr={:?}, layout={:?}", _ptr.as_ptr(), _layout);
    }
}

unsafe impl GlobalAlloc for Locked<BumpAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.allocate(layout).unwrap().as_ptr().cast()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.deallocate(NonNull::new(ptr).unwrap(), layout);
    }
}

/// Align the given address `addr` upwards to alignment `align`.
///
/// Requires that `align` is a power of two.
fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}
