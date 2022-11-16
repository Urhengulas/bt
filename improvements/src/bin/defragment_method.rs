#![feature(allocator_api)]
#![no_main]

use std::{
    alloc::{AllocError, Layout},
    ptr::NonNull,
};

trait Allocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError>;

    /// Defragment the memory.
    ///
    /// The method receives a list of mutable pointers and moves them to compact
    /// the memory.
    fn defragment(&self, a: &[&mut NonNull<[u8]>]);
}
