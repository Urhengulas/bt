#![feature(allocator_api)]

use std::{
    alloc::{Allocator, Layout, System},
    mem,
    ptr::NonNull,
};

fn main() {
    // allocate a Vec
    println!("# allocate a");
    let mut a = (0..3).into_iter().map(|i| Dropper(i)).collect::<Vec<_>>();
    println!("a: {:?}\n", &a);

    // deallocate memory
    println!("# deallocate a");
    let ptr = a.as_mut_ptr().cast();
    let layout = Layout::array::<Dropper>(a.capacity()).unwrap();
    unsafe {
        System.deallocate(NonNull::new(ptr).unwrap(), layout);
    }

    // access memory after deallocation; gives garbage values
    println!("a: {:?}\n", &a);

    // allocate a String; this will use the same memory address as a
    println!("# allocate b");
    let mut b = "Hello, world!".to_string();
    assert_eq!(ptr, b.as_mut_ptr());
    println!("b: {:?} ({:?})\na: {:?}\n", &b.as_bytes(), &b, &a);

    println!("# write to a");
    // write to the old vec; this will also write into the new String
    for (idx, val) in a.iter_mut().enumerate() {
        val.0 = 2 * idx as u16;
    }

    // scary!
    println!("a: {:?},\nb: {:?} ({:?})", &a, &b.as_bytes(), &b);

    // forget about memory to avoid double free error
    mem::forget(b);
}

#[derive(Debug)]
struct Dropper(u16);

impl Drop for Dropper {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
    }
}
