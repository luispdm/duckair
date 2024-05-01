fn main() {
    // function pointers
    /*
     * Aside from passing closures to functions, you can pass functions to functions.
     * This is useful when you want to pass a function you've already defined, instead
     * of rewriting it as a closure
     */
    fn add_one(x: i32) -> i32 {
        x + 1
    }
    fn do_twice(f: fn(i32) -> i32, x: i32) -> i32 {
        f(x) + f(x)
    }
    // the code below prints 12
    println!("do twice on 5 results in: {}", do_twice(add_one, 5));
    // fn is a type which implements all the three closure traits: Fn, FnMut and FnOnce
    // for this reason, wherever I can pass a function pointer, I can also pass a closure:
    println!("do twice on 5 results in: {}", do_twice(|x| x + 1, 5));
    // although it is best practice to write functions accepting closures instead of "fn":
    fn do_twice_alt<T>(f: T, x: i32) -> i32
    where
        T: Fn(i32) -> i32,
    {
        f(x) + f(x)
    }
    // one case in which you might only accept function pointers instead of closures, is when
    // interacting with external code. for example, C doesn't support closures.
    // examples where both a function pointer or a closure can be used:
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
    // another example but this time using tuple structs and enum variants
    #[derive(Debug)]
    enum Status {
        Value(u32),
        Stop,
    }
    let list_of_statuses: Vec<Status> = (0..5).map(Status::Value).collect();
    // i can also do the following but the compiler flags it as "redundant closure":
    // let list_of_statuses: Vec<Status> = (0..5).map(|x| Status::Value(x) /* Status::Value */).collect();
    println!("{:?}", list_of_statuses);
    /*
     * Tuple structs use parentheses to initialize values inside them. "Value(u32)" looks like a function call,
     * in fact these initializers are implemented as functions that take in arguments and return an instance
     * of that tuple struct. This means we can use Value as a function pointer!
     */

    // returning closures from functions
    // closures are represented using traits, so we can't return a concrete type here, instead, we'll use the
    // impl trait syntax:
    fn return_closure() -> impl Fn(i32) -> i32 {
        |x| x + 1
    }
    // that syntax doesn't work in all situations, let'see the following:
    // fn err_return_closure(a: i32) -> impl Fn(i32) -> i32 {
    //     if a > 0 {
    //         move |b| a+b
    //     } else {
    //         move |b| a-b
    //     }
    // }
    // the code above does not compile and it gives the error: "no two closures, even if identical, have the same type"
    // the impl trait syntax only works with one type! so, to fix the error, we have to do as the compiler suggests:
    // "consider boxing your closure or use it as a trait object":
    fn boxed_closure(a: i32) -> Box<dyn Fn(i32) -> i32> {
        if a > 0 {
            Box::new(move |b| a + b)
        } else {
            Box::new(move |b| a - b)
        }
    }
}
