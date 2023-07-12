struct Waypoint {
    name: String,
    latitude: f64,
    longitude: f64
}

struct Segment {
    start: Waypoint,
    end: Waypoint,
}

fn main() {
    let mut kcle = Waypoint {
        name: "KCLE".to_string(),
        latitude: 41.4075,
        longitude: -81.851111
    };
    let mut kslc = Waypoint {
        name: "KSLC".to_string(),
        ..kcle // copying fields from kcle. "name" will be overridden by the statement above though
    };
}
