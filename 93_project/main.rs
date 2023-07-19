mod geo;
use geo::calculate_distance;
use geo::printing::print_distance as pd; // aliasing: useful to avoid collisions

fn main() {
    let cle_latitude_degrees: f64 = 41.4075;
    let cle_longitude_degrees: f64 = -81.851111;
    let slc_latitude_degrees: f64 = 40.7861;
    let slc_longitude_degrees: f64 = -111.9822;

    let distance = calculate_distance(
        cle_latitude_degrees,
        cle_longitude_degrees,
        slc_latitude_degrees,
        slc_longitude_degrees,
    );
    pd(distance, "CLE", "SLC");
}
