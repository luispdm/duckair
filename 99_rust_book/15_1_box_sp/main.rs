/*
 * Smart pointers are structs that act like pointers, with metadata
 * and extra capabilities.
 * In many cases, smart pointers own the data they pont to, unlike references that
 * just borrow the values.
 *
 * String and Vec<> are smart pointers: they own the data, they store extra metadata
 * (like the capacity of the vector) and they have extra capabilities (the String type
 * is able to determine whether the string is a valid UTF-8 or not).
 *
 * Smart pointers implement the Deref and Drop trait:
 * Deref: allows instances of smart pointers to be treated as references
 * Drop: allows you to customize the code that runs when an instance of the smar pointer
 * goes out of scope
 */

fn main() {
    // Box allows you to allocate values on the heap: the smart pointer living on the stack
    // holds the memory address of the value on the heap. The value on the heap is owned by
    // the instance of Box
    let b = Box::new(5);
    println!("{}", b);
    /*
     * Box doesn't have any overhead and don't also have many capabilities either,
     * so you'd typically use it in the following situations:
     * - when you use the value of a type whose size is unknown at compile time, but you
     */

}
