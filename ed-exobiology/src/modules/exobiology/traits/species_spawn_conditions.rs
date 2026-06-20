use crate::SpawnCondition;
use crate::SPECIES_SPAWN_CONDITIONS;
use ed_journals::exobiology::Species;

/// Trait for species that have spawn conditions.
pub trait SpeciesSpawnConditions {
    /// Returns a list of spawn conditions that should all match for this species to spawn on a
    /// planet.
    fn spawn_conditions(&self) -> &[SpawnCondition<'_>];
}

impl SpeciesSpawnConditions for Species {
    fn spawn_conditions(&self) -> &[SpawnCondition<'_>] {
        SPECIES_SPAWN_CONDITIONS
            .iter()
            .find(|(species, _)| species == self)
            .expect("Species should always have a matching spawning condition")
            .1
    }
}

#[cfg(test)]
mod tests {
    use crate::SpeciesSpawnConditions;
    use ed_journals::exobiology::Species;
    use strum::IntoEnumIterator;

    #[test]
    fn all_species_have_matching_spawn_conditions() {
        for species in Species::iter() {
            dbg!(&species);
            assert!(!species.spawn_conditions().is_empty());
        }
    }
}
