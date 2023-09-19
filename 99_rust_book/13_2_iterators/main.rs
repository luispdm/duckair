fn main() {
    // iterators are part of the std library
    let v1 = vec![1, 2, 3];
    /*
     * Iterators are lazy: when "iter()" is called, the iterator
     * is instantiated but nothing happens until the iterator is used.
     *
     * Note: the variable holding the iterator must be declared mutable
     * because "next()" wants a mutable reference to the iterator
     * (see comments below).
     */
    let mut v1_iter = v1.iter();
    // "iter()" returns immutable references
    // "iter_mut()" returns mutable references
    // "into_iter()" returns owned types

    /*
     * If you want to implement the Iterator trait, you only
     * have to implement the "next" method. This is how the Iterator
     * trait looks like in the std library (more or less):
     */
    // pub trait Iterator {
    //     type Item;
    //     fn next(&mut self) -> Option<Self::Item>;
    // }
    let v1_0 = v1_iter.next();
    println!(".next() on v1 returns the Option: {:?}", v1_0);

    /*
     * Two families of iterators: adapters and consumers.
     *
     * Adapters: take in an iterator and return another iterator
     * Consumers: take in an iterator and return another type
     *
     * Note: type annotation is needed when consuming elements from
     * an iterator.
     */
    // example of a consumer: returning the sum of elements from a Vector
    let total: i32 = v1_iter.sum();
    println!("the sum of the elements in v1 is: {}", total); // 6
    // example of an adapter - laziness in action: nothing happens
    // until "collect()" is called. Note that collect is a consumer
    let v2: Vec<i32> = v1.iter().map(|x| x+1).collect();
    println!("{:?}", v2); // [2, 3, 4]

    println!("{:?}", shoe_in_my_size(vec![Shoe{
        size: 43,
        style: String::from("sneaker"),
    }, Shoe{
        size: 44,
        style: String::from("sandals"),
    },Shoe{
        size: 47,
        style: String::from("gym"),
    }, Shoe{
        size: 43,
        style: String::from("climbing"),
    }], 43)); // "climbing" and "sneaker" shoes

    let mut count = Counter::new();
    println!("{:?}", count.next()); // Some(1)
    println!("{:?}", count.next()); // Some(2)
    println!("{:?}", count.next()); // Some(3)
    println!("{:?}", count.next()); // None

    println!("{}", the_power_of_iterators());

    /*
     * Are there any performance penalties in using loops or iterators?
     * No. Rust follows the zero-cost abstraction principle.
     *
     * Zero-cost abstraction principle from the Rust book:
     * What you don’t use, you don’t pay for.
     * What you do use, you couldn’t hand code any better.
     *
     * Link: https://doc.rust-lang.org/book/ch13-04-performance.html
     */
}

#[derive(Debug)]
struct Shoe {
    size: u32,
    style: String,
}

// another example of adapter & consumer iterators
fn shoe_in_my_size(shoes: Vec<Shoe>, size: u32) -> Vec<Shoe> {
    // the closure is capturing a variable defined outside the clousure's scope
    shoes.into_iter().filter(|s| s.size == size).collect()
}

// let's implement an iterator!
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 3 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

/*
 * The power of iterators: when you implement an iterator, you have
 * access to all sort of structs that implement the iterator trait.
 * You can combine the iterators together for whatever you need.
 */
fn the_power_of_iterators() -> u32 {
    Counter::new()
        // (1, 2) (2, 3) (coming from [1, 2, 3] & [2, 3])
        .zip(Counter::new().skip(1))
         // 2, 6 (coming from 1*2 & 2*3)
        .map(|(a, b)| a * b)
        // 6
        .filter(|x| x % 3 == 0)
        // 6
        .sum()
}
