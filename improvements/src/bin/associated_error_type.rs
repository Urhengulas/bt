#![no_main]

use std::{alloc::Layout, ptr::NonNull};

trait Allocator {
    type Error;
    fn allocate(layout: Layout) -> Result<NonNull<[u8]>, Self::Error>;
}
