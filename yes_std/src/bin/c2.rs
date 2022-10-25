#![feature(allocator_api)]

use std::mem;

fn main() {
    let a = vec![1, 2, 3, 4, 5];
    mem::forget(a);

    let b = Box::new(5);
    Box::leak(b);
}
