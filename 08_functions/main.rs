fn main() {
    println!("{}", return_greater(1, 2));
}

// function - pass by value
fn return_greater(n1: u8, n2: u8) -> u8 {
    if n1 > n2 {
        return n1;
    }
    n2
}
