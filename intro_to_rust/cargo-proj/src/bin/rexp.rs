use regex::Regex;

fn main() {
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("The date follows rule: {}", re.is_match("201-01-01"));
}
