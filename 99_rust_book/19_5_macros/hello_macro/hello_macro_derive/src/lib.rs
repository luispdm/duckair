extern crate proc_macro;

// the trait HelloMacro must be defined in the parent crate because procedural macro crates
// cannot export items other than procedural macros themselves (i.e. functions tagged with
// #[proc_macro], #[proc_macro_derive], or #[proc_macro_attribute])

// Rust doesn't have reflective capabilites, so in order to print
// the type on which the trait was implemented on, i need the quote and syn crates
use proc_macro::TokenStream;
use syn; // syn allows us to take a string of Rust code and turn it into a syntax tree data structure
use quote::quote; // this takes a syntax tree data structure and turns it back into Rust code

#[proc_macro_derive(HelloMacro)] // i.e. this is a custom-derived macro with the name "HelloMacro"
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // construct a representation of Rust code as a syntax tree that we can manipulate
    let ast = syn::parse(input).unwrap();
    // build the trait implementation
    impl_hello_macro(&ast)
}

// building the TokenStream back from the syntax tree and transforming it into what we want
// (i.e. the trait implementation)
fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        // implementing the trait defined in the parent crate "hello_macro" for the type "name"
        // "#name" will be replaced with the name of the type which this function is called on
        impl HelloMacro for #name {
            fn hello_macro() {
                // "stringify!" takes an expression and turns it into a string
                // unlike the "format!" macro though, it won't evaluate the expression!
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into() // gen is now turned into a TokenStream
}
