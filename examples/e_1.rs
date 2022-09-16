fn main() {
    let a = "Hello, world!".to_string();
    std::mem::drop(a);
    println!("{a}");
}