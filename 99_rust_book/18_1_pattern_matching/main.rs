fn main() {
    // similar examples can be found at 06_control_flow

    // patterns are a special syntax for matching against the structure of types
    // they can match: literals, destructured (arrays, enums, structs, tuples),
    // variables, wildcards and placeholders

    // the code below shows all the scenarios where patterns are used

    // match expressions
    #[derive(Debug)]
    enum Language {
        English,
        Spanish,
        Italian,
    }
    let lang = Language::English;
    // as you may already know, with match expressions patterns must be exhaustive
    match lang {
        Language::English => println!("hello"),
        Language::Spanish => println!("hola"),
        // this is how you use the variable in the "catch-all" pattern
        lang => println!("unsupported language: {:?}", lang),
    }
    // if you want to match against one pattern only, you can use the "matches!" macro
    // or the if let statement
    let lang = Language::Italian;
    println!("{}", matches!(lang, Language::Spanish)); // false
    if let Language::Spanish = lang {
        println!("hola");
    }

    // if let: patterns don't have to be exhaustive here, we've omitted the "else" block
    let auth_status: Option<&str> = None;
    let is_admin = false;
    if let Some(auth_status) = auth_status {
        // auth_status shadowed
        println!("auth status: {}", auth_status);
    } else if is_admin {
        println!("auth status: admin");
    }

    // while let
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("{top}");
    }

    // for loops
    let vec = vec![1, 2, 3];
    for (i, v) in vec.iter().enumerate() {
        println!("{v} is stored at index {i}");
    }

    // let statements
    let (x, y, _) = (1, 2, 3); // ignoring the last tuple value
    // the code below does not compile as the pattern doesn't match
    // (i am trying to assign two values to a tuple of three)
    // let (x, y) = (1, 2, 3);

    // function parameters (it works with closures too)
    fn print_coords(&(x, y): &(i32, i32)) {
        println!("the coordinates are {x} and {y}");
    }
    let point = (3, 5);
    print_coords(&point);

    // refutable & irrefutable patterns
    // irrefutable: this pattern will always match (more at 06_control_flow)
    let x = 5;
    // refutable
    let x: Option<&str> = None;
    if let Some(x) = x {
        println!("{x}");
    }
    // function parameters, let statements and for loops can only accept
    // irrefutable patterns, because the program cannot do anything meaningful
    // if the patterns don't match.
    // the code below does not compile as the assignment never matches (x is None)
    // let Some(y) = x;
    // if let and while let accept both refutable and irrefutable patterns.
    // in these cases, irrefutable patterns give warning: if let and while let
    // are useless if the pattern always matches, you can simply use "let"
    if let z = 5 {
        println!("{z}");
    }
}
