/*
 * Rust is a memory-safe language: it provides guarantees such as no data races. It also makes it difficult,
 * but not impossible, to have memory that is never cleaned up, i.e. memory leaks.
 * By using Rc and RefCell, one can create reference cycles that end up in memory leaks.
 */
#![allow(dead_code)]
use crate::List::*;
use std::{cell::RefCell, rc::{Rc, Weak}};

// example of memory leak with reference cycles
#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a ref strong count before instantiating b: {}", Rc::strong_count(&a)); // 1
    println!("a tail: {:?}", a.tail()); // Nil
    let b = Rc::new(Cons(4, RefCell::new(Rc::clone(&a))));
    println!("a ref strong count after instantiating b: {}", Rc::strong_count(&a)); // 2
    println!("b ref strong count: {}", Rc::strong_count(&b)); // 1
    println!("b tail: {:?}", b.tail()); // a
    if let Some(next) = a.tail() {
        *next.borrow_mut() = Rc::clone(&b); // the reference cycle happens here
    }
    println!("b ref strong count after modifying a: {}", Rc::strong_count(&b)); // 2
    /*
     * Memory when a and b are instantiated:
     * (stack -> heap)
     * a -> 'a(5, Nil) // a ref count 2
     * b -> 'b(10, 'a) // b ref count 1
     *
     * Memory when a is modified:
     * (stack -> heap)
     * a -> 'a(05, 'b) // a ref count 2
     * b -> 'b(10, 'a) // b ref count 2
     *
     * At this point, if we were to call a.tail() or b.tail(), we'd have a stack overflow and
     * a memory leak. Why?
     * 1. Stack overflow: if we do b.tail(), the function returns 'a, which in turn returns 'b and so on.
     *    It basically gets stuck in a endless loop until the stack is overflowed.
     * 2. Memory leak: at the end of main, b gets cleaned up (stack clean up is LIFO), but not
     *    the value it points to on the heap, (10, 'a), as it is still referenced by a (a tail is 'b).
     *    Then a is dropped, but not the value on the heap it points to, (5, 'b), as it is still referenced
     *    by the value on the heap b used to point to: (10, 'a). b was dropped but its value on the heap still exists!
     *    We ended up with a memory leak as both values on the heap were not cleaned up.
     * To visualize the memory in this situation, check the Let's Get Rusty video on this Rust book chapter:
     * https://youtu.be/pIVZRDFAUyc?si=U71HTNFkMXOIxn3o&t=296
     */
    // uncomment either of the following if you wanna see a stack overflow and a memory leak
    // println!("a tail: {:?}", a.tail());
    // println!("b tail: {:?}", b.tail());

    // the follwing shows how to solve the reference cycle issue with weak pointers and a tree structure as example
    /*
     * First look at the code below and then come back here. Check the "Node" struct especially.
     *
     * Why isn't the field "parent" of "Node" a "RefCell<Rc<Node>>"?
     * There are two reasons, one is related with the logic of tree data structures and the other with
     * reference cycles.
     * 1. Mutual owning references would never allow either Rc to be dropped (reference cycle between
     *    parent and children).
     * 2. Even if #1 was false, it wouldn't make sense for parent to be a RefCell<Rc...:
     *    with Rc, references own the data they point to. If parent were to be a RefCell<Rc..., when
     *    "leaf" is dropped, "branch" would be dropped too! In trees it's usually the other way around:
     *    if branch is dropped, leaf is dropped. A branch should own its leaves, not the opposite.
     *    That's why children is of type RefCell<Vec<Rc.
     *
     * Parent is a RefCell<Weak<Node>> because with Weak, references do not own the data they point to.
     * If leaf were to be dropped, branch would continue existing.
     */
    println!("--------- weak pointers to the rescue! ---------");
    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }
    let leaf = Rc::new(Node{
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!("leaf strong & weak count after creation: {}, {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf)); // 1, 0
    // whenever we want to see or modify the value inside Weak we have to call the "upgrade" method:
    // the method tries to convert Weak to Rc and it returns an Option because the value referenced by Weak
    // might have been dropped and Weak has no idea about it
    println!("leaf's parent after creation: {:?}", leaf.parent.borrow().upgrade()); // None
    {
        let branch = Rc::new(Node{
            value: 4,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        println!("branch strong & weak count after creation: {}, {}", Rc::strong_count(&branch), Rc::weak_count(&branch));  // 1, 1
        println!("leaf strong & weak count after branch is created: {}, {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));  // 2, 0
        // downgrade converts Rc to Weak
        println!("leaf's parent after assigning branch to leaf.parent: {:?}", leaf.parent.borrow().upgrade()); // Some(branch)
    }
    // here the strong count of branch is 0 and the weak count of branch is 1 (weakly-referenced by leaf) => branch is dropped
    // the fact that weak is still 1 doesn't influence the decision to drop branch
    println!("leaf strong & weak count after branch is dropped: {}, {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf)); // 1, 0
    println!("leaf's parent after assigning branch to leaf.parent: {:?}", leaf.parent.borrow().upgrade()); // None
    // all the logic managing the strong count and weak count of Rc & Weak is managed by their respective implementation of Drop
}
