use ed_journals::exobiology::Species;

#[derive(Debug)]
pub struct PlanetSpeciesEntry {
    /// The species on the planet.
    pub species: Species,

    /// Whether the species will actually be able to spawn by also taking into account genus
    /// uniqueness per planet.
    pub will_spawn: WillSpawn,

    /// Whether the species has been scanned at least once.
    pub confirmed: bool,

    /// Whether the species has been scanned three times and has been logged.
    pub completed: bool,
}

#[derive(Debug)]
pub enum WillSpawn {
    Yes,
    Maybe,
    No,
}

impl WillSpawn {
    pub fn yes(&self) -> bool {
        matches!(self, WillSpawn::Yes)
    }

    pub fn maybe(&self) -> bool {
        matches!(self, WillSpawn::Maybe)
    }

    pub fn no(&self) -> bool {
        matches!(self, WillSpawn::No)
    }
}
