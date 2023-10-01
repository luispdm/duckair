/*
 * Rc = Reference Counted
 * A type of smart pointer that allows multiple items to read the same value on the heap
 * by keeping track of the number of references at any given time. It's like having multiple
 * owners of the same data.
 * Note that Rc doesn't allow multiple items to modify the same value otherwise we'd be violating
 * the borrowing rule: "only one mutable reference at any time".
 *
 * Rc is only useful in single-threaded programs.
 *
 * We'll use the Cons list struct from chapter 15.1 (see 15_1_box_smart_pointers) as an example.
*/
use crate::List::{Cons, Nil};
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    /*
     * If Cons had Box<List> instead of Rc<List>, the code below wouldn't have compiled:
     * "use of moved value" - ownership of a has been moved into b.
     * We could solve it using lifetimes but that means the items in the List must live as long as
     * the List itself. That is not possible in this case. The following wouldn't compile for example:
     * list a = Cons(10, &Nil);
     * Check this for further details: https://youtu.be/M9Owp3iLigg?si=jYHgkGTYCqbpEa46&t=245
     */
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(3, Box::new(a)); // error - "use of moved value"

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // *a = Cons(1, Rc::new(Nil)); // error - "Rc doesn't implement DerefMut" (Rc = immutable refs only)
    println!("allocating a, reference counter is at: {}", Rc::strong_count(&a)); // 1
    // Rc::clone(&a) gives us an owned value, but it doesn't perform a deep clone of a,
    // it just increments the reference counter of a
    let b = Cons(3, Rc::clone(&a)); // a.clone() works too but Rc::clone is the convention
    println!("allocating b, reference counter is at: {}", Rc::strong_count(&a)); // 2
    let c = Cons(4, Rc::clone(&a));
    println!("allocating c, reference counter is at: {}", Rc::strong_count(&a)); // 3
    {
        let d = Cons(2, Rc::clone(&a));
        println!("allocating d, reference counter is at: {}", Rc::strong_count(&a)); // 4
    }
    println!("deallocating d, reference counter is at: {}", Rc::strong_count(&a)); // 3

    // the following doesn't compile:
    // let b = Cons(3, a);
    // let c = Cons(3, a); // same as if Cons had Box<List>: error - "use of moved value"
}
