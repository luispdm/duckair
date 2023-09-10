use std::fmt::Display;
// more lifetime basics at "07_ownership_borrowing"

/*
 * The three rules of lifetimes:
 *
 * 1. Each parameter that is a reference gets its own lifetime parameter
 * 2. If there is exactly one input lifetime parameter, that lifetime is
 *    assigned to all output lifetime parameters
 * 3. If there are multiple input lifetime parameters but one of them is
 *    &self or &mut self, the lifetime of self is assigned to all output
 *    lifetime parameters
 *
 * input lifetime parameter  = parameter of a function which holds
 * a lifetime
 * output lifetime parameter = return reference of a function which holds
 * a lifetime
 */

// how to specify lifetimes in different forms
// &i32        - a reference
// &'a i32     - a reference with an explicit lifetime
// &'a mut i32 - a mutable reference with an explicit lifetime

/*
 * 'a does not modify the lifetime of each parameter and returned reference,
 * it is just declaring that there is a relationship between the lifetimes
 * of the parameters and the returned reference.
 *
 * The relationship is: "the lifetime of the returned reference will be
 * the same as the smallest lifetime of the function parameters"
 * E.g. if x goes out of scope before y, the lifetime of the retured reference
 * will be that of x
 */
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let str1 = String::from("ab");
    /*
     * The below does not compile because result might be assigned to str2
     * ("might" because the borrow checker doesn't know at compile time yet),
     * and result might be used after str2 goes out of scope => dangling reference
     */

    // let result: &str;
    // {
    //     let str2 = String::from("b");
    //     result = longest(&str1, &str2);
    // }
    // println!("{}", result);

    // the below does compile because result has the same scope as of str2
    // (the parameter with the smallest lifetime)
    let ann = String::from("public announcement");
    {
        let str2 = String::from("b");
        let result = longest(&str1, &str2);
        println!("{}", result);
        longest_with_announcement(&str1, &str2, &ann);
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let i: ImportantExcerpt;
    {
        let first_sentence = String::from(novel
            .split('.')
            .next()
            .expect("Could not find the first sentence"));
        i = ImportantExcerpt {
            part: &first_sentence,
        };
        // "i" cannot be used outside of this block as we would have a
        // dangling reference because "first_sentence" was declared
        // inside this block (similar example to the usage of "longest")
        println!("{}", i.return_part(&ann));
    }
    // it does not compile!
    // println!("{}", i.return_part(&ann));

    /*
     * &'static = the reference can live as long as the duration of the program
     * All string literals have a static lifetime because string literals are
     * stored in the program binary.
     */
    let _s: &'static str = "static";
}

// lifetimes in structs - the following struct cannot outlive the reference passed to part
// try not to resource to lifetimes in structs as a first solution, try using owned data
// as the program will be easier to read, to work with and to refactor
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // no need to specify a lifetime because rule #3 is respected (see the top of the file)
    fn return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// putting it all together: generics, traits, trait bounds and lifetimes
fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Attention please: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
