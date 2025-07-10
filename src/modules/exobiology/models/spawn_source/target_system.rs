use crate::exobiology::models::spawn_source::spawn_source_star::SpawnSourceStar;
use crate::galaxy::PlanetClass;
use serde::Serialize;
use std::collections::{HashMap, HashSet};

/// The minimum amount of information needed about a system to be able to predict species at a given
/// planet. Used in [SpawnSource](crate::exobiology::SpawnSource).
#[derive(Debug, Serialize)]
pub struct TargetSystem {
    pub star_system_position: [f32; 3],
    pub planet_classes_in_system: HashSet<PlanetClass>,
    pub stars_in_system: HashMap<u8, SpawnSourceStar>,
}
