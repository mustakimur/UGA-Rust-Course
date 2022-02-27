fn main() {
    //find_large();

    //dup_large();

    //gen_large();

    //gen_struct();

    //gen_mix();

    //copy_trait();

    //impl_trait();

    //borrow_val();

    //long_live_string();

    //inequal_lifetime();

    //struct_lifeitme();

    //word_lifetime();

    //static_value();

    trait_bound();
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn find_large() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);
}

fn dup_large() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
}

/*
error[E0369]: binary operation `>` cannot be applied to type `&T`
  --> src/main.rs:61:17
   |
61 |         if item > largest {
   |            ---- ^ ------- &T
   |            |
   |            &T
   |
help: consider restricting type parameter `T`
   |
57 | fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
*/

//fn largest<T>(list: &[T]) -> &T {
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

/* fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
} */

fn gen_large() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let str_list = vec![
        String::from("ABC"),
        String::from("DEF"),
        String::from("GHI"),
        String::from("JKL"),
    ];

    let result = largest(&str_list);
    println!("The largest char is {}", result);
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn gen_struct() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };

    println!("p.x = {}", both_integer.x());
    println!("p.x = {}", both_float.x());

    //println!("p.x = {}", both_integer.distance_from_origin());
    println!("p.x = {}", both_float.distance_from_origin());

    /* let u_x:u32 = 10;
    let i_y: i32 = -10;

    if (u_x < i_y){
        println!("integer overflow panic!");
    } */
}

struct PointXY<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> PointXY<X1, Y1> {
    fn mixup<X2, Y2>(self, other: PointXY<X2, Y2>) -> PointXY<X1, Y2> {
        PointXY {
            x: self.x,
            y: other.y,
        }
    }
}

fn gen_mix() {
    let p1 = PointXY { x: 5, y: 10.4 };
    let p2 = PointXY { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

use std::cmp::Ordering;
use std::fmt::Debug;
use std::fmt::Display;

//#[derive(Eq)]
struct TW {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

/* impl Display for TW {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        println!("{}", self.reply);
        Ok(())
    }
}

impl Ord for TW {
    fn cmp(&self, other: &Self) -> Ordering {
        self.username.cmp(&other.username)
    }
}

impl PartialEq for TW {
    fn eq(&self, other: &Self) -> bool {
        self.username == other.username
    }
}

impl PartialOrd for TW {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
} */

fn copy_trait() {
    let p1 = Pair::new(String::from("First Item"), String::from("Second Item"));
    p1.cmp_display();

    /* let tweet1 = TW {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    let tweet2 = TW {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    let p1 = Pair::new(tweet1, tweet2);
    p1.cmp_display(); */
}

trait Summary {
    fn summarize(&self) -> String;
    fn details(&self) -> String {
        String::from("(Read more...)")
    }
}

trait CustomDisplay {
    fn info(&self) -> String;
}

#[derive(Debug)]
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

#[derive(Debug)]
struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn details(&self) -> String {
        format!("{}: {}", self.reply, self.retweet)
    }
}

impl CustomDisplay for Tweet {
    fn info(&self) -> String {
        format!("user: {}", self.username)
    }
}

fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

/* fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
 */
/* fn notify(item: &(impl Summary + CustomDisplay)) {
    println!("From Tweeter user {}", item.info());
    println!("Breaking news! {}", item.summarize());
} */

//fn returns_summarizable(switch: bool) -> impl Summary {
//fn returns_summarizable(switch: bool) -> Box <Summary> {
fn returns_summarizable(switch: bool) -> Box<dyn Summary> {
    /* Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    } */
    if switch {
        Box::new(NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        })
    } else {
        Box::new(Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        })
    }
}

fn impl_trait() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.details());

    //notify(&tweet);

    /* let res = returns_summarizable(true);
    res.details(); */
}

fn borrow_val() {
    // example 1
    /* let r;
    {
        let x = 5;
        r = &x;
    }

    println!("r = {}", r); */

    // example 2
    {
        let x = 5;
        let r = &x;
        println!("r {}", r);
    }
}

/* fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
} */

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/*
error[E0515]: cannot return value referencing local variable `result`
   --> src/main.rs:332:5
    |
332 |     result.as_str()
    |     ------^^^^^^^^^
    |     |
    |     returns a value referencing data owned by the current function
    |     `result` is borrowed here
*/

/* fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result: String = String::from("really long string");
    let borrow: &'a str = result.as_str();
    borrow
} */

/* fn longest(x: &str, y: &str) -> &'static str {
    let result: String = String::from("really long string");
    let borrow: &'static str = result.as_str();
    borrow
} */

fn long_live_string() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

/*
error[E0597]: `string2` does not live long enough
   --> src/main.rs:343:44
    |
343 |         result = longest(string1.as_str(), string2.as_str());
    |                                            ^^^^^^^ borrowed value does not live long enough
344 |     }
    |     - `string2` dropped here while still borrowed
345 |     println!("The longest string is {}", result);
    |                                          ------ borrow later used here
*/

fn inequal_lifetime() {
    /* let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result); */
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn passover(instance: ImportantExcerpt) {}

fn struct_lifeitme() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let instance = ImportantExcerpt {
        part: first_sentence,
    };
    //drop(novel);
    passover(instance);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn word_lifetime() {
    let str = String::from("hello world");
    let word = first_word(str.as_str());
    println!("First word is {}", word);
}

static GLB_NUM: i32 = 10;

fn ret_GLB_NUM () -> &'static i32{
    & GLB_NUM
}

fn static_value() {
    let res = ret_GLB_NUM();
    println!("{}", res);
}

//fn print_it( input: impl Debug + 'static ) {
fn print_it( input: impl Debug  ) {
    println!( "'static value passed in is: {:?}", input );
}

fn trait_bound() {
    // i is owned and contains no references, thus it's 'static:
    let i = 5;
    print_it(i);

    // oops, &i only has the lifetime defined by the scope of
    // main(), so it's not 'static:
    print_it(&i);
}