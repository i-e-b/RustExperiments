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

pub fn add(a: i32, b: i32) -> i32 {
    a+b
}
