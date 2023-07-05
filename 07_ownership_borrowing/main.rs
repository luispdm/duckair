fn main() {
    // ownership: "there can be one and only one owner of data at a time". It applies only to data on the heap!!!
    let original = String::from("ciao");
    println!("{}", original);
    let new = original;
    // println!("{}", original); // error: borrow of moved value: `original` - "new" is the new owner of the value, "original" doesn't exist anymore
    println!("{}", new);

    // borrowing
    let _next = &new;
    println!("{}", new); // it compiles because the value was borrowed (next has temporary ownership until it is destroyed)

    let mut _new_stuff = String::from("ciao");
    println!("{}", _new_stuff);
    _new_stuff = String::from("hola");
    let borrowing = &_new_stuff;
    println!("{}", borrowing); // ok because "new_stuff" changed before being borrowed by a read-only reference
    _new_stuff = String::from("hello");
    // println!("{}", borrowing);
    /* TODO
     * error: "new_stuff" changed after being borrowed
     * "rust can't guarantee memory safety as there are read-only references that will use the value downstream, so data can't be changed"
     * NEED TO LOOK INTO THIS!
     */
    
    let mut example = String::from("example");
    println!("{}", example);
    {
        let changing = &mut example;
        *changing = String::from("changing");
        println!("{}", changing);
        println!("{}", example);
    }
    println!("{}", example); // prints: "changing" - borrowing ended as the borrower went out of scope, only one owner here
}
