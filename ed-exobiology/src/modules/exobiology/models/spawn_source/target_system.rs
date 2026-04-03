use ed_exobiology::modules::exobiology::models::spawn_source::spawn_source_star::SpawnSourceStar;
use ed_journals::modules::galaxy::models::planet_class::PlanetClass;
use serde::Serialize;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Serialize)]
pub struct TargetSystem {
    pub star_system_position: [f32; 3],
    pub planet_classes_in_system: HashSet<PlanetClass>,
    pub stars_in_system: HashMap<u8, SpawnSourceStar>,
}
