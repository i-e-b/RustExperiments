// Here are some tests around the document examples.
// It's basically the same thing twice.

//! This crate (`unit_testing`) has a function that adss two numbers.
//!
//! # Examples
//!
//! ```
//! assert_eq!(4, unit_testing::add(2,2));
//! ```

/// This function adds two numbers together
///
/// # Examples
///
/// ```
/// use unit_testing::add;
/// assert_eq!(7, add(3,4));
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a+b
}

#[cfg(test)]
mod tests {
    use super::*;  // use all definitions outside of this `mod` scope

    #[test]
    fn adding_stuff(){
        assert_eq!(4, add(1,3));
    }

// Failure stuff

    #[test]#[should_panic(expected = "assertion failed")] // this is done by string contains.
    fn fails_with_a_specific_message() {
        assert_eq!("Hello", "world");
    }
    #[test]#[should_panic]
    fn should_fail() {
        assert!(false);
    }
    #[test]#[ignore]
    fn doesnt_work(){
        assert!(false);
    }
}

