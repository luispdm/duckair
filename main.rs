fn main() {
    const EARTH_RADIUS_KMS: f64 = 6371.0;

    let cle_latitude_degrees:f64 = 41.4075;
    let cle_longitude_degrees:f64 = -81.851111;
    let slc_latitude_degrees:f64 = 40.7861;
    let slc_longitude_degrees:f64 = -111.9822;

    let delta_latitude = (cle_latitude_degrees - slc_latitude_degrees).to_radians();
    let delta_longitude = (cle_longitude_degrees - slc_longitude_degrees).to_radians();

    let cle_latitude_radians = cle_latitude_degrees.to_radians();
    let slc_latitude_radians = slc_latitude_degrees.to_radians();

    let inner_central_angle = f64::powi((delta_latitude / 2.0).sin(), 2)
        + cle_latitude_radians.cos()
        * slc_latitude_radians.cos()
        * f64::powi((delta_longitude / 2.0).sin(), 2);
    
    let central_angle = 2.0 * inner_central_angle.sqrt().asin();
    let distance = EARTH_RADIUS_KMS * central_angle;
    println!("the distance between CLE and SLC airports is: {:.1} km", distance);
}
