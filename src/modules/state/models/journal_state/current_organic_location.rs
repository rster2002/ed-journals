use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CurrentOrganicLocation {
    pub first_organic: (f32, f32),
    pub second_organic: Option<(f32, f32)>,
}
