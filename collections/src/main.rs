fn main() {
    // Code 1
    //println!("Code 1 output:");
    //code1();

    // Code 2
    // println!("Code 2 output:");
    // code2();

    // Code 3
    //println!("Code 3 output:");
    //code3();

    // Code 4
    //println!("Code 4 output:");
    //code4();

    // code 8
    //println!("Code 8 output:");
    //code8();

    // code 9
    //println!("Code 9 output:");
    //code9();

    // code 10
    //println!("Code 10 output:");
    //code10();

    // code 11
    //println!("Code 11 output:");
    //code11();

    // code 12
    println!("Code 12 output:");
    code12();
}

use std::io;

fn code1() {
    let v = vec![1, 2, 3, 4, 5];
    //v.push(6);

    println!("The vector is {:?}", v);

    let first = &v[0];
    println!("The first element is: {}", first);

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("The vector is {:?}", v);

    /* let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let u_int: usize = input.trim().parse().expect("Please type a number!");

    let user_choice = &v[u_int];
    println!("The user choice element is: {}", user_choice); */

    //code2e();
}

fn code2() {
    let v = vec![1, 2, 3, 4, 5];

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let u_int: usize = input.trim().parse().expect("Please type a number!");

    match v.get(u_int) {
        Some(item) => println!("The third element is {}", item),
        None => println!("There is no item element."),
    }
}

fn code2e() {
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    //v.push(6);
    println!("Selected element is: {}", first);
}

fn code3() {
    let mut v = vec![100, 32, 57];
    println!("Before: {:?}", v);
    for i in &mut v {
        *i += 50;
    }

    println!("After: {:?}", v);
}

fn code4() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("Enum Vector: {:?}", row);
}

fn code8() {
    let s1 = String::from("Hello");
    let s2 = String::from("नमस्ते");
    let s3 = String::from("こんにちは");
    let s4 = String::from("안녕하세요");
    let s5 = String::from("你好");
    let s6 = String::from("Olá");
    println!("{} {} {} {} {} {}", s1, s2, s3, s4, s5, s6);
}

fn code9() {
    let mut s1 = String::from("foo");
    let s2 = "_bar";
    s1.push_str(s2);
    println!("s1 is {}", s1);
    //let s3 = String::from("_done");
    //s1.push_str(s3);
    //println!("s1 is {}", s1);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("s3 is {}", s3);
    //println!("s1 was {}", s1);
    //println!("s2 was {}", s2);
}

fn code10() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("Using + operator: {}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("Using format macro operator: {}", s);
}

fn code11() {
    /* let hello = String::from("Здравствуйте");
    let hello = String::from("hello");
    let answer = &hello[0];
    println!("The first character is {}", answer); */

    //let hello = String::from("hello");
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("The first four bytes of the String are {}", s);

    let hello = "Здравствуйте";
    for c in hello.chars() {
        print!("{} ", c);
    }
    println!();

    for b in hello.bytes() {
        print!("{} ", b);
    }
    println!();

    let mut chars = hello.chars();
    let index: usize = 4;
    let item = chars.nth(index);
    println!("{}", item.unwrap());
}

use std::collections::HashMap;

fn code12() {
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");
    let val10 = 10;
    let val50 = 50;

    {
        let mut scores = HashMap::new();
        scores.insert(&blue, val10);
        scores.insert(&yellow, val50);

        println!("Example 1: {:#?}", scores);
    }

    println!("The keys are {} and {}", blue, yellow);
    //println!("The keys are {} and {}", val10, val50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("Example 2: {:#?}", scores);
    //println!("{:?}", teams);
    //println!("{:?}", initial_scores);

    let blue = String::from("Blue");
    let score = scores.get(&blue);

    println!("{} team score {}", blue, score.unwrap());

    scores.insert(String::from("Blue"), 25);

    scores.entry(String::from("Red")).or_insert(100);

    println!("Iterating scores: ");
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    println!("Example 3: {:#?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
