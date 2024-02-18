use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro; // because HelloMacro is a procedural macro, i need to import both crates
use macros::vec_two;

#[derive(HelloMacro)] // this is the trait "HelloMacro" defined inside the crate "hello_macro"
struct Pancakes;

fn main() {
    let v = vec_two!(1, 2, 3);
    println!("vector generated via declarative macro \"vec_two\": {:?}", v);

    Pancakes::hello_macro();
}
