mod modules; // thanks to this statement, "use modules::*;" would be redundant

fn main() {
    modules::outer::inner::print_smth();
    // even though we are using an absolute path here, "mod modules" on top of the file is still required
    crate::modules::print_four_times();
    modules::eat();
}
