use std::ops::{Add, Mul};

#[derive(Debug)]
struct WithGenerics<T, U> {
    name: String,
    first: T,
    second: U, // second's concrete type can be the same as first or a different one
}

fn main() {
    let test = WithGenerics {
        name: "test".to_string(),
        first: 1,
        second: 2.2,
    };
    println!("{}, {}, {}", test.name, test.first, test.second);
    println!("{}", add(1, 2));
    println!("{}", add_and_multiply(2, 3));
    println!("{:?}", dio(&test, &test));
}

// this tells the compiler that T must implement the trait "Add" in order to be used by the method
fn add<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

// when a type must implement multiple traits, prefer the "where" clause
fn add_and_multiply<T>(a: T, b: T) -> T
where
    T: Add<Output = T> + Mul<Output = T> + Copy,
{
    let c = a + b;
    c * a * b
}

// this is how you declare a method that uses both lifetime and generics
fn dio<'a, T>(a: &'a T, _b: &T) -> &'a T {
    a
}
