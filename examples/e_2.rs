#![feature(allocator_api)]

use std::{
    alloc::{Allocator, Layout, System},
    mem,
    ptr::NonNull,
};

fn main() {
    // allocate a Vec
    let mut a = vec![1, 2, 3, 4, 5];
    dbg!(&a);

    // deallocate memory
    let ptr = a.as_mut_ptr();
    unsafe {
        System.deallocate(NonNull::new(ptr).unwrap(), Layout::new::<Vec<u8>>());
    }

    // access memory after deallocation; gives garbage values
    dbg!(&a);

    // allocate a String; this will have the same memory address as the Vec had
    let mut b = "Hello, world!".to_string();
    assert_eq!(b.as_mut_ptr(), ptr);
    dbg!(&a, &b.as_bytes());

    // write to the old vec; this will also write into the new String
    for (idx, val) in a.iter_mut().enumerate() {
        *val = 2 * (idx as u8);
    }

    // scary!
    dbg!(&a, &b.as_bytes(), &b);

    // forget about memory to avoid double free error
    mem::forget(a);
}
