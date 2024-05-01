struct CustomSP {
    data: String,
}

impl Drop for CustomSP {
    // Drop allows us to customize what happens when the smart pointer goes out of scope
    // drop is called automatically when the smart pointer goes out of scope
    fn drop(&mut self) {
        println!("printing {} before the smart pointer goes out of scope...", self.data);
    }
}

fn main() {
    // data is dropped with LIFO order: five will be dropped first and four then
    let four = CustomSP{data: String::from("four")};
    let five = CustomSP{data: String::from("five")};
    println!("{} and {} created", five.data, four.data);
    /*
     * There are situations in which one might want to clean up values early, for example
     * when managing locks: you wanna release the lock manually so other code can
     * access the unlocked resource. To do this, one should call the drop method explicitly.
     *
     * Rust does not allow explicitly calling the custom drop method of the smart pointer,
     * but it does allow calling the std lib drop method. The std lib drop method will
     * actually call the argument's implementation of drop, so in our case, the custom drop
     * implemented above will be called.
     */
    // four.drop(); // it does not compile
    let three = CustomSP{data: String::from("three")};
    println!("{} created", three.data);
    drop(three); // std lib drop
    println!("three dropped manually before the end of main");
}
