use std::{cell::RefCell, ops::Deref, rc::Rc};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<Self>>),
    Nil,
}

impl List {
    fn cons(n: i32, list: Rc<Self>) -> Rc<Self> {
        Rc::new(Self::Cons(n, RefCell::new(list)))
    }
}

fn main() {
    let a = List::cons(1, Rc::new(List::Nil));
    let b = List::cons(2, Rc::clone(&a));

    match a.deref() {
        List::Cons(_, c) => *c.borrow_mut() = Rc::clone(&b),
        List::Nil => (),
    }

    println!(
        "rc count: a={}, b={}",
        Rc::strong_count(&a),
        Rc::strong_count(&b)
    );

    // WARN: the next line creates a stack overflow
    println!("{a:?}");
}
