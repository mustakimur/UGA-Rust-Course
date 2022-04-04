/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = more_cargo::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
/// # Errors
/// This may return Result<Error>

pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
