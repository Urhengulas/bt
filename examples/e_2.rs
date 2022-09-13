#![feature(allocator_api)]

use std::{
    alloc::{Allocator, Layout, System},
    mem,
    ptr::NonNull,
};

fn main() {
    // allocate a Vec
    let mut a = vec![1, 2, 3, 4, 5];
    println!("# allocate a\na: {:?}\n", &a);

    // deallocate memory
    let ptr = a.as_mut_ptr();
    let layout = Layout::array::<u8>(a.capacity()).unwrap();
    unsafe {
        System.deallocate(NonNull::new(ptr).unwrap(), layout);
    }

    // access memory after deallocation; gives garbage values
    println!("# deallocate a\na: {:?}\n", &a);

    // allocate a String; this will have the same memory address as the Vec had
    let mut b = "Hello, world!".to_string();
    assert_eq!(b.as_mut_ptr(), ptr);
    println!(
        "# allocate b\na: {:?},\nb: {:?} ({:?})\n",
        &a,
        &b.as_bytes(),
        &b,
    );

    // write to the old vec; this will also write into the new String
    for (idx, val) in a.iter_mut().enumerate() {
        *val = 2 * (idx as u8);
    }

    // scary!
    println!(
        "# write to a\na: {:?},\nb: {:?} ({:?})",
        &a,
        &b.as_bytes(),
        &b
    );

    // forget about memory to avoid double free error
    mem::forget(a);
}
