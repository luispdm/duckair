#![allow(dead_code)]
#![allow(unused_variables)]

struct Waypoint {
    name: String,
    latitude: f64,
    longitude: f64,
}

struct Segment {
    start: Waypoint,
    end: Waypoint,
}

// unit-like structs, no fields: https://doc.rust-lang.org/book/ch05-01-defining-structs.html#unit-like-structs-without-any-fields
struct Unit;

// in Rust, you don't write methods inside a struct, you write methods and you associate them this way
impl Segment {
    // this is called method because &self is the first parameter, "dot notation" (i.e. an instance is required to use the method)
    fn distance(&self) -> f32 {
        println!(
            "calculating distance between {} and {}...",
            self.start.name, self.end.name // automatic referencing and dereferencing in Rust => don't need a special syntax to use "&self"
        );
        2513.0
    }
}

// there can be multiple implementation blocks for a struct
impl Segment {
    // associated functions and not methods because &self is not the first parameter, :: notation (i.e. Segment::new, Segment::build_segment)
    fn new(start: Waypoint, end: Waypoint) -> Self {
        Self { start, end }
    }
    // alternative constructor
    fn build_segment(start: Waypoint, end: Waypoint) -> Segment {
        Segment { start, end } // field initialization shorthand syntax
    }
}

fn main() {
    // structs can also be declared mutable. You can't make a single field of the struct mutable,
    // the whole struct must be mutable
    let kcle = Waypoint {
        name: "KCLE".to_string(),
        latitude: 41.4075,
        longitude: -81.851111,
    };
    let kslc = Waypoint {
        name: "KSLC".to_string(),
        latitude: 40.7861,
        longitude: -111.9822,
    };
    let kcle_copy = Waypoint {
        name: "KCLE_COPY".to_string(),
        // "update syntax". copying fields from kcle. "name" will be overridden by the statement above
        // the update syntax MUST ALWAYS be the last field inside the struct construction
        ..kcle
    };
    let seg = Segment::new(kcle, kslc);
    println!(
        "the distance between {} and {} is {:.1} km",
        seg.start.name,
        seg.end.name,
        seg.distance()
    );

    let a_380 = Airbus {
        name: "VY6605".to_string(),
        available_crew: 10,
        fuel_range: 2000,
    };
    let b_747 = Boeing {
        name: "FR9090".to_string(),
        available_crew: 2,
        fuel_range: 100,
    };
    println!(
        "is flight {} legal?\t{}",
        a_380.name,
        a_380.is_legal(5, 1000)
    );
    println!(
        "is flight {} legal?\t{}",
        b_747.name,
        b_747.is_legal(5, 1000)
    );
    println!("{:#?}", a_380);

    // tuple structs - despite having the same fields, these are two separate types
    struct One(i32, String);
    struct Two(i32, String);

    let _unit = Unit; // this is how a unit-like struct is instantiated
}

// traits are the equivalent to interfaces in OO languages
trait Flight {
    fn is_legal(&self, required_crew: u8, distance: u16) -> bool;
}

/*
 * telling the compiler to provide the implementation of Debug for Airbus,
 * so instances of this struct can be printed
 */
#[derive(Debug)]
struct Airbus {
    name: String,
    available_crew: u8,
    fuel_range: u16,
}

struct Boeing {
    name: String,
    available_crew: u8,
    fuel_range: u16,
}

impl Flight for Airbus {
    fn is_legal(&self, required_crew: u8, distance: u16) -> bool {
        (self.available_crew >= required_crew) && (self.fuel_range + 150 >= distance)
    }
}

impl Flight for Boeing {
    fn is_legal(&self, required_crew: u8, distance: u16) -> bool {
        (self.available_crew >= required_crew) && (self.fuel_range + 280 >= distance)
    }
}
