#![feature(allocator_api)]

use std::{
    alloc::{AllocError, Allocator, Layout, System},
    ops::Range,
    ptr::NonNull,
};

static MY_ALLOC: MyAllocator = MyAllocator;

struct MyAllocator;

unsafe impl Allocator for MyAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        System.allocate(layout)
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        System.deallocate(ptr, layout)
    }
}

fn main() {
    let mut a = Vec::with_capacity_in(3, &MY_ALLOC);
    (0..3).for_each(|i| a.push(i));
    let a_ptr_range = a.as_ptr_range();
    dbg!(&a, &a_ptr_range);

    let mut b = Vec::with_capacity_in(3, &MY_ALLOC);
    (0..3).for_each(|i| b.push(i * 2));
    let b_ptr_range = b.as_ptr_range();
    dbg!(&b, &b_ptr_range);

    assert!(!overlaps(a_ptr_range, b_ptr_range), "should not overlap");
}

fn overlaps(a: Range<*const i32>, b: Range<*const i32>) -> bool {
    let a_end = unsafe { a.clone().end.offset(-1) };
    let b_end = unsafe { b.clone().end.offset(-1) };
    a.contains(&b.start) || a.contains(&b_end) || b.contains(&a.start) || b.contains(&a_end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overlaps() {
        let a = 0 as *const _;
        let b = 5 as *const _;
        let c = 10 as *const _;
        let d = 3 as *const _;
        assert!(!overlaps(a..b, b..c));
        assert!(overlaps(a..b, d..c));
    }
}
