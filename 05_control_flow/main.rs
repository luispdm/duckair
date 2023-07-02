enum NavigationAids {
    NDB, // 0
    VOR, // 1
    VORDME, // 2
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
    println!("NDB:\t{}", NavigationAids::NDB as u8); // "as u8" required as the compiler can't infer the type
    println!("VOR:\t{}", NavigationAids::VOR as u8);
    println!("VORDME:\t{}", NavigationAids::VORDME as u8);
}
