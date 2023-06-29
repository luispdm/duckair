fn main() {
    let location: [f32; 2] = [0.0, 0.0];
    let (place, lat, long) = ("House", 41.0, -57.0);

    println!("Hello {:?}", location);
    println!("My {} is at {}, {}", place, lat, long);

    let immutable: &str = "I am an &str";
    let mutable: String = format!("and {} too... whoops, my bad! I am a String", immutable);
    println!("{} {}", immutable, mutable);
}
