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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum WillSpawn {
    No,
    Maybe,
    Yes,
    Scanning,
    Completed,
}

impl WillSpawn {
    pub fn no(&self) -> bool {
        matches!(self, WillSpawn::No)
    }

    pub fn maybe(&self) -> bool {
        matches!(self, WillSpawn::Maybe)
    }

    pub fn yes(&self) -> bool {
        matches!(self, WillSpawn::Yes)
    }

    pub fn scanning(&self) -> bool {
        matches!(self, WillSpawn::Scanning)
    }

    pub fn completed(&self) -> bool {
        matches!(self, WillSpawn::Completed)
    }
}
