use std::ops::Add;

fn main() {
    // associated types - they work similarly as generics
    trait Iterator {
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
    }
    struct Counter {}
    impl Iterator for Counter {
        type Item = i32; // now the type of Item is known to the compiler
        fn next(&mut self) -> Option<i32> {
            Some(0)
        }
    }
    // with generics you can have multiple concrete implementations per type; with
    // associated types only one concrete implementation per type is allowed
    // this code does not compile as a concrete implementation of Iterator for Counter already exists!
    // impl Iterator for Counter {
    //     type Item = i16;
    //     fn next(&mut self) -> Option<i16> {
    //         Some(0)
    //     }
    // }
    // so the question is: "does it makes sense to have multiple implementations for a single type"?
    // if the answer is yes, then you need generics

    /*
     * Default generic type parameters and operator overloading.
     * Generic type parameters can specify a default concrete type, this allows implementers to not have to
     * specify a concrete type unless it's different than the default. A good use case for this is operator
     * overloading: Rust allows you to customize the semantics of certain operators that have associated traits
     * in the std lib module "ops". "ops" contains all the overloadable operators. Example below.
     */
    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }
    impl Add for Point {
        type Output = Point;

        fn add(self, other: Self) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }
    assert_eq!(
        Point { x: 1, y: 1 } + Point { x: 1, y: 1 },
        Point { x: 2, y: 2 }
    );
    // the code below shows the std lib implementation of the Add trait
    // it makes sense that Rhs (right hand side) is of type Self, as you typically sum items of the same kind
    // for this reason, we didn't need to specify a concrete type in our implementation of Add for Point
    trait StdAdd<Rhs = Self> {
        type Output;
        fn add(self, rhs: Rhs) -> Self::Output;
    }
    // let's see an example where we do want to specify the type of Rhs
    struct Millimeters(u32);
    struct Meters(u32);
    impl Add<Meters> for Millimeters {
        type Output = Millimeters;
        fn add(self, rhs: Meters) -> Millimeters {
            // self.0 and rhs.0 because Meters and Millimeters are tuple structs with one value
            Millimeters(self.0 + (rhs.0 * 1000))
        }
    }
    /*
     * To summarize, you wanna use generic default type parameters for two reasons:
     * 1. extend a type without breaking existing code
     * 2. allow customization for specific cases (see Meters and Millimeters)
    */

    // TODO continue video on section: "Calling methods with the same name" at 7:17
}
