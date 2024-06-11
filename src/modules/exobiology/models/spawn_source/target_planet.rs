use crate::galaxy::{Atmosphere, Gravity, PlanetClass, PlanetComposition, Volcanism};
use crate::materials::Material;
use serde::Serialize;
use std::collections::HashSet;

#[cfg(feature = "logs")]
use crate::logs::scan_event::ScanEventParent;

#[derive(Debug, Serialize)]
pub struct TargetPlanet {
    pub atmosphere: Atmosphere,
    pub gravity: Gravity,
    pub class: PlanetClass,
    pub surface_temperature: f32,
    pub volcanism: Volcanism,
    pub materials: HashSet<Material>,
    pub composition: Option<PlanetComposition>,
    pub parents: Vec<TargetPlanetParent>,
    pub semi_major_axis: f32,
    pub geological_signals_present: bool,
}

#[derive(Debug, Serialize)]
pub enum TargetPlanetParent {
    Null(u8),
    Star(u8),
    Ring(u8),
    Planet(u8),
}

#[cfg(feature = "logs")]
impl From<ScanEventParent> for TargetPlanetParent {
    fn from(value: ScanEventParent) -> Self {
        match value {
            ScanEventParent::Null(body_id) => TargePlanetParent::Null(body_id),
            ScanEventParent::Star(body_id) => TargePlanetParent::Star(body_id),
            ScanEventParent::Ring(body_id) => TargePlanetParent::Ring(body_id),
            ScanEventParent::Planet(body_id) => TargePlanetParent::Planet(body_id),
        }
    }
}
