use std::fs::File;
use std::io;
use std::io::Read;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn err_msg(path: String) -> Result<String, io::Error> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    fn will_panic() {
        panic!("let's try this");
    }
    #[test]
    fn lets_panic() {
        will_panic();
    }
    #[test]
    #[should_panic]
    fn greater_than_100() {
        will_panic();
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn it_adds_three() {
        assert_ne!(4, add_two(3));
    }

    #[test]
    fn check_file() -> Result<(), io::Error> {
        match err_msg("hello.txt".to_string()) {
            Err(e) => Err(e),
            _ => Ok(()),
        }
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }
}
