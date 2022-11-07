#![feature(allocator_api)]
#![feature(default_alloc_error_handler)]
#![feature(ptr_metadata)]
#![no_main]
#![no_std]

extern crate alloc;

use defmt::{info, println, Debug2Format};
use linked_list_allocator::LockedHeap;
use no_std as _; // global logger + panicking-behavior + memory layout

#[global_allocator]
static GLOBAL_ALLOC: LockedHeap = LockedHeap::empty();
static mut HEAP: [u8; 1024] = [0; 1024];

#[cortex_m_rt::entry]
fn main() -> ! {
    init_heap();
    notmain();
    no_std::exit()
}

fn init_heap() {
    unsafe { GLOBAL_ALLOC.lock().init(HEAP.as_mut_ptr(), HEAP.len()) };
    info!("Initialized heap!");
}

// -------------------------------------------

use alloc::rc::Rc;
use core::{cell::RefCell, ops::Deref};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<Self>>),
    Nil,
}

impl List {
    fn cons(n: i32, list: Rc<Self>) -> Rc<Self> {
        Rc::new(Self::Cons(n, RefCell::new(list)))
    }
}

fn notmain() {
    let a = List::cons(1, Rc::new(List::Nil));
    let b = List::cons(2, Rc::clone(&a));

    match a.deref() {
        List::Cons(_, c) => *c.borrow_mut() = Rc::clone(&b),
        List::Nil => (),
    }

    println!(
        "rc count: a={}, b={}",
        Rc::strong_count(&a),
        Rc::strong_count(&b)
    );

    // WARN: the next line creates a stack overflow
    println!("{:?}", Debug2Format(&a));
}
