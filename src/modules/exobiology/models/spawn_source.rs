use strum::IntoEnumIterator;

use crate::models::exploration::species::Species;
use crate::models::galaxy::atmosphere::AtmosphereDensity;
use crate::models::galaxy::atmosphere_type::AtmosphereType;
use crate::models::galaxy::star_luminosity::StarLuminosity;
use crate::models::galaxy::volcanism::Volcanism;
use crate::models::galaxy::volcanism_type::VolcanismType;
use crate::modules::models::galaxy::atmosphere::Atmosphere;
use crate::modules::models::galaxy::planet_class::PlanetClass;
use crate::modules::models::galaxy::star_class::StarClass;

use super::spawn_condition::SpawnCondition;

#[derive(Debug)]
pub struct SpawnSource {
    pub star: Star,
    pub target_planet: TargetPlanet,
    pub bodies: Vec<Body>,
    pub nebula_distance: f32,
}

impl SpawnSource {
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
                self.target_planet.surface_temperature >= *min_temp
            }
            SpawnCondition::MaxMeanTemperature(max_temp) => {
                self.target_planet.surface_temperature <= *max_temp
            }
            SpawnCondition::NoAtmosphere => {
                self.target_planet.atmosphere.kind == AtmosphereType::None
            }
            SpawnCondition::AnyThinAtmosphere => {
                self.target_planet.atmosphere.density == AtmosphereDensity::Thin
            }
            SpawnCondition::ThinAtmosphere(atmosphere_type) => {
                self.target_planet.atmosphere.density == AtmosphereDensity::Thin
                    && self.target_planet.atmosphere.kind == *atmosphere_type
            }
            SpawnCondition::MinGravity(min_gravity) => self.target_planet.gravity >= *min_gravity,
            SpawnCondition::MaxGravity(max_gravity) => self.target_planet.gravity <= *max_gravity,
            SpawnCondition::PlanetClass(planet_class) => self.target_planet.class == *planet_class,
            SpawnCondition::ParentStarClass(star_class) => self.star.class == *star_class,
            SpawnCondition::ParentStarLuminosity(star_luminosity) => {
                self.star.luminosity == *star_luminosity
            }
            SpawnCondition::MinOrEqualParentStarLuminosity(star_luminosity) => {
                // NOTE: This requires the luminosity enum to be ordered from weak to strong luminosity.
                //  This could be a potential bug if that enum is ever refactored.
                self.star.luminosity >= *star_luminosity
            }
            SpawnCondition::SystemContainsPlanetClass(planet_class) => {
                self.bodies.iter().any(|b: &Body| b.class == *planet_class)
            }
            SpawnCondition::VolcanismType(volcanism_type) => {
                self.target_planet.volcanism.kind == *volcanism_type
            }
            SpawnCondition::MinDistanceFromParentSun(min_distance) => {
                self.target_planet.distance_from_star >= *min_distance
            }
            SpawnCondition::AnyVolcanism => {
                self.target_planet.volcanism.kind != VolcanismType::None
            }
            SpawnCondition::WithinNebulaRange(nebula_range) => {
                self.nebula_distance <= *nebula_range
            }
            SpawnCondition::GeologicalSignalsPresent => {
                // TODO: Quickly draw the NotPresent conclusion if geological signals could not even
                //  possibly exist on the target planet (atmosphere?), instead of waiting for the
                //  result of planetary mapping to confirm signals are not present.

                // NOTE: Do we want to consider the Unknown state (not yet mapped) as a valid datapoint?
                //  If so, we should not return a simple boolean, but an enum with a 'Maybe' value to
                //  indicate uncertainty about this spawn condition.
                self.target_planet.geological_signals_present == GeologicalSignalsPresent::Present
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

#[derive(Debug)]
pub struct TargetPlanet {
    pub atmosphere: Atmosphere,
    pub gravity: f32,
    pub class: PlanetClass,
    pub surface_temperature: f32,
    pub volcanism: Volcanism,
    /// The distance from the parent star in AU
    pub distance_from_star: f32,
    pub geological_signals_present: GeologicalSignalsPresent,
}

#[derive(Debug, PartialEq)]
pub enum GeologicalSignalsPresent {
    /// Mapping of the planet has concluded that geological signals are present.
    Present,
    /// The planet has not been mapped yet, so it is unknown if geological signals are present.
    Unknown,
    /// Mapping of the planet has concluded that geological signals are not present.
    NotPresent,
}

#[derive(Debug)]
pub struct Star {
    pub class: StarClass,
    pub luminosity: StarLuminosity,
}

#[derive(Debug, PartialEq)]
pub struct Body {
    pub class: PlanetClass,
}
