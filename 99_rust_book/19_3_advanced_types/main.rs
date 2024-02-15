fn main() {
    // newtype pattern - part 2
    /*
     * We can use the newtype pattern to increase type safety: imagine we have two functions,
     * one taking age as a parameter and one taking id as a parameter, both types are u32.
     * If you want to prevent passing the age instead of the id and vice-versa, you can
     * create a type wrapping u32. Example:
     */
    struct Age(u32);
    struct ID(u32);
    fn set_age(a: Age) {
        println!("{}", a.0);
    }
    let id = ID(0);
    // the code below does not compile
    // set_age(id);
    /*
     * Another use of the newtype pattern is to abstract away implementation details. For example,
     * we can create:
     * struct People(HashMap<i32, String>);
     * The code using People will not know anything about the internals of People.
     * In general, the newtype pattern is a lightweight way to achieve encapsulation.
     */

    // type aliasing
    // this is not a new type, it's an alias, it is compatible with the type it aliases onto
    type Kilometers = i32;
    let x: Kilometers = 4;
    let y: i32 = 10;
    // 14
    println!("{}", x + y);
    // type aliasing is used to reduce repetition. example:
    let f1: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("long type name"));
    type Shorter = Box<dyn Fn() + Send + 'static>;
    f1();
    let f2: Shorter = Box::new(|| println!("shorter alias"));
    f2();

    // never type
    fn foo() -> ! {
        panic!("this function never returns");
    }
    // when is the never type useful? see the code below
    let guess = "90";
    let mut counter = vec![3, 2, 1];
    while counter.pop().is_some() {
        /*
         * How are two different return types allowed in that match statement?
         * The match expression can only return a u32: the "Err(_)" arm calls "continue",
         * which has a never type (the control is handed over back to the while statement
         * and guess is not assigned), thus Rust concludes that the only return type
         * is that of "Ok(num)".
         */
        let _: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    }
    // another example where the never type is present: the panic macro. from the std lib:
    // impl<T> Option<T> {
    //     pub fn unwrap(self) -> T {
    //         match self {
    //             Some(val) => val,
    //             None => panic!("called `Option::unwrap()` on a `None` value"),
    //         }
    //     }
    // }
    // never types are present in endless loops too.
    // IMPORTANT: if there was a "break", the loop wouldn't have a never type
    // loop {
    //     println!("forever and ever");
    // }

    // dynamically sized types (DST)
    /*
     * Their size is only known at compile time. str is an example:
     * let s1: str = "Hello"; // this code does not compile!
     * How do we fix that? The golden rule is to put such types behind some sort of pointer,
     * as the size of the pointer is always known at compile time.
     * Borrowing and smart pointers are your friends in these cases
     */
    let s1: &str = "";
    /*
     * Traits are DST, that is why trait objects have to be behind a smart pointer.
     * Rust has a special trait called "Size", used to determine whether a type size can be known
     * at compile time or not. Size is automatically implemented for every type whose size is known
     * at compile time. Rust implicitly adds the Size trait bound to every generic function
     */
    fn generic<T /*: Sized //this is added automatically*/>(t: T)
    {
    }
    // in the code below, "?" tells the compiler that the size of T might be known at compile time or not,
    // for this reason, we have to put the function parameter "t" behind a reference
    fn relaxed_restriction<T: ?Sized>(t: &T) {}
}
