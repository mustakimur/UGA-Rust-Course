fn main() {
    // Code 1
    //println!("Code 1 output:");
    //code1();

    // code 2
    //println!("Code 2 output:");
    //code2();

    // code 3
    //println!("Code 3 output:");
    //code3();

    // code 4
    //println!("Code 4 output:");
    //code4();

    // code 5
    //println!("Code 5 output:");
    //code5();

    // code 6
    //println!("Code 6 output:");
    //code6();

    // code 7
    println!("Code 7 output:");
    code7();
}
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn code1() {
    let kind: IpAddrKind = IpAddrKind::V4;
    println!("{:#?}", kind);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    print_ip(&home);
    print_ip(&loopback);
}

fn print_ip(ip: &IpAddr) {
    match ip.kind {
        IpAddrKind::V4 => println!("Ip4 address {}", ip.address),
        IpAddrKind::V6 => println!("Ip6 address {}", ip.address),
    }
}

#[derive(Debug)]
enum IpAddrVariants {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddrVariants {
    fn print_ip_details(&self) {
        match self {
            IpAddrVariants::V4(a, b, c, d) => println!("Ip4 address {}:{}:{}:{}", a, b, c, d),
            IpAddrVariants::V6(address) => println!("Ip6 address {}", address),
        }
    }
}

fn code2() {
    let kind = IpAddrVariants::V4;

    println!("{:#?}", kind(0, 0, 0, 0));

    let home = IpAddrVariants::V4(127, 0, 0, 1);
    let loopback = IpAddrVariants::V6(String::from("::1"));

    home.print_ip_details();
    loopback.print_ip_details();
}

fn code3() {
    let mut some_string = Some("a string");
    let mut absent_string: Option<String> = None;

    if some_string.is_some() {
        println!("It is {}", some_string.unwrap());
    }

    some_string = None;

    if some_string.is_some() {
        println!("It is {}", some_string.unwrap());
    }

    println!("It is {}", absent_string.unwrap_or("none".to_string()));

    absent_string = Some(String::from("hello world"));

    println!("It is {}", absent_string.unwrap_or("none".to_string()));
}

#[derive(Debug)]
enum UsState {
    NewYork,
    Washington,
    Georgia,
    Florida,
    Texas,
    California,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            print!("It is a ");
            match state {
                UsState::NewYork => print!("New York"),
                UsState::Washington => print!("Washington"),
                UsState::Florida => print!("Florida"),
                UsState::Georgia => print!("Georgia"),
                UsState::Texas => print!("Texas"),
                UsState::California => print!("California"),
            };
            println!(" state Quarter!!!");
            25
        }
    }
}

fn code4() {
    let ret = value_in_cents(Coin::Nickel);
    println!("Coin values is {}", ret);

    let ret = value_in_cents(Coin::Quarter(UsState::Georgia));
    println!("Coin values is {}", ret);

    let ret = value_in_cents(Coin::Quarter(UsState::NewYork));
    println!("Coin values is {}", ret);

    println!("Debug enum: {:#?}", UsState::California);
}

fn code5() {
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}

fn code6() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{}, {}", five.unwrap(), six.unwrap());

    if five == Some(5){
        println!("it is five");
    }
}

fn code7() {
    let mut count = 0;
    let coin = Coin::Quarter(UsState::Georgia);
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
        count += 25;
    } else if let Coin::Dime = coin {
        count += 10;
    } else if let Coin::Nickel = coin {
        count += 5;
    } else if let Coin::Penny = coin {
        count += 1;
    }

    println!("The count is {}", count);

    let five = Some(5);
    if let Some(val) = five {
        println!("{}", val);
    }
}
