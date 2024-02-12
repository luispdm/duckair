fn main() {
    println!("just read the comments");
}

// the struct it's public but not its fields => that's how I achieve encapsulation in Rust
pub struct AverageCollection {
    list: Vec<i32>,
    average: f64,
}

/*
 * There's no inheritance concept in Rust. The reasons why you want inheritance are two:
 * 1. code sharing: implement the behaviour on one type so that types inheriting from that reuse
 *    that behaviour
 * 2. polymorphism
 *
 * In Rust you can achieve #1 with default trait implementation (see 99_rust_book/10_1_traits),
 * but keep in mind that, as of February 2024, you can't have fields in traits.
 * In Rust you can achieve #2 with generics and trait bounds. Generics can be used to abstract away
 * concrete types and trait bounds can be used to restrict the characteristics of those types. Rust
 * also offers trait objects: they are similar to generics but they use dynamic dispatch, as opposed
 * to generics that use static dispatch. More at 99_rust_book/17_2_trait_obejcts
 */
