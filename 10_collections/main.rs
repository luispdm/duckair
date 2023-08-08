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
     */

    // two ways to declare a vector
    let v: Vec<&str> = Vec::new();
    let mut mv: Vec<&str> = vec!["one"];
    // four ways to add and remove elements
    mv.push("two");
    mv.push("three");
    mv.push("four");
    mv.pop();
    mv.insert(3, "four");
    mv.remove(3);
    for i in mv.iter() {
        println!("{}", i);
    }
    // arrays and vectors are sliced with ".." (last index is exclusive)
    {
        println!("{:?}", &mv[0..2]); // prints one, two"
    }
    // 10 > mv.len() => Rust panics. Uncomment to trigger the error
    // println!("{}", mv[10]);
    /*
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

    // sets store values only but they use a map to store data internally (key, map)
    // btree sets and hashsets
    let mut s = HashSet::new();
    s.insert(1);
    s.insert(1); // as with other languages, sets do not allow duplicates => no error, element overwritten
    println!("{:?}", s);
}
