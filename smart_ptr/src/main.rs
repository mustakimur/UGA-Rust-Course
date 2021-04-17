fn main() {
    //code1();

    //code2();

    //code3();

    //code4();

    //code5();

    //code6();

    //code7();

    //code8();

    //code9();

    //code10();

    //code11();

    code12();
}

fn code1() {
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
} */

fn code2() {
    //let list = List::Cons(1, List::Cons(2, List::Cons(3, List::Nil)));
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn code3() {
    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );
}

fn code4() {
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

fn code5() {
    let mut x = 5;
    let mut y = &mut x;

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

    /* let mut a = 10;
    y = &mut a; */
    *y = 10;

    println!("*y = {}", *y);
    println!("x = {}", x);
    //println!("x = {} and *y = {}", x, *y);
}

fn code6() {
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
        //&self.x
        &self.y
    }
}

fn code7() {
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
    println!("Hello, {}!", name);
}

//deref coercion
fn code8() {
    let m = MyBox::new(String::from("Rust"), String::from("C/C++"));

    hello(&(*m)[..]);
    //hello(&m);
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn code9() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}

fn code10() {
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

enum List3 {
    Cons(i32, Rc<List3>),
    Nil,
}

use std::rc::Rc;

fn code11() {
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

fn code12() {
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
