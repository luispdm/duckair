use std::ops::Deref;

fn main() {
    let x = 5;
    let y = &x;
    // * = deref operator, same as in other languages
    assert_eq!(5, *y);
    /*
     * this time z points to a copy of x because x is a primitive and when
     * primitives are passed to functions they are copied, instead of ownership
     * being transferred
     */
    let z = Box::new(x);
    // * can be called on Box because Box is a smart pointer (implements Deref)
    assert_eq!(5, *z);

    let a = MyBox::new(x);
    // thanks to the deref method, the compiler knows what to do when it finds "*a"
    assert_eq!(5, *a); // syntactic sugar for: *(a.deref())

    // implicit deref coercion: converting a reference of a type to a reference
    // of another type
    let b = MyBox::new(String::from("bbb"));
    hello(&b);
    // &MyBox<String> -> &String -> &str
    // this works because both MyBox and String implement the Deref trait.
    // if Rust didn't have automatic deref coercion, we'd have had to write the
    // above statement as:
    hello(&(*b)[..]);
    // ...worsening the readability of our code significantly

    /*
     * Rust performs automatic deref coercion when it finds types and trait implementations
     * in the following three cases:
     * - from &T to &U when T: Deref<Target=U>
     * - from &mut T to &mut U when T: DerefMut<Target=U>
     * - from &mut T to &U when T: Deref<Target=U>
     * (taken from the Rust book: https://doc.rust-lang.org/book/ch15-02-deref.html#how-deref-coercion-interacts-with-mutability)
     * 
     * Deref coercion cannot be performed from &T to &mut U because of the
     * borrowing rules: "at any given time, you can have either one mutable reference or
     * any number of immutable references."
     * In this case, Rust cannot guarantee &T is the only existing immutable reference.
     * Going from &mut T to &U does not violate the borrowing rules because the compiler is sure
     * that &mut T is the only mutable reference out there.
     */
}

fn hello(name: &str) {
    println!("hello {}", name);
}

// defining a tuple struct that implements the Deref trait
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        // one thing to note here: x is stored on the stack, conversely to
        // Box where x would be stored on the heap. this is ok for now as we're
        // focusing only on the Deref trait
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    /*
     * Why does deref return &T and not T?
     * Because if deref returned T, it would move ownership of T outside of
     * the instance of MyBox, which is a smart pointer and this is something
     * we don't usually want (remember, in many cases smart pointers own
     * the data they point to)
     */
    fn deref(&self) -> &T { // -> &Self::Target works too
        &self.0
    }
    /*
     * As the Deref trait is used to override the dereference operator for
     * immutable references, there's also "DerefMut", which lets you override
     * the dereference operator for mutable references
     */
}
