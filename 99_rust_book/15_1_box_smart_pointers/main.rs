/*
 * Smart pointers are structs that act like pointers, with metadata
 * and extra capabilities.
 * In many cases, smart pointers own the data they point to, unlike references that
 * just borrow the values.
 *
 * String and Vec are smart pointers: they own the data, they store extra metadata
 * (like the capacity of the vector) and they have extra capabilities (the String type
 * is able to determine whether the string is a valid UTF-8 or not).
 *
 * Smart pointers implement the Deref and Drop trait:
 * Deref: allows instances of smart pointers to be treated as references
 * Drop: allows you to customize the code that runs when an instance of the smart pointer
 * goes out of scope
 */

fn main() {
    // Box allows you to allocate values on the heap: the smart pointer living on the stack
    // holds the memory address of the value on the heap. The value on the heap is owned by
    // the instance of Box
    let b = Box::new(5);
    println!("{}", b);
    // when b goes out of scope, the smart pointer on the stack and the data on the heap
    // the smart pointer points to will be both deallocated
    /*
     * Box doesn't have any overhead as it doesn't have many capabilities. You'd typically
     * use Box in the following situations:
     * - when you are dealing with a large amount of data and you wanna transfer its ownership
     *   without copying it
     * - when you own a value and you want that value to only implement a specific trait,
     *   rather than being a specific type (trait objects - see chapter 17)
     * - when you use the value of a type whose size is unknown at compile time in a context
     *   which requires knowing the size at compile time (recursive type - see example below)
     */
    println!("{:?}", Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil)))))));
}

use List::{Cons, Nil};

// recursive types: when Box is useful - Cons lists: https://en.wikipedia.org/wiki/Cons
/*
 * At compile time, Rust needs to know the amount of space enums take on the stack.
 * Because of the nature of enums, there can only be one existing variant at a time, so the
 * most space the enum is gonna take is equal to the most space the largest variant is going to take.
 * For example, the size of the enum "_Message" is equal to the size of its variant "Metadata" (the
 * biggest one as it is a tuple with 3 values).
 * 
 * If I were to remove the Box smart pointer from the definition of the List enum,
 * I will see the compilation error:
 * "recursive type `List` has infinite size - recursive without indirection". Why?
 * To calculate the space List needs, Rust calculates that for Nil and when it will check the space
 * Cons takes, it will need to know how much space List takes, because List is a member of the Cons
 * tuple, so it will check the enum (List) again, and again it will check Cons and it will need to
 * know how much space List takes... Recursive problem!
 *
 * So how does the Box smart pointer fix this recursive problem?
 * This time, when Rust checks the amount of space Cons takes, it knows that Box is a fixed-size
 * smart pointer living on the stack, pointing to an arbitrary amount of data on the heap (List).
 */

 #[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

enum _Message {
    Sender,
    Content(String),
    Timestamp(u128),
    Metadata(u32, String, bool),
    Move {x: u8, y: u8}, // anonymous struct
}
