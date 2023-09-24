// thanks to the "pub use self..." statements in lib.rs,
// main doesn't have to include the name of the submodule (add_one)
// between the crate (lib_demo_luispdm) and the item (Dummy)
use lib_demo_luispdm::Dummy;

fn main() {
    println!("{:?}", Dummy{});
}
