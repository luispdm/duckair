#![allow(irrefutable_let_patterns)]

enum NavigationAids {
    Ndb,    // 0
    Vor,    // 1
    VorDme, // 2
}

/* enum NavigationAids {
    NDB = 5, // 5
    VOR, // 6
    VORDME, // 7
} */

/* enum NavigationAids {
    NDB, // 0
    VOR = 4, // 4
    VORDME, // 5
} */

/* enum NavigationAids {
    NDB = 3, // 3
    VOR = 2, // 2
    VORDME = 5, // 5
} */

/* enum NavigationAids {
    NDB = 3, // 3
    VOR = 2, // 2
    VORDME, // it doesn't compile because the compiler tries to set this to 3, which is already held by "NDB"
} */

fn main() {
    // if-else operator
    // same as in go

    // enum
    // it works similarly to "iota" in go
    println!("NDB:\t{}", NavigationAids::Ndb as u8); // "as u8" required as the compiler can't infer the type
    println!("VOR:\t{}", NavigationAids::Vor as u8);
    println!("VORDME:\t{}", NavigationAids::VorDme as u8);

    // match statement: it works like switch-case in other languages
    let animal = "Crocodile";
    match animal {
        "Duck" => println!("Quack"),
        "Dog" => println!("Woof"),
        _ => println!("No sound for {}", animal),
    }

    // more match statements
    let frequency: u16 = 201;
    match frequency {
        200..=500 => println!("{}Hz is in range", frequency), // "=500" means inclusive range. Exclusive range syntax is experimental as of 07-2023
        // the above statement can be rewritten as: frq if frq > 200 && frq <= 500 => println!("{}Hz is a valid frequency", frequency),
        _ => println!("{}Hz is out of range", frequency),
    }

    // option enum: used to express the existence or non-existence of a value - there's no null/nil in Rust!
    let idx = 6;
    let phrase = String::from("Hello World!");
    let letter = phrase.chars().nth(idx);
    match letter {
        Some(chr) => println!("Char at index {} is {}", idx, chr),
        None => println!("No character found at index {}", idx),
    }

    // it is possible to instantiate an Option via Some and None. None wants an explicit type
    let _some_one = Some(1);
    let one = 1;
    let two: Option<i32> = None;
    println!("The sum between 1 and None is: {}", one + two.unwrap_or(0)); // "unwrap" extracts Some from the Option

    /*
     * if let statement: the compiler tests if the assignment "if let ch = anodah_animoh"
     * can be executed.
     * 
     * To understand it, better read it backwards:
     * "if anodah_animoh matches ch => execute the code inside the block"
     */
    let anodah_animoh = "Cat";
    if let ch = anodah_animoh {
        /*
         * Irrefutable pattern: this will always be executed as "ch" can be assigned
         * to "anodah__animoh".
         *
         * In this case the rust-analyzer can evaluate the irrefutability of the pattern
         * at compile time, hence the warning coming from the IntelliSense.
         * The warning is silenced with the top-level statement:
         * "#![allow(irrefutable_let_patterns)]".
         */
        println!("Animal {} goes meow!", ch);
    }
    /*
     * The code inside this block will be executed as "Bacon" is contained inside the tuple.
     * However, at compile time this is not known yet, as the lookup of the tuple
     * is done at runtime.
     * For this reason the pattern is NOT deemed as irrefutable as in the previous "if let".
     */
    let dish = ("Bacon", "Ham", "Eggs");
    if let ("Bacon", b, c) = dish {
        println!("Bacon is served with {} and {}", b, c);
    }

    // loops
    // infinite loop
    /* loop {
        println!("You'll never get to the bottom of this!");
    } */
    // loops can return values
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };
    println!("counting stuff in the loop returns... {}", result);
    // while and for loops are pretty similar to other languages
    // while let works similarly to if let
    // for each
    let nums = [1, 2, 3, 4];
    // for each
    for num in nums { // you can also do "for num in nums.iter()"
        println!("{}", num);
    }
    // you can run for loops over ranges
    for num in 0..4 { // exclusive range (prints 1 to 4)
        println!("{}", num);
    }
}
