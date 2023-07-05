fn main() {
    const EARTH_RADIUS_KMS: f64 = 6371.0;

    // some waypoints missing from the route
    let route = [
        ("KCLE", 41.4075, -81.851111),
        ("BRYTO", 41.74170, -85.51130),
        ("GIJ", 41.76860, -86.31850),
        ("NEPTS", 41.96750, -87.05300),
        ("THORR", 42.12330, -87.60030),
        ("OBK", 42.22140, -87.95160),
        ("COTON", 42.31990, -89.31220),
        ("DBQ", 42.40150, -90.70910),
        ("VIGGR", 42.55520, -93.12410),
        ("KSLC", 40.7861, -111.9822),
    ];

    let mut total_distance = 0.0;
    let mut previous_waypoint: Option<(&str, f64, f64)> = None;

    for waypoint in route {
        match previous_waypoint {
            None => {
                previous_waypoint = Option::from(waypoint);
                continue;
            }
            Some(previous_waypoint_value) => {
                let previous_waypoint_radians = previous_waypoint_value.1.to_radians();
                let waypoint_radians = waypoint.1.to_radians();

                let delta_latitude = (previous_waypoint_value.1 - waypoint.1).to_radians();
                let delta_longitude = (previous_waypoint_value.2 - waypoint.2).to_radians();

                let inner_central_angle = f64::powi((delta_latitude / 2.0).sin(), 2)
                    + previous_waypoint_radians.cos()
                        * waypoint_radians.cos()
                        * f64::powi((delta_longitude / 2.0).sin(), 2);

                let central_angle = 2.0 * inner_central_angle.sqrt().asin();
                let distance = EARTH_RADIUS_KMS * central_angle;

                total_distance += distance;
                previous_waypoint = Option::from(waypoint);

                println!(
                    "The distance between {} and {} is {:.1} kilometers",
                    previous_waypoint_value.0, waypoint.0, distance
                );
            }
        }
    }
    println!("Total distance is {:.1} kilometers", total_distance);
}
