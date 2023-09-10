use std::fmt::Display;

trait Summary {
    // traits can have default implementations
    fn summarize(&self) -> String {
        String::from("read more...")
    }
    // default implementations can call other methods of the trait
    fn summarize_with_author(&self) -> String {
        format!("read more... from {}", self.author())
    }
    fn author(&self) -> &String;
}

struct Tweet {
    pub username: String,
    pub tweet: String,
    pub replies: Vec<String>,
}

struct Article {
    pub author: String,
    pub headline: String,
}

impl Summary for Article {
    fn author(&self) -> &String {
        &self.author
    }
}

impl Summary for Tweet {
    // Only the implementations of Summary here.
    // If you want to define other methods for Tweet,
    // they must be wrapped inside another "impl Tweet".

    // default implementations can be overridden
    fn summarize(&self) -> String {
        format!("{} tweeted: {}", self.username, self.tweet)
    }
    fn author(&self) -> &String {
        &self.username
    }
}

trait AppendBar {
    fn append_bar(self) -> Self;
}

// Rust gives the possibility to implement traits for types of the standard library too
impl AppendBar for String {
    fn append_bar(self) -> Self {
        format!("{self}Bar")
    }
}

// This is how polymorphism is achieved in Rust.
// It's syntactic sugar of what's below
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// this syntax is called "trait bound"
fn notify_tb<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Sometimes that syntactic sugar is not powerful enough.
// What if I want arg1 and arg2 to be exactly of the same type?
// This is something I can only specify with trait bounds
fn not_same_type(arg1: &impl Summary, arg2: &impl Summary) {
    
}
fn same_type<T: Summary>(arg1: &T, arg2: &T) {
    
}

// more on trait bounds at 11_generics

// polymorphism pt.2 - this is very useful in closures and iterators
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch{
        Article {
            author: String::from("Deux Jean"),
            headline: String::from("C'est la vie"),
        }
    } else {
        /*
         * The code below doesn't compile! The return type
         * must be the same in all the function body.
         * Different return types can be specified only
         * via trait objects, which we will cover in chapter 17
         */
        // Tweet {
        //     username: String::from("@johndoe"),
        //     tweet: String::from("fake news... fake news everywhere!"),
        //     replies: vec![],
        // }
        Article {
            author: String::from("Juan Dos"),
            headline: String::from("Ojalá!"),
        }
    }
}

/*
 * Blanket implementation.
 * Implement a trait on a type that implements another trait.
 * It is widely used in the standard library.
 * The below is an example from the standard library, in fact,
 * it doesn't compile locally.
 */
// impl<T: Display> ToString for T {
// }

fn main() {
    let article = Article {
        author: String::from("John Pulitzer"),
        headline: String::from("It's the end of the world as we know it"),
    };
    let tweet = Tweet {
        username: String::from("@johndoe"),
        tweet: String::from("fake news... fake news everywhere!"),
        replies: vec![],
    };
    not_same_type(&article, &tweet)
    // same_type(&article, &tweet); // doesn't compile!
}
