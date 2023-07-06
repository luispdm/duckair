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

    // if let statement: the compiler tests if the variable on the left can be assigned to to the one on the right
    // if it can, the pattern is irrefutable and the code in the block is executed
    let anodah_animoh = "Cat";
    if let ch = anodah_animoh {
        println!("Aniaml {} goes meow!", ch); // irrefutable pattern: this will always be executed as "ch" can be assigned to "anodah__animoh"
    }
    // this pattern is not irrefutable, as "Bacon" is not contained inside "dish", therefore the code inside the block is not executed
    let dish = ("Ham", "Eggs");
    if let ("Bacon", b) = dish {
        println!("Bacon is served with {}", b);
    }

    // loops
    // infinite loop
    /* loop {
        println!("You'll never get to the bottom of this!");
    } */
    // while and for loops are pretty similar to other languages
    // while let works similarly to if let
    // for each
    let nums = [1, 2, 3, 4];
    for num in nums {
        println!("{}", num);
    }
}
