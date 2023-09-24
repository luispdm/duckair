/*
 * Example of documentation code. The documentation code is compiled as HTMl by cargo
 * when you run `cargo doc`.
 * `cargo doc --open` opens the documentation file in a web browser.
 *
 * The following documentation code documents the item containing the comment
 * (the crate itself "lib_demo_luispdm") instead of the item following the comment.
 */

//! # lib_demo_luispdm
//!
//! `lib_demo_luispdm` is an example of documenting a library crate

/*
 * The following statements make the functions and struct documentations
 * available from the index page of the docs. The users reading the docs
 * don't have to first click on the module name and then on the function/struct
 * to see the related documentation.
 * This section of the index page is be called "re-exports".
 */
pub use self::add_one::add_one;
pub use self::add_one::Dummy;
pub use self::add_two::add_two;

pub mod add_one {
    /*
     * The documentation below includes examples on how to use the functions.
     * Rust runs those examples as documentation tests when you type `cargo test`.
     * You can run documentation tests only with: `cargo test --doc`.
     * 
     * Beside examples, there are other common sections that one usually introduces in
     * their crate: Errors, Panics and Safety (when unsafe Rust is implemented).
     */

    /// Adds one to the given number.
    ///
    /// # Examples
    ///
    /// ```
    /// let arg = 5;
    /// let answer = lib_demo_luispdm::add_one::add_one(arg);
    ///
    /// assert_eq!(6, answer);
    /// ```
    pub fn add_one(x: u64) -> u64 {
        x+1
    }
    #[derive(Debug)]
    pub struct Dummy {

    }
}

pub mod add_two {
    pub fn add_two(x: u64) -> u64 {
        x+2
    }
}
