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
}
