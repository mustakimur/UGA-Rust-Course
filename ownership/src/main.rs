fn main() {
   //Code 1
   /*
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
   */

   // Code 2
   /*
      // you cannot store a literal to a String type
      // let src: String = "hello";

      let s: String = String::from("string literal to String");
      //s.push_str(", world!"); // without mut String, cannot borrow it
      println!("{}", s);
   */

   // Code 3
   /*
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
   */

   // Code 4
   /*
   let s1 = String::from("source string");
   let s2 = s1;

   /********
   * error[E0382]: borrow of moved value: `s1`
   *  --> src/main.rs:38:20
   *   |
   * 35 |     let s1 = String::from("source string");
   *    |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
   * 36 |     let s2 = s1;
   *    |              -- value moved here
   * 37 |
   * 38 |     println!("{}", s1);
   *    |                    ^^ value borrowed here after move
   *********/

   // println!("{}", s1);
   println!("{}", s2);
   */

   // Code 5
   /*
   let mut m1 = String::from("lets try mut string");
   let mut m2 = m1;

   /******
   * 54 |     let mut m1 = String::from("lets try mut string");
   *   |         ----^^
   *   |         |
   *   |         help: remove this `mut`
   *   |
   *   = note: `#[warn(unused_mut)]` on by default

   *warning: variable does not need to be mutable
   *  --> src/main.rs:55:9
   *   |
   *55 |     let mut m2 = m1;
   *   |         ----^^
   *   |         |
   *   |         help: remove this `mut`

   *error[E0382]: borrow of moved value: `m1`
   *  --> src/main.rs:56:20
   *   |
   *54 |     let mut m1 = String::from("lets try mut string");
   *   |         ------ move occurs because `m1` has type `String`, which does not implement the `Copy` trait
   *55 |     let mut m2 = m1;
   *   |                  -- value moved here
   *56 |     println!("{}", m1);
   *   |                    ^^ value borrowed here after move
       ******/
   // println!("{}", m1);
   println!("{}", m2);
   */

   // Code 6
   /*
   let s1 = String::from("lets clone");
   let s2 = s1.clone();

   println!("s1 = {}, s2 = {}", s1, s2);
   */

   // Code 7
   /*
   let x = 5;
   let y = x;
   println!("{} => {}", x, y);
   */

   // Code 8
   /*
      let x = 15;
      let y = &x;
      println!("{} => {}", x, y);
   */

   // Code 9 [discuss]
   /*
   let gv = String::from("source string");
   {
      let lv = gv;

      println!("{}", gv);
      println!("{}", lv);
   }

   println!("{}", gv);
   println!("{}", lv);
   */

   //Code 10 [discuss]
   /*
      let mut x = 5;
      let y = x;
      println!("{} => {}", x, y);
      x = 10;
      println!("{} => {}", x, y);
   */

   // Code 11 [lecture, future, one mutable reference, no immutable after]
   /*
   let mut x = 15;
   let y = &x;
   println!("{} => {}", x, y);
   /******
    * error[E0506]: cannot assign to `x` because it is borrowed
    *    --> src/main.rs:112:7
    *     |
    * 110 |       let y = &x;
    *     |               -- borrow of `x` occurs here
    * 111 |       println!("{} => {}", x, y);
    * 112 |       x = 20;
    *    |       ^^^^^^ assignment to borrowed `x` occurs here
    * 113 |       println!("{} => {}", x, y);
    *     |                               - borrow later used here
    ******/
   //x = 20;
   println!("{} => {}", x, y);
   */

   // Code 12
   // code12();

   // Code 13
   //code13();

   // Code 14
   // code14();

   // Code 15
   // code15();

   // Code 16
   // code16();

   // Code 17
   // code17();

   // Code 18
   // code18();

   // Code 19
   // code19();

   // Code 20
   // code20();

   // code exercise
   // swap_example();

   // code slices
   code21();
}

