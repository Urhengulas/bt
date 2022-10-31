#![feature(allocator_api)]
#![feature(default_alloc_error_handler)]
#![feature(ptr_metadata)]
#![no_main]
#![no_std]

extern crate alloc;

use alloc::{
    alloc::{Allocator, Global, Layout},
    string::{String, ToString},
    vec::Vec,
};
use core::ptr::NonNull;

use defmt::{println, write, Format};
use linked_list_allocator::LockedHeap;
use no_std as _; // global logger + panicking-behavior + memory layout

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();
static mut HEAP: [u8; 1024] = [0; 1024];

fn notmain() {
    // allocate Vec A
    println!("# allocate A");
    let mut a = (0..5).into_iter().map(|i| Dropper(i)).collect::<Vec<_>>();
    print_a(&a);

    // deallocate memory of A, but keep raw pointer
    println!("# deallocate A");
    let ptr = a.as_mut_ptr().cast();
    let layout = Layout::array::<Dropper>(a.capacity()).unwrap();
    unsafe { Global.deallocate(NonNull::new(ptr).unwrap(), layout) };

    // access memory of A after deallocation; gives garbage values
    print_a(&a);

    // allocate String B; this might (and in our case will) use the same memory address as A
    println!("# allocate B");
    let mut b = "Hello, world!".to_string();
    assert_eq!(ptr, b.as_mut_ptr());
    print_a(&a);
    print_b(&b);

    // write to the old vec; this will also write into the new String
    println!("# write to A");
    for (idx, val) in a.iter_mut().enumerate() {
        val.0 = 2 * idx as u8;
    }

    // scary!
    print_a(&a);
    print_b(&b);
}

#[derive(Debug)]
struct Dropper(u8);

impl Drop for Dropper {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
    }
}

impl Format for Dropper {
    fn format(&self, fmt: defmt::Formatter) {
        write!(fmt, "Dropper({})", self.0)
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

fn print_a(a: &[Dropper]) {
    println!("A: {:?}", &a,);
}

fn print_b(b: &String) {
    println!("B: {:?} ({:?})", &b.as_bytes(), &b.as_str(),);
}
