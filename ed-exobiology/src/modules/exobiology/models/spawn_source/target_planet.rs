use ed_journals::modules::galaxy::{Atmosphere, Gravity, PlanetClass, PlanetComposition, Volcanism};
use ed_journals::modules::logs::content::log_event_content::scan_event::ScanEventParent;
use ed_journals::modules::materials::models::material::Material;
use serde::Serialize;
use std::collections::HashSet;

#[derive(Debug, Serialize)]
pub struct TargetPlanet {
    pub atmosphere: Atmosphere,
    pub gravity: Gravity,
    pub class: PlanetClass,
    pub surface_temperature: f32,
    pub volcanism: Volcanism,
    pub materials: HashSet<Material>,
    pub composition: Option<PlanetComposition>,
    pub parents: Vec<ScanEventParent>,
    pub semi_major_axis: f32,
    pub geological_signals_present: bool,
}