// Code 12 [lecture]
fn code12() {
   let s = String::from("hello"); // s comes into scope

   takes_ownership(s); // s's value moves into the function...
                       // ... and so is no longer valid here

   /*******
   error[E0382]: borrow of moved value: `s`
      --> src/main.rs:176:19
       |
   171 |    let s = String::from("hello"); // s comes into scope
       |        - move occurs because `s` has type `String`, which does not implement the `Copy` trait
   172 |
   173 |    takes_ownership(s); // s's value moves into the function...
       |                    - value moved here
   ...
   176 |    println!("{}", s);
       |                   ^ value borrowed here after move
   ********/
   //println!("{}", s);

   let x = 5; // x comes into scope

   makes_copy(x); // x would move into the function,
                  // but i32 is Copy, so it’s okay to still
                  // use x afterward
   println!("{}", x);
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) {
   // some_string comes into scope
   println!("{}", some_string);

   /******
   error[E0596]: cannot borrow `some_string` as mutable, as it is not declared as mutable
      --> src/main.rs:203:4
       |
   199 | fn takes_ownership(some_string: String) {
       |                    ----------- help: consider changing this to be mutable: `mut some_string`
   ...
   203 |    some_string.push_str("world");
       |    ^^^^^^^^^^^ cannot borrow as mutable
   ******/

   // some_string.push_str("world");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
   // some_integer comes into scope
   println!("{}", some_integer);
   /*******
      error[E0384]: cannot assign to immutable argument `some_integer`
      --> src/main.rs:222:4
       |
   219 | fn makes_copy(some_integer: i32) {
       |               ------------ help: make this binding mutable: `mut some_integer`
   ...
   222 |    some_integer += 10;
       |    ^^^^^^^^^^^^^^^^^^ cannot assign to immutable argument
   ********/
   //some_integer += 10;
} // Here, some_integer goes out of scope. Nothing special happens.

