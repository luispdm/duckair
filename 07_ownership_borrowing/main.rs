fn main() {
    // ownership: "there can be one and only one owner of data at a time". It applies only to data on the heap!!!
    // when the owner goes out of scope, the value is dropped
    let original = String::from("ciao");
    println!("{}", original);
    let new = original;
    // println!("{}", original); // error: borrow of moved value: `original` - "new" is the new owner of the value, "original" doesn't exist anymore
    println!("{}", new);

    // borrowing - done via references (&)
    /*
     * 1. At any given time, you can have either one mutable reference
     * or any number of immutable references.
     * 2. References must always be valid.
     * 3. The scope of the reference ends when the reference is last used.
     * 
     * Addition on 1:
     * Think of "&var" as a shared reference (you can have more than one).
     * Think of "&mut var" as an exclusive reference (you can only have one).
     */
    let _next = &new;
    println!("{}", new); // it compiles because the value was borrowed (next has temporary ownership until it is destroyed)

    let mut _new_stuff = String::from("ciao");
    println!("{}", _new_stuff);
    // 👇 "new_stuff" can be changed here because the immutable reference comes afterwards
    _new_stuff = String::from("hola");
    let borrowing = &_new_stuff;
    println!("{}", borrowing); // reference last used => scope of the reference ends here
    _new_stuff = String::from("hello");
    /*
     * 👇 this causes compilation errors on new_stuff 2nd's assignment
     * as the immutable reference is used after the value is changed
     */
    // println!("{}", borrowing);
    // 👇 ok because there are no more immutable references in use
    let mutable = &mut _new_stuff;
    mutable.push_str(", world");
    // compilation error: a mutable reference is used after (👇 is an immutable reference)
    // println!("{}", _new_stuff);
    println!("{}", mutable);

    let mut example = String::from("example");
    println!("{}", example);
    {
        let changing = &mut example;
        *changing = String::from("changing");
        println!("{}", changing);
        println!("{}", example);
    }
    println!("{}", example); // prints: "changing" - borrowing ended as the borrower went out of scope here

    // lifetime - it only applies to data stored on the heap
    let _outer_int;
    {
        let inner_int = 5;
        _outer_int = &inner_int;
    }
    // println!("{}", _outer_int); // error: dangling reference as "inner_int" was deallocated

    let to_ret = 5;
    println!("{}", ok_ref(&to_ret));

    let p1 = 42;
    let p2 = 88;
    println!("{}", which_lifetime(&p1, &p2));
}

/* fn ret_bad_ref<'life>() -> &'life i32 {
    let val = 5;
    return &val; // cannot return a reference to a local variable as it will be deallocated => dangling reference
} */

fn ok_ref(val: &i32) -> &i32 {
    val // the scope sending the reference is the same getting the reference back => no need to specify lifetime and no dangling references
}

// need to specify the lifetime of the param you are returning as there are 2 params in the function
fn which_lifetime<'a>(p1: &'a i32, p2: &i32) -> &'a i32 {
    if p1 < p2 {
        return p1;
    }
    &0
}
