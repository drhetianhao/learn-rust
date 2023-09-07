
fn main() {
    let kcle_latitude_degrees: f64 = 41.4075;
    let kcle_longitude_degrees: f64 = -81.851111;
    let kslc_latitude_degrees: f64 = 40.7861;
    let kslc_longitude_degrees: f64 = -111.9822;

    let distance = geo::distance(kcle_latitude_degrees,
    kcle_longitude_degrees,
    kslc_latitude_degrees,
    kslc_longitude_degrees);

    println!(
        "The distance between the two points is {:.1} kilometers", distance);
}

mod geo {
    const EARTH_RADIUS_IN_KILOMETERS: f64 = 6371.0;

    pub(crate) fn distance(start_latitude: f64, start_longitude: f64,
                           end_latitude: f64, end_longitude: f64) -> f64 {
        let start_latitude_radians = start_latitude.to_radians();
        let end_latitude_radians = end_latitude.to_radians();
        let delta_latitude = (start_latitude - end_latitude).to_radians();
        let delta_longitude = (start_longitude - end_longitude).to_radians();

        let inner_central_angle = f64::powi((delta_latitude / 2.0).sin(),2)
        + start_latitude_radians.cos() * end_latitude_radians.cos()
        * f64::powi((delta_longitude / 2.0).sin(),2);
        let central_angle = 2.0 * inner_central_angle.sqrt().asin();
        let distance = EARTH_RADIUS_IN_KILOMETERS * central_angle;
        distance
    }
}