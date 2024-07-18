use serde::Serialize;
use crate::exobiology::Species;

#[derive(Debug, Serialize)]
pub struct PlanetSpeciesEntry {
    /// The species on the planet.
    pub specie: Species,

    /// Whether the species will actually be able to spawn by also taking into account genus
    /// uniqueness per planet.
    pub will_spawn: WillSpawn,

    /// Whether the species has been scanned at least once.
    pub scanned: bool,

    /// Whether the species has been scanned three times and has been logged.
    pub logged: bool,
}

#[derive(Debug, Serialize)]
pub enum WillSpawn {
    Yes,
    Maybe,
    No,
}
