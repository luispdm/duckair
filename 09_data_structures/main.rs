#![allow(dead_code)]
#![allow(unused_variables)]

struct Waypoint {
    name: String,
    latitude: f64,
    longitude: f64
}

struct Segment {
    start: Waypoint,
    end: Waypoint,
}

// in Rust, you don't write methods inside a struct, you write methods and you associate them this way
// associative methods
impl Segment {
    fn new(start: Waypoint, end: Waypoint) -> Self {
        Self {
            start, end
        }
    }

    fn distance(&self) -> f32 {
        println!("calculating distance between {} and {}...", self.start.name, self.end.name);
        2513.0
    }
}

fn main() {
    // structs must also be declared mutable with the "mut" keyword
    let kcle = Waypoint {
        name: "KCLE".to_string(),
        latitude: 41.4075,
        longitude: -81.851111
    };
    let kslc = Waypoint {
        name: "KSLC".to_string(),
        latitude: 40.7861,
        longitude: -111.9822
    };
    let kcle_copy = Waypoint {
        name: "KCLE_COPY".to_string(),
        ..kcle // copying fields from kcle. "name" will be overridden by the statement above
    };
    let seg = Segment::new(kcle, kslc);
    println!("the distance between {} and {} is {:.1} km", seg.start.name, seg.end.name, seg.distance());

    let a_380 = Airbus {
        name: "VY6605".to_string(),
        available_crew: 10,
        fuel_range: 2000
    };
    let b_747 = Boeing {
        name: "FR9090".to_string(),
        available_crew: 2,
        fuel_range: 100
    };
    println!("is flight {} legal?\t{}", a_380.name, a_380.is_legal(5, 1000));
    println!("is flight {} legal?\t{}", b_747.name, b_747.is_legal(5, 1000));
}

// traits are the equivalent to interfaces in OO languages
trait Flight {
    fn is_legal(&self, required_crew: u8, distance: u16) -> bool;
}

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
