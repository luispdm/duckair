fn main() {
    // default & explicit types
    let def_num = 0; // signed integer 32
    let uint8_num: u8 = 0;
    let def_float = 0.0; // float 64
    let f32_num = 3.0_f32;
    let _unused_its_ok = "cargo doesn't raise any warning";
    println!("{} {} {} {}", def_num, uint8_num, def_float, f32_num);

    // casting
    let f_32: f32 = 3.0;
    let num = 15;
    let casting = num as f32;
    // println!("{}", num/f_32); // compiler error
    println!("{}", casting / f_32); // 5

    // mutability & immutability
    let _immutable = "i cannot change";
    let mut _mutable = "but i can";
    _mutable = "see";

    // scope & shadowing
    let scope = "learning scope & shadowing";
    println!("{}", scope);
    let scope = "shadowing my variable in the same block";
    println!("{}", scope); // "shadowing my variable in the same block"
    let scope = "my variables lives on in an inner block";
    {
        println!("{}", scope); // "my variables lives on in an inner block"
        let scope = "shadowing my variable in an inner block";
        println!("{}", scope); // "shadowing my variable in an inner block"
    }
}
