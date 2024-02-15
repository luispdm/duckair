use std::{
    fmt::{self, Debug},
    ops::{Add, Deref},
};

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

    // methods with the same name
    // Rust allows you to implement multiple traits with the same method name, as well as implementing the
    // same method on the type itself, without referencing any of the traits. how do you call
    // each of the implementations? see the code below
    trait Wizard {
        fn fly(&self);
    }
    trait Pilot {
        fn fly(&self);
    }
    struct Human {}
    impl Pilot for Human {
        fn fly(&self) {
            println!("this is your captain speaking")
        }
    }
    impl Wizard for Human {
        fn fly(&self) {
            println!("up!")
        }
    }
    impl Human {
        // implementing the same method name for the type itself, no trait involved this time
        fn fly(&self) {
            println!("*waving arms furiously*");
        }
    }
    let human = Human {};
    // human's own implementation is called
    human.fly();
    // pilot's implementation is called
    Pilot::fly(&human);
    // wizard's implementation is called
    Wizard::fly(&human);
    // you can do the same thing with associative functions
    trait RotaryEngine {
        fn accelerate();
    }
    trait JetEngine {
        fn accelerate();
    }
    struct Car {}
    impl Car {
        fn accelerate() {
            println!("turbo lag... go!");
        }
    }
    impl RotaryEngine for Car {
        fn accelerate() {
            println!("*mazda 787b sound*");
        }
    }
    impl JetEngine for Car {
        fn accelerate() {
            println!("*a pleasantly painful sound*");
        }
    }
    Car::accelerate();
    <Car as RotaryEngine>::accelerate();
    <Car as JetEngine>::accelerate();

    // supertraits
    // sometimes traits depend on functionality of other traits: i.e. your trait depends on the other
    // trait being implemented, that trait is called "supertrait".
    // this is how you tell to a trait to depend on another trait
    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            // without the addition to the trait, the compiler wouldn't be able to find "to_string()"
            let out = self.to_string();
            println!("{}", out);
        }
    }
    // Point must implement fmt of the Display trait if it wants to implement OutlinePrint
    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }
    impl OutlinePrint for Point {
        fn outline_print(&self) {
            println!("{}", self.x);
        }
    }
    // newtype pattern
    // in chapter 10 we learned about the orphan rule, which states that we can implement a trait on a type
    // as long as the type or the trait belong to our crate.
    // the code below does not compile as both Display and String are defined outside of our crate:
    // impl fmt::Display for Vec<String> {
    //     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    //         write!(f, "")
    //     }
    // }
    // with the newtype pattern, we can circumvent this restriction. i have to create a tuple struct
    // wrapping the type I want to implement the trait for:
    struct Wrapper(Vec<String>);
    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }
    /*
     * What's the downside of this approach? I cannot call the methods of Vec<String> directly on Wrapper.
     * e.g.: "w.first();" does not compile, I have to do: "w.0.first();".
     * There is a solution: I can implement the Deref trait for Wrapper such that dereferencing Wrapper returns
     * Vec<String>. The code below allows me to do that:
     */
    impl Deref for Wrapper {
        type Target = Vec<String>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    let w = Wrapper(vec![String::from("first"), String::from("second")]);
    if let Some(first) = w.first() {
        println!("{first}");
    }
    // if I want to modify Vec<String> from Wrapper, I also have to implement DerefMut for Wrapper.
}
