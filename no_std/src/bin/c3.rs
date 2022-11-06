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
use core::cell::RefCell;

#[derive(Debug)]
enum List {
    Cons(RefCell<Rc<Self>>),
    Nil,
}

impl List {
    fn cons(list: Rc<Self>) -> Rc<Self> {
        Rc::new(Self::Cons(RefCell::new(list)))
    }

    fn tail(&self) -> Option<&RefCell<Rc<Self>>> {
        match self {
            Self::Cons(item) => Some(item),
            Self::Nil => None,
        }
    }
}

fn notmain() {
    let a = List::cons(Rc::new(List::Nil));
    let b = List::cons(Rc::clone(&a));
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!(
        "rc count: a={}, b={}",
        Rc::strong_count(&a),
        Rc::strong_count(&b)
    );

    println!("a next item = {:?}", Debug2Format(&a.tail()));
}
