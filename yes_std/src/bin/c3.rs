use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
enum List {
    Cons(RefCell<Rc<Self>>),
    Nil,
}

impl List {
    fn cons(list: Rc<Self>) -> Rc<Self> {
        Rc::new(Self::Cons(RefCell::new(list)))
    }

    fn tail(&self) -> Option<&RefCell<Rc<Self>>> {
        match self {
            Self::Cons(item) => Some(item),
            Self::Nil => None,
        }
    }
}

fn main() {
    let a = List::cons(Rc::new(List::Nil));
    let b = List::cons(Rc::clone(&a));
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!(
        "rc count: a={}, b={}",
        Rc::strong_count(&a),
        Rc::strong_count(&b)
    );

    println!("a next item = {:?}", a.tail());
}
