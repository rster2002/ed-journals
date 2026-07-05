use crate::SpawnSourceStar;
use ed_journals::galaxy::PlanetClass;
use std::collections::{HashMap, HashSet};

/// Information about a star system.
#[derive(Debug, Clone, PartialEq)]
pub struct TargetSystem {
    /// The coordinates of the star system.
    pub star_system_position: [f32; 3],

    /// Set of unique planet classes in this system.
    pub planet_classes_in_system: HashSet<PlanetClass>,

    /// Map of stars in the system, indexed by their body id.
    pub stars_in_system: HashMap<u8, SpawnSourceStar>,
}
