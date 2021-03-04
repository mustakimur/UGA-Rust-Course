fn main() {
    // Code 1
    println!("Code 1 output:");
    code1();

    // code 2
    println!("Code 2 output:");
    code2();

    // code 3
    println!("Code 3 output:");
    code3();
}

enum IpAddrKind {
    V4,
    V6,
}
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn code1() {
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
