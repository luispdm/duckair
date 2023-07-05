fn main() {
    let location: [f32; 2] = [0.0, 0.0];
    let (place, lat, long) = ("House", 41.0, -57.0);

    println!("Hello {:?}", location);
    println!("My {} is at {}, {}", place, lat, long);

    let a_str: &str = "I am an &str";
    let a_string: String = format!("and {} too... whoops, my bad! I am a String", a_str);
    println!("{} {}", a_str, a_string);
}
