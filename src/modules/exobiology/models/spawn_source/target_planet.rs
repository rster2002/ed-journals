use crate::galaxy::{
    Atmosphere, Gravity, LocalDistance, PlanetClass, PlanetComposition, Volcanism,
};
use crate::logs::scan_event::ScanEventParent;
use crate::materials::Material;
use serde::Serialize;
use std::collections::HashSet;

#[derive(Debug, Serialize)]
pub struct TargetPlanet {
    pub is_landable: bool,
    pub atmosphere: Atmosphere,
    pub gravity: Gravity,
    pub class: PlanetClass,
    pub surface_temperature: f32,
    pub volcanism: Volcanism,
    pub materials: HashSet<Material>,
    pub composition: Option<PlanetComposition>,
    pub parents: Vec<ScanEventParent>,
    pub semi_major_axis: LocalDistance,
    pub geological_signals_present: bool,
}
