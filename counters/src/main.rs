use std::cell::RefCell;
use std::rc::Rc;

type Counter = Box<dyn FnMut() -> i32>;

fn counters() -> (Counter, Counter) {
    let cnt = Rc::new(RefCell::new(0));
    let cnt1 = Rc::clone(&cnt);
    let cnt2 = Rc::clone(&cnt);

    let counter1 = move || -> i32 {
        *cnt1.borrow_mut() += 1;
        *cnt1.borrow_mut()
    };

    let counter100 = move || -> i32 {
        *cnt2.borrow_mut() += 100;
        *cnt2.borrow_mut()
    };

    (Box::new(counter1), Box::new(counter100))
}

fn main() {
    let (mut c1, mut c100) = counters();

    println!("{}", c1());
    println!("{}", c1());
    println!("{}", c100());
    println!("{}", c100());
    println!("{}", c1());
    println!("{}", c100());
}

// 1
// 2
// 102
// 202
// 203
// 303
