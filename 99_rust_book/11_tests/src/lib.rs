pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)] // this tells Rust to compile this code when we run "cargo test"
mod tests {
    use super::*;

    // a test in Rust fails when something inside the test panics
    #[test]
    fn it_works() {
        // all the parameters passed to assert_eq! and assert_ne!
        // must implement the Debug and Display traits
        assert_eq!(add(2, 2), 4);
    }
    #[test]
    fn it_doesnt_work() {
        assert_eq!(add(2, 3), 4);
    }
    #[test]
    fn fails_with_msg() {
        let exp = 10;
        let rec = 11;
        // fail with a custom message
        assert_eq!(exp, rec, "Variable rec is not {exp}, got {rec}");
    }
    #[test]
    // when there's no "expected", you are telling Rust that no matter what
    // the reason behind panicking is, the test must pass (ofc if the test
    // doesn't panic, the test fails)
    #[should_panic(expected = "on purpose")]
    fn panic_should_be_ok() {
        panic!("on purpose");
    }
    #[test]
    fn negative_test() {
        assert_ne!(add(2, 3), 4);
    }
    #[test]
    fn five_not_even() {
        assert!(!is_even(5))
    }
    /*
     * Tests returning a Result are useful when there are operations
     * that might return an error: in that case, one can use the "?" operator
     * so the test can end there if the operation effectively fails
     */
    #[test]
    fn with_result() -> Result<(), String> {
        if add(2, 3) == 5 {
            Ok(())
        } else {
            Err(String::from("2 + 3 must be 4!"))
        }
    }
    #[test]
    #[ignore = "broken - fix me"] // cargo won't run this test by default
    fn i_wont_run() {

    }
}
