fn main() {
    // string literal are immutable
    let s1: &str = "string literals";
    // but the pointer refers to string literals can be mutable
    let mut s: &str = s1;
    println!("{}", s);
    // so you can change a mutable pointer where refer to
    s = "pointer refer to a different location";
    // but you cannot change the literal itself
    // s1 = "Try";
    println!("{} {}", s1, s);

    // you cannot store a literal to a String type
    // let src: String = "hello";

    let s: String = String::from("string literal to String");
    //s.push_str(", world!"); // without mut String, cannot borrow it
    println!("{}", s);

    let mut s: String = String::from("mutable string");
    s.push_str(", aha!"); // push_str() appends a literal to a String
    println!("{}", s);

    {
        let s = String::from("limited scope by parentheses"); // s is valid from this point forward

        // do stuff with s
        println!("{}", s);
        // implicit call to <_ZN4core3ptr13drop_in_place17h2e6e11db72490ffcE>
    } // this scope is now over, and s is no
      // longer valid

    println!("{}", s);

    let s1 = String::from("source string");
    let s2 = s1;

    /*
    error[E0382]: borrow of moved value: `s1`
      --> src/main.rs:38:20
       |
    35 |     let s1 = String::from("source string");
       |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
    36 |     let s2 = s1;
       |              -- value moved here
    37 |
    38 |     println!("{}", s1);
       |                    ^^ value borrowed here after move
    */

    // println!("{}", s1);
    println!("{}", s2);

    let mut m1 = String::from("lets try mut string");
    let mut m2 = m1;

    /*
    54 |     let mut m1 = String::from("lets try mut string");
       |         ----^^
       |         |
       |         help: remove this `mut`
       |
       = note: `#[warn(unused_mut)]` on by default

    warning: variable does not need to be mutable
      --> src/main.rs:55:9
       |
    55 |     let mut m2 = m1;
       |         ----^^
       |         |
       |         help: remove this `mut`

    error[E0382]: borrow of moved value: `m1`
      --> src/main.rs:56:20
       |
    54 |     let mut m1 = String::from("lets try mut string");
       |         ------ move occurs because `m1` has type `String`, which does not implement the `Copy` trait
    55 |     let mut m2 = m1;
       |                  -- value moved here
    56 |     println!("{}", m1);
       |                    ^^ value borrowed here after move
        */
    // println!("{}", m1);
    println!("{}", m2);

    let s1 = String::from("lets clone");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}
