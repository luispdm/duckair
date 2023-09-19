use std::{collections::HashMap, hash::Hash, thread, time::Duration};

fn main() {
    let var = 5;
    // closures have access to variables defined outside their scope
    let closure = || {
        println!("i can access {}", var);
    };
    closure();
    // functions cannot do that - this doesn't compile!
    // fn func() {
    //     println!("I cannot access {}", var);
    // }

    // closures have the ability to infer input parameter type and return type at compile time
    let print = |num| {
        println!("type inference for {}", num);
        num
    };
    // the type inference occurs when the closure is called
    print(2);
    // this doesn't compile: closures with type inference can only have one concrete type inferred
    // for each input parameter
    // print("2");

    /*
     * The type inference for the closures works as it works for the variables:
     * closures are usually short and only relevant in their own context.
     *
     * That said, nothing prevents you from using type declarations.
     * See the example below
     */
    // let with_type = |num: i8| -> i8 {
    //     println!("explicit type defined for {}", num);
    //     num
    // };

    // see comments and structs after main
    generate_workout(20, 0);

    /*
     * Closures implement either Fn, FnOnce or FnMut.
     * Fn: immutable borrow of the input parameters
     * FnMut: mutable borrow of the input parameters
     * FnOnce: take ownership of the input parameters and can only be called once
     *
     * Rust is able to infer which kind of Trait the closure is implementing automatically.
     * We can also force a closure to take ownership of a value. This is useful when passing
     * data between threads (example at "12_concurrency/main.rs").
     * See example below
    */
    let x = vec![1];
    // taking ownership takes place with the "move" keyword
    let equal_to_x = move |z| z == x;
    let zz = vec![1]; 
    equal_to_x(zz);
    // equal_to_x(zz); // "use of moved value zz"
    // println!("{:?}", x); // "borrow of moved value x" - the closure has taken ownership of x
}

// the power of closures: using closures & hashmaps to avoid repeating expensive calculations
struct Cache<T, U>
where
    T: Fn(U) -> U,
    U: Eq + Hash + Copy, // Eq and Hash required by HashMap key type
{
    calculation: T,
    values: HashMap<U, U>,
}

impl<T, U> Cache<T, U>
where
    T: Fn(U) -> U,
    U: Eq + Hash + Copy,
{
    fn new(calculation: T) -> Cache<T, U> {
        Cache {
            calculation,
            values: HashMap::new(),
        }
    }
    fn value(&mut self, arg: U) -> &U {
        // Copy trait required for arg because it is used twice here
        self.values
            .entry(arg)
            .or_insert_with(|| (self.calculation)(arg))
    }
}

fn generate_workout(intensity: u32, random: u32) {
    // implementing this with a std function would have meant repeating code and/or
    // executing the function even when it was not necessary (see first "else if" branch)
    let mut cache = Cache::new(|num| {
        println!("simulating computationally expensive function...");
        thread::sleep(Duration::from_secs(2));
        num * 2
    });
    if intensity < 25 {
        println!("Do {} pushups", cache.value(intensity));
        // this time the closure is not being called because the entry already exist in the map
        println!("Now, do {} situps", cache.value(intensity));
    } else if random == 3 {
        println!("Take a break!");
    } else {
        println!("Run {} minutes", cache.value(intensity));
    }
}