// Code 13 [lecture]
fn code13() {
   let s1 = gives_ownership(); // gives_ownership moves its return
                               // value into s1

   println!("{}", s1);

   let s2 = String::from("world"); // s2 comes into scope

   let s2 = takes_and_gives_back(s2); // s2 is moved into
                                      // takes_and_gives_back, which also
                                      // moves its return value into s3
   println!("{}", s2);
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {
   // gives_ownership will move its
   // return value into the function
   // that calls it

   let some_string = String::from("hello"); // some_string comes into scope

   some_string // some_string is returned and
               // moves out to the calling
               // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String {
   // a_string comes into
   // scope

   a_string // a_string is returned and moves out to the calling function
}

// Code 14 [lecture]
fn code14() {
   let s1 = String::from("hello");

   let (s1, len) = calculate_length1(s1);

   println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length1(s: String) -> (String, usize) {
   let length = s.len(); // len() returns the length of a String

   (s, length)
}

// code 15 [lecture]
fn code15() {
   let s1 = String::from("hello");

   let len = calculate_length2(&s1);

   println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length2(s: &String) -> usize {
   // s is a reference to a String
   s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

// code 16 [lecture]
fn code16() {
   let s = String::from("hello");

   change1(&s);
}

fn change1(some_string: &String) {
   /*****************
   error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
      --> src/main.rs:320:4
       |
   319 | fn change(some_string: &String) {
       |                        ------- help: consider changing this to be a mutable reference: `&mut String`
   320 |    some_string.push_str(", world");
       |    ^^^^^^^^^^^ `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable
   ******************/
   //some_string.push_str(", world");
}

// code 17 [lecture]
fn code17() {
   let mut s = String::from("hello");
   println!("Before: {}", s);
   change2(&mut s);
   // change2(& s);
   println!("After: {}", s);
}

fn change2(some_string: &mut String) {
   some_string.push_str(", world");
}

/*fn change2(some_string: & String) {
   some_string.push_str(", world");
}*/

// code 18 [lecture]
fn code18() {
   let mut s = String::from("hello");

   let r1 = &mut s;
   /*
   error[E0499]: cannot borrow `s` as mutable more than once at a time
      --> src/main.rs:358:13
       |
   357 |    let r1 = &mut s;
       |             ------ first mutable borrow occurs here
   358 |    let r2 = &mut s;
       |             ^^^^^^ second mutable borrow occurs here
   359 |
   360 |    println!("{}, {}", r1, r2);
       |                       -- first borrow later used here
   */

   //let r2 = &mut s;

   //println!("{}, {}", r1, r2);
}

// code 19 [lecture]
fn code19() {
   let mut s = String::from("hello");

   {
      let r1 = &mut s;
   } // r1 goes out of scope here, so we can make a new reference with no problems.

   let r2 = &mut s;

   mixed();
   mixedScope();
}

fn mixed() {
   let mut s = String::from("hello");

   let r1 = &s; // no problem
   let r2 = &s; // no problem

   /*
   error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
      --> src/main.rs:396:13
      |
   394 |    let r1 = &s; // no problem
      |             -- immutable borrow occurs here
   395 |    let r2 = &s; // no problem
   396 |    let r3 = &mut s; // BIG PROBLEM
      |             ^^^^^^ mutable borrow occurs here
   */
   // let r3 = &mut s;

   //println!("{}, {}, and {}", r1, r2, r3);
   println!("{}, and {}", r1, r2);
}

fn mixedScope() {
   let mut s = String::from("hello");

   let r1 = &s; // no problem
   let r2 = &s; // no problem
   println!("{} and {}", r1, r2);
   // r1 and r2 are no longer used after this point

   let r3 = &mut s; // no problem
   println!("{}", r3);
}

// code 20
fn code20() {
   //let reference_to_nothing = dangle();
   let reference_to_nothing = no_dangle();
}

/*
error[E0106]: missing lifetime specifier
   --> src/main.rs:433:16
    |
433 | fn dangle() -> &String {
    |                ^ expected named lifetime parameter
    |
    = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
    |
433 | fn dangle() -> &'static String {
*/

/*fn dangle() -> &String {
   // dangle returns a reference to a String

   let s = String::from("hello"); // s is a new String

   &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
*/

fn no_dangle() -> String {
   let s = String::from("hello");

   s
}

// code exercise
fn swap_example() {
   // for two scalar value
   let mut i10 = 10;
   let mut i20 = 20;

   println!("[1] Before: i10 = {}, i20 = {}", i10, i20);

   let t = i10;
   i10 = i20;
   i20 = t;

   println!("[1] After: i10 = {}, i20 = {}", i10, i20);
   //println!("t = {}", t);

   // for two string type value
   let mut l = String::from("left");
   let mut r = String::from("right");

   println!("[2] Before: l = {}, r = {}", l, r);

   let t = l;
   l = r;
   r = t;
   println!("[2] After: l = {}, r = {}", l, r);
   // we cannot use t anymore
   //println!("t = {}", t);

   let mut i10 = 10;
   let mut i20 = 20;

   println!("[3] Before: i10 = {}, i20 = {}", i10, i20);

   let (i10, i20) = swap_int(i10, i20);

   println!("[3] After: i10 = {}, i20 = {}", i10, i20);

   // Reference

   let mut i10 = 10;
   let mut i20 = 20;

   println!("[4] Before: i10 = {}, i20 = {}", i10, i20);

   swap_int_ref(&mut i10, &mut i20);

   println!("[4] After: i10 = {}, i20 = {}", i10, i20);

   // string swap using function

   let mut l = String::from("left");
   let mut r = String::from("right");

   println!("[5] Before: l = {}, r = {}", l, r);

   let (l, r) = swap_str(l, r);

   println!("[5] After: l = {}, r = {}", l, r);

   // string swap using function (ref)

   let mut l = String::from("left");
   let mut r = String::from("right");

   println!("[6] Before: l = {}, r = {}", l, r);

   swap_str_ref(&mut l, &mut r);

   println!("[6] After: l = {}, r = {}", l, r);
}

fn swap_int(mut a: i32, mut b: i32) -> (i32, i32) {
   let t = a;
   a = b;
   b = t;
   (a, b)
}

fn swap_int_ref(a: &mut i32, b: &mut i32) {
   *a ^= *b;
   *b ^= *a;
   *a ^= *b;
}

fn swap_str(mut lstr: String, mut rstr: String) -> (String, String) {
   let t = lstr;
   lstr = rstr;
   rstr = t;

   (lstr, rstr)
}

fn swap_str_ref(lstr: &mut String, rstr: &mut String) {
   std::mem::swap(lstr, rstr);
}

fn code21() {
   let mut s = String::from("hello world");
   let len = s.len();

   let hello = &s[0..5];
   // let hello = &s[..5];
   let world = &s[6..len];
   // let world = &s[6..];

   let word = first_word(&s);
   /*****
      error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
      --> src/main.rs:581:4
       |
   580 |    let word = first_word(&s);
       |                          -- immutable borrow occurs here
   581 |    s.clear();
       |    ^^^^^^^^^ mutable borrow occurs here
   582 |    println!("{} => {}", s, word);
       |                            ---- immutable borrow later used here
      ******/
   //s.clear();
   println!("{} => {}", s, word);

   let my_string_literal = "hello world";

   // Because string literals *are* string slices already,
   // this works too, without the slice syntax!
   let word = first_word_str(my_string_literal);
   println!("{} => {}", s, word);
}

fn first_word(s: &String) -> &str {
   let bytes = s.as_bytes();

   for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
         return &s[0..i];
      }
   }

   &s[..]
}

fn first_word_str(s: &str) -> &str {
   let bytes = s.as_bytes();

   for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
         return &s[0..i];
      }
   }

   &s[..]
}
