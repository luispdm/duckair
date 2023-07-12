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
    println!("the distance between {} and {} is {:.1} km", seg.start.name, seg.end.name, seg.distance())
}
