pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn is_even(num: i32) -> bool {
    num%2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    // a test in Rust fails when something inside the test panics
    #[test]
    fn it_works() {
        assert_eq!(add(2, 2), 4);
    }
    #[test]
    fn it_doesnt_work() {
        assert_eq!(add(2, 3), 4);
    }
    #[test]
    fn negative_test() {
        assert_ne!(add(2, 3), 4);
    }
    #[test]
    fn five_not_even() {
        assert!(!is_even(5))
    }
}
