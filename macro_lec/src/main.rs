#[macro_export]
macro_rules! mm_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

/* use tr_macro::HelloMacro;

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
} */

use tr_macro::HelloMacro;
use tr_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Waffle;

fn main() {
    let v = mm_vec![1, 2, 3];
    //let v = mm_vec!(1, 2, 3]);
    //let v = mm_vec![];
    //println!("our vector: {:?}", v);

    //Pancakes::hello_macro();
    Waffle::hello_macro();
}

/* fn act_mm_vec() -> Vec<i32> {
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
} */
