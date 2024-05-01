use std::borrow::BorrowMut;

fn main() {
    // matching on literals - see 06_control_flow

    //matching on named variables
    let t = Some(5);
    let u = 10;
    match t {
        Some(50) => println!("five"),
        // u is shadowed
        Some(u) => println!("Matched: {u}"),
        _ => println!("underscore"),
    }

    // matching on multiple patterns
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        _ => println!("a"),
    }

    // matching ranges of values - see 06_control_flow
    // you can match range of values for numeric and literal values only

    // match expressions destructuring structs
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 1 };
    // i don't necessarily need to reassign the struct fields
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(1, b);
    match p {
        Point { x, y: 0 } => println!("on the x axis: {x}"),
        Point { x: 0, y } => println!("on the y axis: {y}"),
        Point { x, y } => println!("on neither axis: {x}, {y}"),
    }

    // match expressions destructuring enums
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
        AnotherPalette(ComplexChange),
    }
    #[derive(Debug)]
    enum ComplexChange {
        Ycmk(i32, i32, i32, i32),
        IMadeThisUp(String),
    }
    let msg = Message::AnotherPalette(ComplexChange::Ycmk(1, 2, 3, 4));
    match msg {
        Message::Quit => println!("quit"),
        Message::ChangeColor(r, g, b) => println!("{r}, {g}, {b}"),
        Message::AnotherPalette(ComplexChange::Ycmk(y, c, m, k)) => {
            println!("it's ycmk: {y}, {c}, {m}, {k}");
        }
        Message::AnotherPalette(ComplexChange::IMadeThisUp(strr)) => {
            println!("i made this up: {strr}");
        }
        smth => println!("something else: {:?}", smth),
    }

    // more complex destructuring match expressions
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: 10 });
    println!("{feet}, {inches}, {x}, {y}");

    // ignoring values
    // useful to ignore a function parameter when a fn must match the signature of another fn
    fn foo(_: i32, h: i32) {
        println!("{h}");
    }

    // ignoring tuples with Options
    let mut setting_val = None;
    let new_setting_val = Some(19);
    match (setting_val, new_setting_val) {
        (Some(10), Some(_)) => setting_val = Some(0),
        (Some(_), Some(_)) => println!("can't overwrite a value other than 10"),
        _ => setting_val = new_setting_val,
    }
    println!("{:?}", setting_val); // Some(19)

    // more ignore syntax in tuples
    // note that some of the patterns below are unreachable
    let nums = (1, 2, 3, 4, 5);
    match nums {
        (first, _, third, _, _) => println!("{first}, {third}"),
        // ".." is the range syntax
        (first, ..) => println!("capturing {first} only"),
        (.., last) => println!("capturing {last} only"),
        (first, .., last) => println!("capturing {first} and {last} only"),
        // (.., second, ..) => (), // it does not compile as Rust doesn't know what "second" binds to
        _ => println!(),
    }

    // ignored variables and binding
    let some_str = Some(String::from("hey"));
    if let Some(s) = some_str {
        println!("i don't care about the value inside s");
    }
    // the code below does not compile because even though "s" is not used in the
    // if let statement above, ownership of "some_str" is still moved inside "s"
    // println!("{:?}", some_str);
    // if I ignore the variable in the if let statement above (i.e. "Some(_)"), the code compiles

    // match guards
    let new_num = Some(10);
    match new_num {
        Some(xx) if xx < 5 => println!("less than 5"),
        _ => (),
    }
    let guard = true;
    match new_num.unwrap_or_default() {
        // the condition on guard applies to both 4 and 10
        4 | 10 if guard => println!("print only IF new_num is 4 OR 10 AND guard is true"),
        _ => (),
    }

    // matching with @ operator
    enum Msg {
        Hello { id: i32 },
    }
    let message = Msg::Hello { id: 50 };
    match message {
        // check if id is within range, capture the value ("@") and print it
        Msg::Hello { id: id_var @30..=70 } => println!("{id_var} in range"),
        // check if id is within range but don't capture the value
        Msg::Hello { id: 10..=12 } => println!("in another range"),
        // "catch-all" capturing the value
        Msg::Hello { id } => println!("another {id}"),
    }
}
