/*
 * Here I am telling the compiler there is a module called "modules"
 * which lives in a file called "modules.rs"
 * 
 * The name of the module and the name of the file must match
 */
mod modules; // thanks to this statement, "use modules::*;" is not needed

fn main() {
    modules::outer::inner::print_smth();
    // even though we are using an absolute path here, "mod modules" on top of the file is still required
    crate::modules::print_four_times();
    modules::eat();
    modules::hosting::hosting();
}
