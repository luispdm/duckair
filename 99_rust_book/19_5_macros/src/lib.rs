/*
 * Macros are like functions: they take code in input and return code in output.
 * Writing macros allows you to reduce the amount of code you have to maintain.
 * There are a couple of key differences between functions and macros:
 * 1. functions must declare the number of parameter they accept; macros can accept
 * a variable number of parameters
 * 2. functions are called at runtime; macros are expanded before the code finishes compiling
 *
 * There are two types of macros: declarative macros and procedural macros:
 * declarative macros match against patterns and replace code with other code;
 * procedural macros take code as input, operate on that code and produce code as output.
 */

// declarative macro
// the code below is the implementation of the "vec!" macro of the std lib
#[macro_export]
macro_rules! vec_two {
    /*
     * The body of the declarative macro is similar to a match expression.
     * In this case, as we only have one match arm, if the code doesn't match "$x:expr",
     * an error will be thrown.
     * Note that macros can have multiple match arms.
     *
     * The match expression bellow means: match the expression in parentheses ("expr")
     * followed by "," between 0 or more times ("*") and assign it to the variable x ("$x:").
     * "expr" matches any Rust expression.
     */
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
            // when the macro is called with "vec_two![1, 2, 3]", "temp_vec.push($x)" is replaced with:
            // temp_vec.push(1);
            // temp_vec.push(2);
            // temp_vec.push(3);
        }
    }; // macro arms are separated by ";"
}

// procedural macros (see the library crates inside this folder for an example of procedural macros)
/*
 * Three kinds of procedural macros:
 * 1. custom-derived
 * 2. attribute-like
 * 3. function-like
 * All the types of procedural macros are defined using a similar syntax:
 *
 * use proc_macro; // bringing this crate into scope allows us to use "TokenStream"
 *
 * #[some_attribute] // this specifies the kind of macro we are creating
 * pub fn my_macro(input: TokenStream) -> TokenStream {} // my_macro is the name of my procedural macro
 *
 * Tokens are the smallest individual elements of a program, they can represent: keywords, identifiers,
 * operators, separators or literals.
 */

// attribute-like macros
/*
 * They are similar to custom-derived macros, but instead of generating code for the derived attribute,
 * they can create a custom attribute. Custom-derived macros only work on structs and enums, whereas
 * attribute-like macros work on other types such as functions.
 * Example (the code below won't compile as it needs to be in its own crate):
 *
 * #[route(GET, "/")]
 * fn index() {}
 *
 * #[proc_macro_attribute]
 * // attr is going to be: GET, "/"
 * // item is going to be: fn index() {}
 * pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {}
 */

// function-like macros
/*
 * Similar to functions, but they can take an arbitrary number of arguments and they operate on Rust code.
 * Example:
 *
 * let sql = sql!(SELECT * FROM posts WHERE id=1);
 * #[proc_macro]
 * pub fn sql(input: TokenStream) -> TokenStream {}
 */