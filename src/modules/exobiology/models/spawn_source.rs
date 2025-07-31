pub mod spawn_source_star;
pub mod target_planet;
pub mod target_system;

use strum::IntoEnumIterator;

use crate::exobiology::models::spawn_source::spawn_source_star::SpawnSourceStar;
use crate::exobiology::models::spawn_source::target_planet::TargetPlanet;
use crate::exobiology::models::spawn_source::target_system::TargetSystem;
use crate::exobiology::{SpawnCondition, Species};
use crate::galaxy::{AtmosphereDensity, AtmosphereType, Nebula, VolcanismType};
use crate::logs::scan_event::ScanEventParent;

/// Model for predicting species at a given planet, bases on information about the system and
/// planet.
#[derive(Debug)]
pub struct SpawnSource<'a> {
    pub target_system: &'a TargetSystem,
    pub target_planet: &'a TargetPlanet,
}

impl SpawnSource<'_> {
    /// Returns a list of species that could spawn on this spawn source.
    pub fn get_spawnable_species(&self) -> Vec<Species> {
        Species::iter()
            .filter(|species| self.can_spawn_species(species))
            .collect()
    }

    /// Checks if the given species can spawn on this spawn source.
    pub fn can_spawn_species(&self, species: &Species) -> bool {
        species
            .spawn_conditions()
            .iter()
            .all(|condition| self.satisfies_spawn_condition(condition))
    }

    /// Checks if the spawn source satisfies the given condition.
    pub fn satisfies_spawn_condition(&self, condition: &SpawnCondition) -> bool {
        match condition {
            SpawnCondition::MinMeanTemperature(min_temp) => {
                &self.target_planet.surface_temperature >= min_temp
            }
            SpawnCondition::MaxMeanTemperature(max_temp) => {
                &self.target_planet.surface_temperature <= max_temp
            }
            SpawnCondition::NoAtmosphere => {
                self.target_planet.atmosphere.kind == AtmosphereType::None
            }
            SpawnCondition::AnyThinAtmosphere => {
                self.target_planet.atmosphere.density == AtmosphereDensity::Thin
            }
            SpawnCondition::ThinAtmosphere(atmosphere_type) => {
                self.target_planet.atmosphere.density == AtmosphereDensity::Thin
                    && &self.target_planet.atmosphere.kind == atmosphere_type
            }
            SpawnCondition::MinGravity(min_gravity) => {
                &self.target_planet.gravity.as_g() >= min_gravity
            }
            SpawnCondition::MaxGravity(max_gravity) => {
                &self.target_planet.gravity.as_g() <= max_gravity
            }
            SpawnCondition::PlanetClass(planet_class) => &self.target_planet.class == planet_class,
            SpawnCondition::ParentStarClass(star_class) => {
                self.parent_stars().any(|star| &star.class == star_class)
            }
            SpawnCondition::ParentStarLuminosity(star_luminosity) => self
                .parent_stars()
                .any(|star| &star.luminosity == star_luminosity),
            SpawnCondition::MinOrEqualParentStarLuminosity(star_luminosity) => self
                .parent_stars()
                .any(|star| &star.luminosity >= star_luminosity),
            SpawnCondition::SystemContainsPlanetClass(planet_class) => self
                .target_system
                .planet_classes_in_system
                .contains(planet_class),
            SpawnCondition::VolcanismType(volcanism_type) => {
                &self.target_planet.volcanism.kind == volcanism_type
            }
            SpawnCondition::MinDistanceFromParentSun(min_distance) => {
                // TODO not sure, but Eccentricity might need to be taken into account as well
                &(self.target_planet.semi_major_axis / 149597870700.0) >= min_distance
            }
            SpawnCondition::AnyVolcanism => {
                self.target_planet.volcanism.kind != VolcanismType::None
            }
            SpawnCondition::WithinNebulaRange(nebula_range) => {
                &Nebula::closest_to(self.target_system.star_system_position)
                    .distance_to(self.target_system.star_system_position)
                    .as_ly()
                    <= nebula_range
            }
            SpawnCondition::GeologicalSignalsPresent => {
                self.target_planet.geological_signals_present
            }
            SpawnCondition::MaterialPresence(material) => {
                self.target_planet.materials.contains(material)
            }
            SpawnCondition::RockyComposition => {
                let Some(composition) = &self.target_planet.composition else {
                    return false;
                };

                composition.rock > 0.0
            }
            SpawnCondition::IcyComposition => {
                let Some(composition) = &self.target_planet.composition else {
                    return false;
                };

                composition.ice > 0.0
            }
            SpawnCondition::MetalComposition => {
                let Some(composition) = &self.target_planet.composition else {
                    return false;
                };

                composition.metal > 0.0
            }

            SpawnCondition::Special => false,

            SpawnCondition::Any(conditions) => conditions
                .iter()
                .any(|condition| self.satisfies_spawn_condition(condition)),
            SpawnCondition::All(conditions) => conditions
                .iter()
                .all(|condition| self.satisfies_spawn_condition(condition)),
        }
    }

    pub fn parent_stars(&self) -> impl Iterator<Item = &SpawnSourceStar> {
        self.target_planet.parents.iter().filter_map(|parent| {
            if let ScanEventParent::Star(id) = parent {
                return self.target_system.stars_in_system.get(id);
            }

            None
        })
    }
}
