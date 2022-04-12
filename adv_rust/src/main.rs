fn main() {
    //raw_pointer();

    //calling_unsafe();

    //fail_to_multi_mut();

    //unsafe_split_multi_mut();

    //call_to_c();

    //mutate_static();

    //unsafe_trait();

    //unsafe_union();

    //fn_pointer_pass();

    //not_only_function();

    //return_closure();

    //associate_types();

    //op_over_add();

    //op_over_add_diff();

    //method_same_name();

    //associate_same_name();

    //trait_to_trait();

    //new_type();

    //never_return();

    unknown_size();

    //mutex_sample();

    //mutex_share_move_lock();

    //mutex_share_unsafe_rc();

    //mutex_share_safe_rc();
}

fn raw_pointer() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    let address = 0x012345usize;
    let r3 = address as *const i32;

    unsafe {
        //println!("r1 is: {}", *r1);
        //println!("r2 is: {}", *r2);
        println!("r2 is: {}", *r3);

        //*r1 = 10;
        //*r2 = 20;

        //println!("r1 is: {}", *r1);
        //println!("r2 is: {}", *r2);
    }
}

unsafe fn dangerous(r1: *const i32, r2: *mut i32) {
    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);

    *r2 = 30;

    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);
}

fn calling_unsafe() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    unsafe {
        dangerous(r1, r2);
    }
}

/*
error[E0499]: cannot borrow `*slice` as mutable more than once at a time
  --> src/main.rs:56:30
   |
51 | fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
   |                        - let's call the lifetime of this reference `'1`
...
56 |     (&mut slice[..mid], &mut slice[mid..])
   |     -------------------------^^^^^--------
   |     |     |                  |
   |     |     |                  second mutable borrow occurs here
   |     |     first mutable borrow occurs here
   |     returning this value requires that `*slice` is borrowed for `'1`
*/

/* fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();

    assert!(mid <= len);

    (&mut slice[..mid], &mut slice[mid..])
} */

fn fail_to_multi_mut() {
    let mut vector = vec![1, 2, 3, 4, 5, 6];
    //let (left, right) = split_at_mut(&mut vector, 3);
}

use std::slice;

fn un_bl_split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn unsafe_split_multi_mut() {
    let mut vector = vec![1, 2, 3, 4, 5, 6];
    println!("original (before): {:?}", vector);
    let (left, right) = un_bl_split_at_mut(&mut vector, 3);
    left[0] = 10;
    right[0] = 40;
    println!("left: {:?}, right: {:?}", left, right);
    println!("original (after): {:?}", vector);
}

extern "C" {
    fn abs(input: i32) -> i32;
}

fn call_to_c() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn mutate_static() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

use std::cmp::Ordering;

struct Demo {
    text: String,
}

unsafe trait UnsafeOrd {
    fn cmp(&self, other: &Self) -> Ordering;
}

unsafe impl UnsafeOrd for Demo {
    fn cmp(&self, other: &Self) -> Ordering {
        if (self.text == other.text) {
            Ordering::Equal
        } else if (self.text > other.text) {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

fn unsafe_trait() {
    let d = Demo {
        text: String::from("hello"),
    };
    let o = Demo {
        text: String::from("world"),
    };
    println!("Result: {:?}", d.cmp(&o));
}

union MyUnion {
    f1: u32,
    f2: f32,
}

fn unsafe_union() {
    let u = MyUnion { f2: 1.0 };
    let f = unsafe { u.f1 };
    println!("{}", f);
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn add_two(x: i32) -> i32 {
    x + 2
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn fn_pointer_pass() {
    let answer = do_twice(add_two, 5);

    println!("The answer is: {}", answer);
}

fn not_only_function() {
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
}

/* fn returns_closure() -> dyn Fn(i32) -> i32 {
    |x| x + 1
} */

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn return_closure() {
    let cls = returns_closure();
    println!("return: {}", cls(3));
}

/* pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
} */

pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn associate_types() {
    let mut c = Counter::new();
    println!("{}", c.next().unwrap());
}

/* trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
} */

use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn op_over_add() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

fn op_over_add_diff() {
    println!("{}", (Millimeters(10) + Meters(1)).0);
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

/* impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
} */

fn method_same_name() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    //person.fly();
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn associate_same_name() {
    println!("A baby dog is called a {}", Dog::baby_name());
    //println!("A baby dog is called a {}", Animal::baby_name());
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}

use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct PointXY {
    x: i32,
    y: i32,
}

impl fmt::Display for PointXY {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(X: {}, Y: {})", self.x, self.y)
    }
}

impl OutlinePrint for PointXY {}

fn trait_to_trait() {
    let p = PointXY { x: 1, y: 3 };
    p.outline_print();
}

type Thunk = Box<dyn Fn() + Send + 'static>;

fn takes_long_type(f: Thunk) {
    f();
}

fn returns_long_type() -> Thunk {
    Box::new(|| println!(".world!"))
}

fn new_type() {
    let f: Thunk = Box::new(|| println!("hello."));
    takes_long_type(f);
    let ff = returns_long_type();
    ff();
}

fn never_fn() -> ! {
    /* let x: ! = { 123 };
    x */
    panic!("lets panic");
}

fn never_return() {
    //never_fn();

    let mut value = None;
    /* loop {
        let guess: u32 = match value {
            Some(num) => num,
            None => "hello",
        };
    } */

    println!("before");
    let mut guess: u32;
    loop {
        guess = match value {
            Some(num) => num,
            None => continue,
        };
        println!("Guess: {}", guess);
        value = Some(50);
    }
    println!("outside guess {}", guess);

    /* loop {
        let guess: u32 = match value {
            Some(num) => num,
            None => panic!("lets panic"),
        };
    } */

    /* loop {
        let guess: u32 = match value {
            Some(num) => num,
            None => println!("just print"),
        };
    } */
}

fn unknown_size() {
    fn generic1<T>(t: T) {
        // --snip--
    }
    fn generic2<T: Sized>(t: T) {
        // --snip--
    }
    fn generic3<T: ?Sized>(t: &T) {
        // --snip--
    }

    struct Foo<T>(T);
    //struct FooUse(Foo<[i32]>);

    struct Bar<T: ?Sized>(T);
    struct BarUse(Bar<[i32]>);
}

use std::sync::Mutex;

fn mutex_sample() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);

    let mut num = m.lock().unwrap();
    *num = 10;
    println!("m = {:?}", m);
}

use std::thread;

fn mutex_share_move_lock() {
    /* let counter = Mutex::new(0);
    let mut handles = vec![];

    for _ in 0..10 {
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap()); */
}

use std::rc::Rc;

fn mutex_share_unsafe_rc() {
    /* let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap()); */
}

use std::sync::Arc;

fn mutex_share_safe_rc() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
