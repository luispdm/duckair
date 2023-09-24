#![allow(unused_imports)]
// import Rng only
use rand::Rng;
// import with nested paths
use rand::{CryptoRng, SeedableRng};
// import all
use rand::*;

fn main() {
    let mut rng = rand::thread_rng();
    let rn: f64 = rng.gen();
    println!("{}", rn);
    // VSCode IntelliSense doesn't recognize the crate but the code compiles
    // no need to specify "use lib_demo_luispdm" on top as the library has already
    // been declared in the Cargo.toml file
    println!("{}", lib_demo_luispdm::add_one::add_one(1));
}
