#![allow(unused_imports)]
use std::{
    error::Error,
    fs::File,
    io::{ErrorKind, Read},
};

fn main() {
    println!("{}", return_greater(1, 2));

    let mut original = String::from("initial value");
    println!("{}", original);
    print_var(&original);
    println!("Can use original again: {}", original); // original holds the value again as "print_var" returned and the vars inside its scope got destroyed
    change_var(&mut original);
    println!("Has original changed? Let's see: {}", original); // yes, it has

    // closures
    let thanos = String::from("I am");
    let closure = |s: &String| -> String {
        format!("{}... inevitable!", s)
    };
    println!("{}", closure(&thanos));
    // closures can also be used directly instead of assigning them to a variable
    let check = String::from("\"anonymous");
    println!("{}", |s: &String| -> String {
        format!("{} closure\"", s)
    }(&check));

    // errors
    // panic!("panic"); // this is how you panic
    // panic_vec() // this is how you panic pt.2: "index out of bounds"

    // error handling with Result Enum - UNCOMMENT IF YOU WANT MAIN TO CRASH
    /* let path = "/impossiblethatthispathexists/i/do/not/exist";
    match File::open(path) {
        Ok(f) => {
            println!("{:#?}", f);
        }
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create(path) { // trying to recover the error by creating the file
                Ok(fc) => {
                    println!("{:#?}", fc);
                }
                Err(e) => {
                    println!("cannot create file because: {:#?}", e);
                }
            },
            _ => {
                println!("cannot open file because: {:#?}", err);
            }
        },
    } */
    // less-verbose alternative
    // let f = File::open("my/path").unwrap_or_else(|err| {
    //     if err.kind() == ErrorKind::NotFound {
    //         File::create("my/path").unwrap_or_else(|err| {
    //             panic!("Impossible to create the file");
    //         })
    //     } else {
    //         panic!("Impossible to open the file");
    //     }
    // });
    // if the only code to be executed is just a panic, "unwrap" can be used instead of "unwrap_or_else"

    // error propagation
    let path = "08_functions/dummy.md";
    let data = read_file(path);
    match data {
        Ok(d) => println!("{:#?}", d),
        Err(e) => println!("{:#?}", e),
    }
}

// pass by value
fn return_greater(n1: u8, n2: u8) -> u8 {
    if n1 > n2 {
        return n1;
    }
    n2
}

// pass by reference - borrowing the value from the parameter passed in
fn print_var(s: &String) {
    println!("fn prints {}", s);
}

// pass by value - if this is called then "original" in main() can't be used anymore, as the ownership got transferred to "s" (fn param)
/* fn print_original(s: String) {
    println!("fn prints {}", s);
} */

// pass by reference - borrowing for read and write (mutability)
fn change_var(s: &mut String) {
    *s = String::from("I have become death, destroyer of worlds");
}

fn _panic_vec() {
    let v = vec![1, 2];
    println!("{}", v[5]);
}

// TODO need to know what "Box" and "dyn" are used for
fn read_file(p: &str) -> Result<String, Box<dyn Error>> {
    // "?" makes the function return if "open" fails. Same thing for "read_to_string"
    let mut handle = File::open(p)?;
    let mut data = String::new();
    handle.read_to_string(&mut data)?;
    Ok(data)
}
