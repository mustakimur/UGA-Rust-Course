fn main() {
    code1();

    //code2();

    //code3();

    //code4();

    //code5();

    //code6();

    //code7();

    //code8();

    //code9();
}

fn code1() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    let address = 0x012345usize;
    let r3 = address as *const i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        //println!("r2 is: {}", *r3);

        //*r1 = 10;
        *r2 = 20;

        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

unsafe fn dangerous(r1: *const i32, r2: *mut i32) {
    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);

    *r2 = 30;

    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);
}

fn code2() {
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
}

fn code3() {
    let mut vector = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = split_at_mut(&mut vector, 3);
} */

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

fn code4() {
    let mut vector = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = un_bl_split_at_mut(&mut vector, 3);
}

extern "C" {
    fn abs(input: i32) -> i32;
}

fn code5() {
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

fn code6() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn code7() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
}

fn code8() {
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

fn code9() {
    let cls = returns_closure();
    println!("return: {}", cls(3));
}
