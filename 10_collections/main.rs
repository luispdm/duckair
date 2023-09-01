#![allow(unused_variables)]

use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    /*
     * Fixed-size collections are stored on the stack while resizable collections
     * are stored on the heap.
     *
     * Vectors, linked lists and double ended queue vectors are called "sequences".
     * Sequences store entries sequentially.
     * Elements in linked lists and double-ended queue vectors can be added and
     * removed from the front or the back of the list but cannot be sorted.
     * The opposite is true for vectors
     * 
     * As for every other element stored on the heap, resizable collections
     * will be dropped when they go out of scope.
     */

    // two ways to declare a vector
    let v: Vec<&str> = Vec::new(); // here the type MUST be specified
    let mut mv = vec!["one"]; // here the type is inferred from the items
    // four ways to add and remove elements
    mv.push("two");
    mv.push("three");
    mv.push("four");
    mv.pop(); // it returns an Option! not the actual value!
    mv.insert(3, "four");
    mv.remove(3);
    // as for arrays, "iter()" is not mandatory to print the elements of a vector
    // there's also iter_mut(), which allows modifying the elements of the vector
    for i in mv.iter() {
        println!("{}", i);
    }
    // arrays and vectors are sliced with ".." (last index is exclusive)
    {
        println!("{:?}", &mv[0..2]); // prints one, two"
    }
    /*
     * &mv[..]  => the whole slice
     * &mv[1..] => the slice from the 2nd element
     * &mv[..2] => the slice till the 2nd element
     */
    // 10 > mv.len() => Rust panics at runtime. Uncomment to trigger the error
    // println!("{}", mv[10]);
    /*
     * If mv was an array and not a vector, mv[10] would fail at compile time
     * as the size of mv would be known at compile time already.
     * 
     * Prefer "mv.get(index)". This is safer as it returns an Option.
     * It is slightly slower than "mv[X]" but they are quite close in terms of performance,
     * the difference might be noticed on millions-items vectors
     */
    // how to use the Option returned by mv.get? "if let" or "match"
    if let Some(item) = mv.get(2) {
        println!("{}", item); // "three"
    }
    let item = mv.get(10);
    match item {
        Some(item) => {
            println!("{}", item)
        }
        _ => {
            println!("element at index 10 not found")
        }
    }
    let next = &mv[0];
    // mv.push("next");
    /*
     * Rust doesn't allow mv.push here as the method takes a mutable reference to the vector,
     * but there is an immutable reference (next) which is later used in the code (println!...).
     * One of the rules of ownership and borrowing say:
     * "at any given time, you can have either one mutable reference or
     * any number of immutable references".
     * 
     * When we create an immutable reference we expect the underlying value not to change,
     * but when we push a new element to the vector we might need to allocate more memory to
     * make room for that element, moving the existing elements to other memory locations,
     * so the immutable reference would be pointing to something else.
    */
    println!("{}", next);

    // other useful methods are: clear(), contains(x) and len(), which do exactly what their names suggest
    let mut vdq: VecDeque<String> = VecDeque::new();
    vdq.push_back("two".to_string());
    vdq.push_front("one".to_string()); // pop_front and pop_back exist too

    /*
     * Maps. Two generic types: one for the key and another for the value.
     * Once the type is chosen it cannot be changed.
     * Btree maps and hashmaps. In btree maps keys can be sorted.
     */
    let mut hm = HashMap::new();
    hm.insert(1, ("1", "1"));
    hm.insert(1, ("one", "onemap")); // no key collision checking => value at 1 overwritten
    if let Some(item) = hm.get(&1) { // "get" works exactly as in vectors
        println!("{:?}", item);
    }
    // how to print the elements of a map
    for (k, v) in &hm {
        println!("{}: {:?}", k, v);
    }
    // "or_insert" inserts a value if no element exists for the specified key
    hm.entry(1).or_insert(("two", "twomap")); // do nothing
    hm.entry(2).or_insert(("two", "twomap")); // insert: 2: ("two", "twomap")

    // sets store values only but they use a map to store data internally (key, map)
    // btree sets and hashsets
    let mut s = HashSet::new();
    s.insert(1);
    s.insert(1); // as with other languages, sets do not allow duplicates => no error, element overwritten
    println!("{:?}", s);
}
