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
    // thanks to the deref method, the compiler knows how to perform "*a"
    assert_eq!(5, *a); // syntactic sugar for: *(a.deref())
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
    fn deref(&self) -> &T { // &Self::Target works too
        &self.0
    }
    // TODO continues at 6:40
}
