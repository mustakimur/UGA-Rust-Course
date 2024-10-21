macro_rules! a_macro {
    () => {
        let a = 10;
        println!("a macro print {}", a);
    };
}

fn simple_macro() {
    a_macro!();
}

macro_rules! x_and_y {
    (x => $e:expr) => {
        println!("X: {}", $e)
    };
    (y => $e:expr) => {
        println!("Y: {}", $e)
    };
}

fn expr_macro() {
    x_and_y!(x => 10);
    x_and_y!(y => 20 + 30);
}

macro_rules! build_fn {
    ($func_name:ident) => {
        fn $func_name() {
            println!("You called {:?}()", stringify!($func_name));
        }
    };
}

fn create_fn() {
    build_fn!(Hi_there);
    Hi_there();
}

macro_rules! print_ex {
    ($e: expr) => {
        println!("{:?} = {:?}", stringify!($e), $e)
    };
}

fn create_ex() {
    print_ex!({
        let x = 20;
        let y = 30;
        (x + y) * 10
    });
}

macro_rules! exame {
    ($l: expr; and $r: expr) => {
        println!(
            "{:?} && {:?} = {:?}",
            stringify!($l),
            stringify!($r),
            $l && $r
        );
    };

    ($l: expr; or $r: expr) => {
        println!(
            "{:?} || {:?} = {:?}",
            stringify!($l),
            stringify!($r),
            $l || $r
        );
    };
}

fn run_exame() {
    exame!(1==1; and 2==1+1);
    exame!(true; or false);
}

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

/* fn act_mm_vec() -> Vec<i32> {
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
} */

fn custom_vec() {
    let v = mm_vec![1, 2, 3];
    //let v = mm_vec!([1, 2, 3]);
    //let v = mm_vec![];
    println!("our vector: {:?}", v);
}

use tr_macro::HelloMacro;

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

use tr_macro_derive::{do_copy, sql, HelloMacro};

#[derive(HelloMacro)]
struct Waffle;

fn proc_macro_ex() {
    Pancakes::hello_macro();
    Waffle::hello_macro();
}

#[do_copy]
fn hello_world() {
    println!("Hello, world!");
}

fn main() {
    //simple_macro();

    //expr_macro();

    //create_fn();

    //create_ex();

    //run_exame();

    //custom_vec();

    //proc_macro_ex();

    hello_world();

    //let sql = sql!("SELECT * FROM posts WHERE id=1");
    //println!("{}", sql);
}
