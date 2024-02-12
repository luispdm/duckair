pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // why do trait objects have to use a box smart pointer? see chapter 19
    pub components: Vec<Box<dyn Draw>>, // dynamic dispatch, see below
}

impl Screen {
    pub fn run(&self) {
        for c in self.components.iter() {
            c.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("drawing button");
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("drawing selectbox");
    }
}

// why can't I simply use generics and trait bounds for my use case?
struct Screen2<T: Draw> {
    components: Vec<T>,
}

impl<T> Screen2<T>
where
    T: Draw,
{
    fn run(&self) {
        for c in self.components.iter() {
            c.draw();
        }
    }
}

/*
 * Trait bounds + generics can achieve the same functionality of trait objects but with a huge difference:
 * I cannot store a mixture of types that implement the Draw trait in the components array
 * of Screen2, I am limited to one type only. In fact, the code below does not compile.
 */

// fn test() {
//     let s = Screen2{
//         components: vec![
//             Button{
//                 width: 0,
//                 height: 0,
//                 label: String::from("label"),
//             },
//             SelectBox{
//                 width: 0,
//                 height: 0,
//                 options: vec![],
//             }
//         ]
//     }
// }

/*
 * Trait bounds and generics do a process called "monomorphization": at compile time,
 * the compiler generates non-generic implementations of functions based on the concrete types
 * used, instead of the generic types. That's how generics work (see 11_generics and
 * 99_rust_book/10_1_traits).
 *
 * With generics and trait bounds, thanks to the monomorphization, the compiler knows
 * the concrete functions called at compile time. This is called "static dispatch".
 * The opposite is called "dynamic dispatch": the compiler figures out the concrete functions
 * called at runtime. Dynamic dispatch is used with trait objects.
 * Dynamic dispatch adds a little runtime cost but also the flexibility of storing any object that
 * implements a certain trait in a Vector.
 *
 * Only objects-safe traits can be turned into trait bounds. When a trait is considered object-safe?
 * When it respects the following rules:
 * 1. the return type is not self
 * 2. there are no generic parameters
*/
