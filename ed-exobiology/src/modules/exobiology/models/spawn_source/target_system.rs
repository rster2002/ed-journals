use crate::exobiology::SpawnSourceStar;
use ed_journals::galaxy::PlanetClass;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct TargetSystem {
    pub star_system_position: [f32; 3],
    pub planet_classes_in_system: HashSet<PlanetClass>,
    pub stars_in_system: HashMap<u8, SpawnSourceStar>,
}
