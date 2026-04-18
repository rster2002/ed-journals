use crate::exobiology::SpawnCondition;
use crate::exobiology::r#static::species_spawn_conditions::SPECIES_SPAWN_CONDITIONS;
use ed_journals::exobiology::Species;

pub trait SpeciesSpawnConditions {
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
    use crate::exobiology::SpeciesSpawnConditions;
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
