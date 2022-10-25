#![feature(allocator_api)]

use linked_list_allocator::LockedHeap;

static ALLOCATOR: LockedHeap = LockedHeap::empty();
static mut HEAP: [u8; 64] = [0; 64];

fn main() {
    init_heap();
    dbg!(ALLOCATOR.lock().free());

    // allocate four small objects
    let _a = Box::try_new_in([5_u8; 16], &ALLOCATOR).unwrap();
    let _b = Box::try_new_in([5_u8; 16], &ALLOCATOR).unwrap();
    let _c = Box::try_new_in([5_u8; 16], &ALLOCATOR).unwrap();
    let _d = Box::try_new_in([5_u8; 16], &ALLOCATOR).unwrap();
    dbg!(ALLOCATOR.lock().free());

    // deallocate two small objects; they are _not_ consecutive
    drop(_a);
    drop(_c);
    dbg!(ALLOCATOR.lock().free());

    // allocating one big object fails, although there is theoretically enough memory available
    let _e = Box::try_new_in([5_u8; 32], &ALLOCATOR).unwrap_err();

    // allocating two small objects succeeds; combined they have the same size as the big object
    let _f = Box::try_new_in([5_u8; 16], &ALLOCATOR).unwrap();
    let _g = Box::try_new_in([5_u8; 16], &ALLOCATOR).unwrap();
}

pub fn init_heap() {
    unsafe {
        let heap_start = HEAP.as_mut_ptr();
        let heap_size = HEAP.len();
        ALLOCATOR.lock().init(heap_start, heap_size);
    };
    println!("Initialized heap!");
}
