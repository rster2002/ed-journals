/// Calculates the distance between two coords using the give planet radius, which can be given in
/// any scale (so you can use meters or kilometers and the result will use the same scale)
pub fn planet_distance(radius: f32, coord1: (f32, f32), coord2: (f32, f32)) -> f32 {
    let difference_lat = (coord1.0 - coord2.0).to_radians();
    let difference_long = (coord1.1 - coord2.1).to_radians();

    let a = (difference_lat / 2.0).sin().powi(2)
        + coord1.0.to_radians().cos()
            * coord2.0.to_radians().cos()
            * (difference_long / 2.0).sin().powi(2);

    let c = 2.0 * a.sqrt().atan2(1.0 - a.sqrt());
    radius * c
}

#[cfg(test)]
mod tests {
    use crate::galaxy::functions::planet_distance::planet_distance;

    #[test]
    fn distance_is_calculated_correctly() {
        let distance = planet_distance(
            6378100.0,
            (52.262_42, 4.871_265_4),
            (52.263_485, 4.873_533_2),
        );

        assert_eq!(distance, 194.70883);
    }
}
