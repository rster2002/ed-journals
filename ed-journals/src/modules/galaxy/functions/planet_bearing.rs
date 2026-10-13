/// Calculate the bearing (compas direction) between two coordinates on a planet.
pub fn planet_bearing(coord1: &(f32, f32), coord2: &(f32, f32)) -> f32 {
    let y = (coord2.1 - coord1.1).sin() * coord2.0.cos();
    let x = coord1.0.cos() * coord2.0.sin()
        - coord1.0.sin() * coord2.0.cos() * (coord2.1 - coord1.1).cos();
    let theta = y.atan2(x);

    (theta * 180.0 / std::f32::consts::PI + 360.0) % 360.0
}
