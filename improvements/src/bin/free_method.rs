#![no_main]

trait Allocator {
    fn free(&self) -> usize;
}
