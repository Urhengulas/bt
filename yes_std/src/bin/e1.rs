fn main() {
    let a = "Hello, world!".to_string();
    std::mem::drop(a);
    // println!("{a}"); // error[E0382]: borrow of moved value: `a`
}
