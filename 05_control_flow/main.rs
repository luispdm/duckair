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

    // option enum: used to express the existence or non-existence of a value - there's no null/nil in Rust!
    let idx = 6;
    let phrase = String::from("Hello World!");
    let letter = phrase.chars().nth(idx);
    match letter {
        Some(chr) => println!("Char at index {} is {}", idx, chr),
        None => println!("No character found at index {}", idx),
    }
}
