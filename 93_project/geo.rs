const EARTH_RADIUS_KMS: f64 = 6371.0;

/*
 * in Rust the outer-most module inside a file takes the name of the file itself
 * for example, this module is called "geo"
 *
 * if "pub" is omitted the function becomes private
 */
pub fn calculate_distance(
    start_latitude: f64,
    start_longitude: f64,
    end_latitude: f64,
    end_longitude: f64,
) -> f64 {
    let delta_latitude = (start_latitude - end_latitude).to_radians();
    let delta_longitude = (start_longitude - end_longitude).to_radians();

    let start_latitude_radians = start_latitude.to_radians();
    let end_latitude_radians = end_latitude.to_radians();

    let inner_central_angle = f64::powi((delta_latitude / 2.0).sin(), 2)
        + start_latitude_radians.cos()
            * end_latitude_radians.cos()
            * f64::powi((delta_longitude / 2.0).sin(), 2);

    let central_angle = 2.0 * inner_central_angle.sqrt().asin();
    EARTH_RADIUS_KMS * central_angle
}

// this is how you declare a public submodule
pub mod printing {
    pub fn print_distance(distance: f64, start_airport: &str, end_airport: &str) {
        println!(
            "the distance between {} and {} airports is: {:.1} km",
            start_airport, end_airport, distance
        );
    }
}
