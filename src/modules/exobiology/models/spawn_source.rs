use strum::IntoEnumIterator;

use crate::logs::content::log_event_content::scan_event::{
    ScanEvent, ScanEventKind, ScanEventPlanet, ScanEventStar,
};
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
    body_name: String,
    pub star: Option<Star>,
    pub target_planet: Option<TargetPlanet>,
    pub bodies: Vec<Body>,
    pub nebula_distance: Option<f32>,
}

impl SpawnSource {
    pub fn new(body_name: impl Into<String>) -> SpawnSource {
        SpawnSource {
            body_name: body_name.into(),
            star: None,
            target_planet: None,
            bodies: Vec::new(),
            nebula_distance: None,
        }
    }

    pub fn supply_scan_event(&mut self, scan: &ScanEvent) {
        // TODO: Check if the scan event is actually relevant for this spawn source.
        //  Perhaps check the star system name, or the body name, etc.
        if !self.body_name.starts_with(&scan.star_system) {
            return;
        }

        match &scan.kind {
            ScanEventKind::Star(scan) => {
                self.supply_star_scan_event(&scan);
            }
            ScanEventKind::Planet(scan) => {
                self.supply_planet_scan_event(&scan);
            }
            _ => {} // Ignore belt clusters, etc.
        }
    }

    fn supply_star_scan_event(&mut self, scan: &ScanEventStar) {
        self.star = Some(Star {
            class: scan.star_type.clone(),
            luminosity: scan.luminosity.clone(),
        });
    }

    fn supply_planet_scan_event(&mut self, scan: &ScanEventPlanet) {
        self.target_planet = Some(TargetPlanet {
            atmosphere: scan.atmosphere.clone(),
            gravity: scan.surface_gravity,
            class: scan.planet_class.clone(),
            surface_temperature: scan.surface_temperature,
            volcanism: scan.volcanism.clone(),
            distance_from_star: 0.0, // FIXME: Find a way to get this value from the scan event.
            geological_signals_present: GeologicalSignalsPresent::Unknown, // FIXME: Find a way to get this value from the scan event.
        });
    }

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
                    target_planet.gravity >= *min_gravity
                } else {
                    false
                }
            }
            SpawnCondition::MaxGravity(max_gravity) => {
                if let Some(target_planet) = &self.target_planet {
                    target_planet.gravity <= *max_gravity
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
            SpawnCondition::ParentStarClass(star_class) => {
                if let Some(star) = &self.star {
                    star.class == *star_class
                } else {
                    false
                }
            }
            SpawnCondition::ParentStarLuminosity(star_luminosity) => {
                if let Some(star) = &self.star {
                    star.luminosity == *star_luminosity
                } else {
                    false
                }
            }
            SpawnCondition::MinOrEqualParentStarLuminosity(star_luminosity) => {
                if let Some(star) = &self.star {
                    // FIXME: This requires the luminosity enum to be ordered from weak to strong luminosity.
                    //  This could be a potential bug if that enum is ever refactored.
                    star.luminosity >= *star_luminosity
                } else {
                    false
                }
            }
            SpawnCondition::SystemContainsPlanetClass(planet_class) => {
                self.bodies.iter().any(|b: &Body| b.class == *planet_class)
            }
            SpawnCondition::VolcanismType(volcanism_type) => {
                if let Some(target_planet) = &self.target_planet {
                    target_planet.volcanism.kind == *volcanism_type
                } else {
                    false
                }
            }
            SpawnCondition::MinDistanceFromParentSun(min_distance) => {
                if let Some(target_planet) = &self.target_planet {
                    target_planet.distance_from_star >= *min_distance
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
                if let Some(nebula_distance) = self.nebula_distance {
                    nebula_distance <= *nebula_range
                } else {
                    false
                }
            }
            SpawnCondition::GeologicalSignalsPresent => {
                if let Some(target_planet) = &self.target_planet {
                    // TODO: Quickly draw the NotPresent conclusion if geological signals could not even
                    //  possibly exist on the target planet (atmosphere?), instead of waiting for the
                    //  result of planetary mapping to confirm signals are not present.

                    // NOTE: Do we want to consider the Unknown state (not yet mapped) as a valid datapoint?
                    //  If so, we should not return a simple boolean, but an enum with a 'Maybe' value to
                    //  indicate uncertainty about this spawn condition.
                    target_planet.geological_signals_present == GeologicalSignalsPresent::Present
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
