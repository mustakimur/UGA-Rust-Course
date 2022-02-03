#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    // code 1
    // code1();

    // code 2
    /*let email = String::from("anemail@example.com");
    let username = String::from("anotheruser");
    let user = code2(email, username);
    println!("{:#?}", user);*/

    // code 3
    // code3(user);

    // code 4
    code4();

    // code 5
    // code5();

    // code 6
    // code6();

    // code 7
    // code7();

    // code 8
    // code8();

    // code 9
    //code9();
}

fn code1() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("{:#?}", user1);

    user1.email = String::from("anotheremail@example.com");
}

fn code2(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn code3(user1: User) {
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
    println!("{:#?}", user2);
}

fn code4() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

/*
error[E0106]: missing lifetime specifier
  --> src/main.rs:66:15
   |
66 |     username: &str,
   |               ^ expected named lifetime parameter
   |
help: consider introducing a named lifetime parameter
   |
65 | struct UserSTR<'a> {
66 |     username: &'a str,
   |

error[E0106]: missing lifetime specifier
  --> src/main.rs:67:12
   |
67 |     email: &str,
   |            ^ expected named lifetime parameter
   |
help: consider introducing a named lifetime parameter
   |
65 | struct UserSTR<'a> {
66 |     username: &str,
67 |     email: &'a str,
   |

error[E0308]: mismatched types
  --> src/main.rs:74:16
   |
74 |         email: "someone@example.com",
   |                ^^^^^^^^^^^^^^^^^^^^^
   |                |
   |                expected struct `String`, found `&str`
   |                help: try using a conversion method: `"someone@example.com".to_string()`

error[E0308]: mismatched types
  --> src/main.rs:75:19
   |
75 |         username: "someusername123",
   |                   ^^^^^^^^^^^^^^^^^
   |                   |
   |                   expected struct `String`, found `&str`
   |                   help: try using a conversion method: `"someusername123".to_string()`
*/

struct UserSTR {
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool,
}

fn code5() {
    let user1 = UserSTR {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn code6() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:#?}", rect1);
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn code7() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.area() > other.area()
    }
}

fn code8() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", (&rect1).can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn code9() {
    let sq = Rectangle::square(3);
    let area = sq.area();
    println!("Rectangle area: {}", area);
}
