#![feature(allocator_api)]
#![no_main]

use std::{
    alloc::{AllocError, Layout},
    ptr::NonNull,
    time::Duration,
};

trait Allocator {
    fn allocate(layout: Layout, timeout: Duration) -> Result<NonNull<[u8]>, AllocError>;
    fn deallocate(ptr: NonNull<u8>, layout: Layout, timeout: Duration) -> Result<(), AllocError>;
}
