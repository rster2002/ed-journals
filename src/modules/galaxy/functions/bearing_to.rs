/// Calculates the bearing degrees going from the origin coordinates to the target coordinates.
pub fn bearing_to(origin: (f32, f32), target: (f32, f32)) -> f32 {
    let lat1 = origin.0.to_radians();
    let long1 = origin.1.to_radians();
    let lat2 = target.0.to_radians();
    let long2 = target.1.to_radians();

    let difference_long = long2 - long1;
    let bearing = (difference_long.sin() * lat2.cos()).atan2(lat1.cos() * lat2.sin() - lat1.sin() * lat2.cos() * difference_long.cos()).to_degrees();

    if bearing < 0.0 {
        (bearing + 360.0) % 360.0
    } else {
        bearing
    }
}

#[cfg(test)]
mod tests {
    use crate::galaxy::functions::bearing_to::bearing_to;

    #[test]
    fn bearing_is_calculated_correctly() {
        let bearing = bearing_to((30.1225, -101.09655555555555), (29.774166666666666, -101.16900000000001));

        assert_eq!(bearing, 190.23299);
    }
}
