#![feature(allocator_api)]

use std::{
    alloc::{Allocator, Global, Layout},
    mem,
    ptr::NonNull,
};

fn main() {
    // allocate Vec A
    println!("# allocate A");
    let mut a = (0..5).into_iter().map(|i| Dropper(i)).collect::<Vec<_>>();
    println!("A: {:?}\n", &a);

    // deallocate memory of A, but keep raw pointer
    println!("# deallocate A");
    let ptr = a.as_mut_ptr().cast();
    let layout = Layout::array::<Dropper>(a.capacity()).unwrap();
    unsafe { Global.deallocate(NonNull::new(ptr).unwrap(), layout) };

    // access memory of A after deallocation; gives garbage values
    println!("A: {:?}\n", &a);

    // allocate String B; this might (and in our case will) use the same memory address as A
    println!("# allocate B");
    let mut b = "Hello, world!".to_string();
    assert_eq!(ptr, b.as_mut_ptr());
    println!("B: {:?} ({:?})\nA: {:?}\n", &b.as_bytes(), &b, &a);

    // write to the old vec; this will also write into the new String
    println!("# write to A");
    for (idx, val) in a.iter_mut().enumerate() {
        val.0 = 2 * idx as u8;
    }

    // scary!
    println!("B: {:?} ({:?})\nA: {:?}\n", &b.as_bytes(), &b, &a);
}

#[derive(Debug)]
struct Dropper(u8);

impl Drop for Dropper {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
    }
}
