fn main() {
    // arrays
    let location: [f32; 2] = [0.0, 0.0];
    let _location_two: [f32; 2] = [0.0; 2]; // array with two elements, each = 0.0

    // tuples
    let tup = ("House", 41.0, -57.0);
    // there are two ways to retrieve values from tuples
    let (place, lat, long) = tup; // destructuring
    let _place_two = tup.0; // dot notation
    println!("Hello {:?}", location);
    println!("My {} is at {}, {}", place, lat, long);

    // strings
    let a_str: &str = "I am an &str";
    let a_string: String = format!("and {} too... whoops, my bad! I am a String", a_str);
    println!("{} {}", a_str, a_string);
    // characters
    let a_char = 'a';
    println!("Chars are defined with '' like in go: {}", a_char);

    // numbers
    let _big_num = 500_000_000; // big numbers can use "_" as separators for better readability
    let _b = 0b1111_0000; // binary
    let _h = 0xff; // hexadecimal
    let _o = 0o77; // octal
    let _y = b'y'; // byte
}
