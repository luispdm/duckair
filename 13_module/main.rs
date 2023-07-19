use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let rn: f64 = rng.gen();
    println!("{}", rn);
}
