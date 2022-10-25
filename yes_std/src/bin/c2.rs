#![feature(allocator_api)]

use std::mem;

fn main() {
    let a = vec![1, 2, 3, 4, 5];
    mem::forget(a);
}
