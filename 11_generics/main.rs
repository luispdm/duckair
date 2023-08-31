use std::ops::{Add, Mul};

#[derive(Debug)]
struct WithGenerics<T, U> {
    name: String,
    first: T,
    second: U, // second's concrete type can be the same as first or a different one
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
fn lt_gn<'a, T>(a: &'a T, _b: &T) -> &'a T {
    a
}

/*
 * Generics identifiers defined here don't need to be the same as the ones
 * defined in the original struct (note X and Y here, while T and U were defined above).
 */
impl<X, Y> WithGenerics<X, Y> {
    fn first(&self) -> &X {
        &self.first
    }
    fn second(&self) -> &Y {
        &self.second
    }
}

// this method can be used only if "first" is a bool
impl<U> WithGenerics<bool, U> {
    fn no_generic(&self) -> bool {
        self.first
    }
}

// this is possible too but the generic types must be respected
impl<T, U> WithGenerics<T, U> {
    fn mixup<V, W>(self, another: WithGenerics<V, W>) -> WithGenerics<T, W> {
        WithGenerics {
            name: "i am quirky".to_string(),
            first: self.first, // another.first wouldn't work
            second: another.second, // self.second wouldn't work
        }
    }
}

// generics apply to enums too
enum TheResult<T, E> {
    Ok(T),
    Err(E),
}

/*
 * Generics in Rust do not cause performance penalties.
 * How? Consider the following code:
 * 
 * enum Test<T> {
 *   Value(T),
 *   NoVal
 * }
 * 
 * fn main() {
 *   let integer = Test::Value(5);
 *   let float = Test::Value(5.0);
 * }
 * 
 * At compile time, Rust will turn the generic enum into 2 enums:
 * enum Test_i32 {
 *   Value(i32),
 *   NoVal
 * }
 * enum Test_f64 {
 *   Value(f64),
 *   NoVal
 * }
 * 
 * fn main() {
 *   let integer = Test_i32::Value(5);
 *   let float = Test_f64::Value(5.0);
 * }
 */

fn main() {
    let test = WithGenerics {
        name: "test".to_string(),
        first: 1,
        second: 2.2,
    };
    println!("{}, {}, {}", test.name, test.first, test.second);
    println!("{}", add(1, 2));
    println!("{}", add_and_multiply(2, 3));
    println!("{:?}", lt_gn(&test, &test));
}
