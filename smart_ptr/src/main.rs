fn main() {
    //trait_life();
    
    //simple_box();

    //rec_no_box();

    //rec_w_box();

    //reg_refer();

    //reg_mut_refer();

    //not_in_stack();

    //custom_deref();

    //deref_coercion();

    //drop_sequeunce();

    //explicit_drop();

    //intro_rc();

    //ref_count();

    //ref_thread();

    rc_refcell();
}

fn trait_life() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result =
        longest_with_an_announcement(string1.as_str(), string2, "Today is someone's birthday!");
    println!("The longest string is {}", result);
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn simple_box() {
    let b = Box::new(5);
    println!("b = {}", b);
}

/*
error[E0072]: recursive type `List` has infinite size
  --> src/main.rs:12:1
   |
12 | enum List {
   | ^^^^^^^^^ recursive type has infinite size
13 |     Cons(i32, List),
   |               ---- recursive without indirection
   |
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to make `List` representable
   |
13 |     Cons(i32, Box<List>),
   |               ^^^^    ^
*/

/* enum List {
    Cons(i32, List),
    Nil,
}

fn rec_no_box() {
    let list = List::Cons(1, List::Cons(2, List::Cons(3, List::Nil)));
} */

/* enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn rec_w_box() {
    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );
} */

fn reg_refer() {
    let x = 5;
    let y = &x;

    println!("x = {} and *y = {}", x, *y);

    /*
    error[E0277]: can't compare `{integer}` with `&{integer}`
      --> src/main.rs:59:5
       |
    59 |     assert_eq!(5, y);
       |     ^^^^^^^^^^^^^^^^^ no implementation for `{integer} == &{integer}`
       |
       = help: the trait `PartialEq<&{integer}>` is not implemented for `{integer}`
    */
    //assert_eq!(5, y);
}

fn reg_mut_refer() {
    let mut x = 5;
    let mut y = &mut x;

    //let mut a = 10;
    //y = &mut a;
    *y = 10;

    println!("*y = {}", *y);
    println!("x = {}", x);
    //println!("x = {} and *y = {}", x, *y);
}

fn not_in_stack() {
    let mut x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    x = 10;

    assert_eq!(10, x);
    assert_eq!(5, *y);
}

struct MyBox<T> {
    x: T,
    y: T,
}

impl<T> MyBox<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.x
    }
}

fn custom_deref() {
    let a = 5;
    let b = 10;
    let y = MyBox::new(a, b);

    assert_eq!(5, a);
    /*
    without imple of Deref:
    error[E0614]: type `MyBox<{integer}>` cannot be dereferenced
       --> src/main.rs:126:19
        |
    126 |     assert_eq!(5, *y);
        |                   ^^
    */
    assert_eq!(5, *y);
    //*(y.deref())
}

fn hello(name: &str) {
//fn hello(name: &mut str) {
    println!("Hello, {}!", name);
}

//deref coercion
fn deref_coercion() {
    let m = MyBox::new(String::from("Rust"), String::from("C/C++"));

    hello(&(*m)[..]);
    //hello(&m);

    //let mut m = MyBox::new(String::from("Rust"), String::from("C/C++"));
    //hello(&mut m);
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn drop_sequeunce() {
    let c = CustomSmartPointer {
        data: String::from("c instance"),
    };
    let d = CustomSmartPointer {
        data: String::from("d instance"),
    };
    println!("CustomSmartPointers created.");
}

fn explicit_drop() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    //c.drop();
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}

enum List2 {
    Cons(i32, Box<List2>),
    Nil,
}

use std::rc::Rc;

enum List3 {
    Cons(i32, Rc<List3>),
    Nil,
}

fn intro_rc() {
    /* let a = List2::Cons(5, Box::new(List2::Cons(10, Box::new(List2::Nil))));
    let b = List2::Cons(3, Box::new(a));
    let c = List2::Cons(4, Box::new(a)); */

    let a = Rc::new(List3::Cons(
        5,
        Rc::new(List3::Cons(10, Rc::new(List3::Nil))),
    ));
    let b = List3::Cons(3, Rc::clone(&a));
    let c = List3::Cons(4, Rc::clone(&a));
}

fn ref_count() {
    let a = Rc::new(List3::Cons(
        5,
        Rc::new(List3::Cons(10, Rc::new(List3::Nil))),
    ));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = List3::Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = List3::Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

use std::thread;

fn ref_thread(){
    let a = Rc::new(List3::Cons(
        5,
        Rc::new(List3::Cons(10, Rc::new(List3::Nil))),
    ));
    let b = Box::new(5);

    let handle = thread::spawn(move || {
        //println!("count after creating a = {}", Rc::strong_count(&a));
        println!("b = {}", b);
    });

    handle.join();
}

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;

fn rc_refcell() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    println!("a before = {:?}", a);
    println!("b before = {:?}", b);
    println!("c before = {:?}", c);

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            let mut one_borrow = self.sent_messages.borrow_mut();
            //let mut two_borrow = self.sent_messages.borrow_mut();

            one_borrow.push(String::from(message));
            //two_borrow.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}