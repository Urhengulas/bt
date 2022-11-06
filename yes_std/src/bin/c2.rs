fn main() {
    let a = Box::new(42);
    Box::leak(a);

    let b = Box::new([0, 1, 2, 3, 4]);
    core::mem::forget(b);
}
