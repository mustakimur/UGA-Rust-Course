/* fn main() {
    let mut a: i32 = 0;
    a = a + 1;
    println!("{}", a);
} */

/* fn main() {
    let x = 5;
    let x: i32 = 5; //type annotation
    let mut x = 5; //mutable x: i32
    x = 10;
} */

/* fn main() {
    let x = (let y = 6);
} */

/* fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
} */

/* fn main() {
    another_function(5, 6);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
} */

/* fn main() {
    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
} */

/* fn main() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
} */

/* fn main(){
    let mut x = 10u32;
    x = -10;
    let y = 1_000_000;
    let z = 2_00 + y;
    println!("{0} == {0}", z);
    let b = 0b1010;
    println!("{} = {:b}", b, b);
    let ascii = b'A';
    println!("{}", ascii);
    let f:f64 = -3.1416;
    println!("{}", f);
} */

/* fn main() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
} */

/* fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
} */

/* fn main(){
    let mut tt: (char, i32, f32) = ('A', 65, 65.0);
    tt.0 = 'C';
    println!("{:?}", tt);
} */

/* fn main() {
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
} */

/* fn main() {
    let a = [1, 2, 3, 4, 5];
    let index = 10;

    let element = a[index];

    println!("The value of element is: {}", element);
} */

/* fn main(){
    let arr: [i32; 5] = [1,2,3,4,5];
    for n in 1..=10{
        println!("{}", arr[n]);
    }
} */

fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

/* fn main() {
    let number = 3;

    if number {
        println!("number was three");
    }
} */

/* fn main() {
    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }
} */

/* fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
} */

/* fn main() {
    let condition = true;

    let number = if condition { 5 } else { "six" };

    println!("The value of number is: {}", number);
} */

// type casting
/* fn main() {
    let decimal: f32 = 65.4321;
    //let integer: u32 = decimal;

    let integer: u32 = decimal as u32;
    println!("{} => {}", decimal, integer);

    //let character: char = integer as char;
    let short_integer: u8 = integer as u8;
    let character: char = short_integer as char;
    println!("{} => {} => {}", integer, short_integer, character);

    let s_integer: i32 = -10;
    let integer: u32 = s_integer as u32;
    println!("{} => {}", s_integer, integer);
} */

/* fn main() {
    let sinteger = -10;
    let uinteger:u32 = 25;

    if sinteger == uinteger {
        println!("it's a match");
    } else {
        println!("{} != {}", sinteger, uinteger);
    }
} */

// aliasing
/* fn main() {
    type NanoSecond = u64;
    type Inch = u64;

    let nanoseconds: NanoSecond = 5;
    let inches: Inch = 2;

    println!(
        "{} nanoseconds + {} inches = {} unit?",
        nanoseconds,
        inches,
        nanoseconds + inches
    );
} */

/* use std::io;

fn fact(n: i32) -> i32 {
    if n == 0 {
        1
    } else {
        let x = fact(n - 1);
        n * x
    }
}

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let u_int: i32 = input.trim().parse().expect("Please type a number!");
    //let u_int = input.trim().parse::<i32>().expect("Please type a number!");

    let res = fact(u_int);
    println!("fact({}) = {}", u_int, res);
} */

/* fn main() {
    loop {
        println!("again!");
    }
} */

/* #[macro_use]
extern crate scan_rules;

fn main() {
    let result = try_readln! {
        (let n: u32, let m: u32) => (n, m)
    };
    match result {
        Ok((n, m)) => println!("I read n={}, m={}", n, m),
        Err(e) => println!("Failed to parse input: {}", e),
    }
} */

/* fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
} */

/* fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
} */

/* fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
} */

/* fn main() {
    for x in (1..10).step_by(2) {
        println!("{}", x);
    }
} */
