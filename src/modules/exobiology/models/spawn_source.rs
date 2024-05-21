use std::collections::HashSet;

use serde::Serialize;
use strum::IntoEnumIterator;

use crate::exobiology::{SpawnCondition, Species};
use crate::galaxy::{
    Atmosphere, AtmosphereDensity, AtmosphereType, PlanetClass, PlanetComposition, StarClass,
    StarLuminosity, Volcanism, VolcanismType,
};
use crate::logs::content::log_event_content::scan_event::{DistanceLs, Gravity};
use crate::materials::Material;

#[derive(Debug, Serialize)]
pub struct SpawnSource {
    pub body_name: String,
    pub star_system_position: Option<[f32; 3]>,
    pub parent_stars: Vec<SpawnSourceStar>,
    pub target_planet: Option<TargetPlanet>,
    pub geological_signals_present: Option<bool>,
    pub distance_from_star: Option<DistanceLs>,
    pub distance_from_nebula: Option<DistanceLs>,
    pub planet_classes_in_system: HashSet<PlanetClass>,
    pub stars_in_system: HashSet<SpawnSourceStar>,
}

impl SpawnSource {
    /// Returns a list of species that could spawn on this spawn source.
    pub fn get_spawnable_species(&self) -> HashSet<Species> {
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
                if let Some(target_planet) = &self.target_planet {
                    target_planet.surface_temperature >= *min_temp
                } else {
                    false
                }
            }
            SpawnCondition::MaxMeanTemperature(max_temp) => {
                if let Some(target_planet) = &self.target_planet {
                    target_planet.surface_temperature <= *max_temp
                } else {
                    false
                }
            }
            SpawnCondition::NoAtmosphere => {
                if let Some(target_planet) = &self.target_planet {
                    target_planet.atmosphere.kind == AtmosphereType::None
                } else {
                    false
                }
            }
            SpawnCondition::AnyThinAtmosphere => {
                if let Some(target_planet) = &self.target_planet {
                    target_planet.atmosphere.density == AtmosphereDensity::Thin
                } else {
                    false
                }
            }
            SpawnCondition::ThinAtmosphere(atmosphere_type) => {
                if let Some(target_planet) = &self.target_planet {
                    target_planet.atmosphere.density == AtmosphereDensity::Thin
                        && target_planet.atmosphere.kind == *atmosphere_type
                } else {
                    false
                }
            }
            SpawnCondition::MinGravity(min_gravity) => {
                if let Some(target_planet) = &self.target_planet {
                    target_planet.gravity.as_g() >= *min_gravity
                } else {
                    false
                }
            }
            SpawnCondition::MaxGravity(max_gravity) => {
                if let Some(target_planet) = &self.target_planet {
                    target_planet.gravity.as_g() <= *max_gravity
                } else {
                    false
                }
            }
            SpawnCondition::PlanetClass(planet_class) => {
                if let Some(target_planet) = &self.target_planet {
                    target_planet.class == *planet_class
                } else {
                    false
                }
            }
            SpawnCondition::ParentStarClass(star_class) => self
                .parent_stars
                .iter()
                .any(|star| star.class == *star_class),
            SpawnCondition::ParentStarLuminosity(star_luminosity) => self
                .parent_stars
                .iter()
                .any(|star| star.luminosity == *star_luminosity),
            SpawnCondition::MinOrEqualParentStarLuminosity(star_luminosity) => self
                .parent_stars
                .iter()
                .any(|star| star.luminosity >= *star_luminosity),
            SpawnCondition::SystemContainsPlanetClass(planet_class) => {
                self.planet_classes_in_system.contains(planet_class)
            }
            SpawnCondition::VolcanismType(volcanism_type) => {
                if let Some(target_planet) = &self.target_planet {
                    target_planet.volcanism.kind == *volcanism_type
                } else {
                    false
                }
            }
            SpawnCondition::MinDistanceFromParentSun(min_distance) => {
                if let Some(distance_from_star) = &self.distance_from_star {
                    distance_from_star.as_au() >= *min_distance
                } else {
                    false
                }
            }
            SpawnCondition::AnyVolcanism => {
                if let Some(target_planet) = &self.target_planet {
                    target_planet.volcanism.kind != VolcanismType::None
                } else {
                    false
                }
            }
            SpawnCondition::WithinNebulaRange(nebula_range) => {
                if let Some(nebula_distance) = &self.distance_from_nebula {
                    nebula_distance.as_ly() <= *nebula_range
                } else {
                    false
                }
            }
            SpawnCondition::GeologicalSignalsPresent => {
                if let Some(geological_signals_present) = &self.geological_signals_present {
                    *geological_signals_present
                } else {
                    false
                }
            }
            SpawnCondition::MaterialPresence(material) => {
                if let Some(target_planet) = &self.target_planet {
                    target_planet.materials.contains(material)
                } else {
                    false
                }
            }
            SpawnCondition::RockyComposition => {
                if let Some(target_planet) = &self.target_planet {
                    target_planet.composition.rock > 0.0
                } else {
                    false
                }
            }
            SpawnCondition::IcyComposition => {
                if let Some(target_planet) = &self.target_planet {
                    target_planet.composition.ice > 0.0
                } else {
                    false
                }
            }
            SpawnCondition::MetalComposition => {
                if let Some(target_planet) = &self.target_planet {
                    target_planet.composition.metal > 0.0
                } else {
                    false
                }
            }
            SpawnCondition::Any(conditions) => conditions
                .iter()
                .any(|condition| self.satisfies_spawn_condition(condition)),
            SpawnCondition::All(conditions) => conditions
                .iter()
                .all(|condition| self.satisfies_spawn_condition(condition)),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TargetPlanet {
    pub atmosphere: Atmosphere,
    pub gravity: Gravity,
    pub class: PlanetClass,
    pub surface_temperature: f32,
    pub volcanism: Volcanism,
    pub materials: HashSet<Material>,
    pub composition: PlanetComposition,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize)]
pub struct SpawnSourceStar {
    pub body_id: u8,
    pub class: StarClass,
    pub luminosity: StarLuminosity,
}
