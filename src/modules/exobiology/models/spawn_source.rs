use std::collections::HashSet;

use strum::IntoEnumIterator;

use crate::logs::content::log_event_content::fsd_jump_event::FSDJumpEvent;
use crate::logs::content::log_event_content::fss_body_signals_event::{
    FSSBodySignalEventSignal, FSSBodySignalsEvent,
};
use crate::logs::content::log_event_content::location_event::LocationEvent;
use crate::logs::content::log_event_content::scan_event::{
    DistanceLs, Gravity, ScanEvent, ScanEventKind, ScanEventParent, ScanEventPlanet, ScanEventStar,
};
use crate::logs::content::LogEvent;
use crate::models::exploration::planetary_signal_type::PlanetarySignalType;
use crate::models::exploration::species::Species;
use crate::models::galaxy::atmosphere::AtmosphereDensity;
use crate::models::galaxy::atmosphere_type::AtmosphereType;
use crate::models::galaxy::star_luminosity::StarLuminosity;
use crate::models::galaxy::volcanism::Volcanism;
use crate::models::galaxy::volcanism_type::VolcanismType;
use crate::models::materials::material::Material;
use crate::modules::models::galaxy::atmosphere::Atmosphere;
use crate::modules::models::galaxy::planet_class::PlanetClass;
use crate::modules::models::galaxy::star_class::StarClass;

use super::spawn_condition::SpawnCondition;

#[derive(Debug)]
pub struct SpawnSource {
    pub body_name: String,
    pub star_system_position: Option<[f32; 3]>,
    pub parent_stars: Vec<Star>,
    pub parent_stars_ids: HashSet<u8>,
    pub target_planet: Option<TargetPlanet>,
    pub geological_signals_present: Option<bool>,
    pub distance_from_star: Option<DistanceLs>,
    pub distance_from_nebula: Option<DistanceLs>,
    pub planet_classes_in_system: HashSet<PlanetClass>,
    pub stars_in_system: Vec<Star>,
}

impl SpawnSource {
    pub fn new(body_name: impl Into<String>) -> SpawnSource {
        SpawnSource {
            body_name: body_name.into(),
            star_system_position: None,
            parent_stars: Vec::new(),
            parent_stars_ids: HashSet::new(),
            target_planet: None,
            planet_classes_in_system: HashSet::new(),
            stars_in_system: Vec::new(),
            geological_signals_present: None,
            distance_from_star: None,
            distance_from_nebula: None, // FIXME: No idea how to get this data yet.
        }
    }

    pub fn feed_scan_event(&mut self, scan: &ScanEvent) {
        // Only interested in events that are in the same star system as the spawn source.
        if !self.body_name.starts_with(&scan.star_system) {
            return;
        }

        let targets_tracked_body = self.body_name == scan.body_name;

        if targets_tracked_body {
            self.distance_from_star = Some(scan.distance_from_arrival.clone());
        }

        match &scan.kind {
            ScanEventKind::Star(star_scan) => {
                self.feed_star_scan_event(&scan, &star_scan);
            }
            ScanEventKind::Planet(planet_scan) => {
                if targets_tracked_body {
                    self.feed_planet_scan_event(&scan, &planet_scan);
                } else {
                    self.planet_classes_in_system
                        .insert(planet_scan.planet_class.clone());
                }
            }
            _ => {} // Ignore belt clusters, etc.
        }
    }

    pub fn feed_fss_body_signals_event(&mut self, signals: &FSSBodySignalsEvent) {
        if self.body_name != signals.body_name {
            return;
        }

        let geological_signals_present = signals
            .signals
            .iter()
            .any(|signal| signal.kind == PlanetarySignalType::Geological);

        self.geological_signals_present = Some(geological_signals_present);
    }

    pub fn feed_location_event(&mut self, location: &LocationEvent) {
        // Only interested in events that are in the same star system as the spawn source.
        if !self
            .body_name
            .starts_with(&location.system_info.star_system)
        {
            return;
        }

        self.star_system_position = Some(location.system_info.star_pos);
    }

    pub fn feed_fsd_jump_event(&mut self, jump: &FSDJumpEvent) {
        if !self.body_name.starts_with(&jump.system_info.star_system) {
            return;
        }

        self.star_system_position = Some(jump.system_info.star_pos);
    }

    fn feed_star_scan_event(&mut self, scan: &ScanEvent, star: &ScanEventStar) {
        self.stars_in_system.push(Star {
            body_id: scan.body_id.clone(),
            class: star.star_type.clone(),
            luminosity: star.luminosity.clone(),
        });
        self.recalculate_parent_stars();
    }

    fn feed_planet_scan_event(&mut self, scan: &ScanEvent, planet: &ScanEventPlanet) {
        let composition = if let Some(composition) = &planet.composition {
            PlanetComposition {
                ice: composition.ice,
                rock: composition.rock,
                metal: composition.metal,
            }
        } else {
            PlanetComposition {
                ice: 0.0,
                rock: 0.0,
                metal: 0.0,
            }
        };

        self.target_planet = Some(TargetPlanet {
            atmosphere: planet.atmosphere.clone(),
            gravity: planet.surface_gravity.clone(),
            class: planet.planet_class.clone(),
            surface_temperature: planet.surface_temperature,
            volcanism: planet.volcanism.clone(),
            materials: HashSet::from_iter(planet.materials.iter().map(|m| m.name.clone())),
            composition,
        });

        for parent in &scan.parents {
            match parent {
                ScanEventParent::Star(parent_star_id) => {
                    self.parent_stars_ids.insert(*parent_star_id);
                }
                // Ignore non-star parents
                _ => {}
            }
        }

        self.recalculate_parent_stars();
    }

    fn recalculate_parent_stars(&mut self) {
        self.parent_stars = self
            .parent_stars_ids
            .iter()
            .filter_map(|star_id| {
                self.stars_in_system
                    .iter()
                    .find(|star| star.body_id == *star_id)
                    .cloned()
            })
            .collect();
    }

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
                    nebula_distance.as_au() <= *nebula_range
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

#[derive(Debug)]
pub struct TargetPlanet {
    pub atmosphere: Atmosphere,
    pub gravity: Gravity,
    pub class: PlanetClass,
    pub surface_temperature: f32,
    pub volcanism: Volcanism,
    pub materials: HashSet<Material>,
    pub composition: PlanetComposition,
}

#[derive(Debug)]
pub struct PlanetComposition {
    pub ice: f32,
    pub rock: f32,
    pub metal: f32,
}

#[derive(Debug, Clone)]
pub struct Star {
    pub body_id: u8,
    pub class: StarClass,
    pub luminosity: StarLuminosity,
}

#[derive(Debug, PartialEq)]
pub struct Body {
    pub class: PlanetClass,
}

mod tests {
    use crate::logs::content::LogEventContent;
    use crate::logs::LogDir;
    use crate::models::exploration::genus::Genus;
    use crate::models::exploration::species::Species;
    use crate::modules::exobiology::models::spawn_condition;
    use crate::modules::exobiology::models::spawn_source::SpawnSource;
    use std::collections::{HashMap, HashSet};
    use std::env::current_dir;
    use std::thread::current;

    use super::Body;
    // use crate::blocking::JournalDir;
    //
    // use crate::modules::logs::content::log_event_content::JournalEventContent;

    #[test]
    fn spawnable_species_no_false_negatives() {
        let dir_path = current_dir().unwrap().join("test-files").join("journals");
        let log_dir = LogDir::new(dir_path);
        let logs = log_dir.journal_logs().unwrap();

        // Species found in the logs, grouped by body name.
        // These are the value we will compare against the calculated spawnable species.
        let mut expected_species = HashMap::<String, HashSet<Species>>::new();
        for journal in &logs {
            let reader = journal.create_blocking_reader().unwrap();

            let mut body_name = String::new();

            for entry in reader.flatten() {
                if let LogEventContent::Location(location) = &entry.content {
                    body_name = location.system_info.body.clone()
                }

                if let LogEventContent::Touchdown(touchdown) = &entry.content {
                    body_name = touchdown.body.clone();
                }

                if let LogEventContent::ScanOrganic(organic) = &entry.content {
                    expected_species
                        .entry(body_name.clone())
                        .or_insert_with(HashSet::new)
                        .insert(organic.species.clone());
                }
            }
        }

        // Create a SpawnSource for each body in our test data that has an expected list of species.
        let mut spawn_sources = HashMap::<String, SpawnSource>::new();
        for (body, _) in &expected_species {
            spawn_sources.insert(body.clone(), SpawnSource::new(body.clone()));
        }

        // Supply the journal events to the spawn sources.
        for journal in &logs {
            let reader = journal.create_blocking_reader().unwrap();

            for entry in reader.flatten() {
                if let LogEventContent::Scan(scan) = &entry.content {
                    for (_, spawn_source) in &mut spawn_sources {
                        spawn_source.feed_scan_event(scan);
                    }
                }

                if let LogEventContent::FSSBodySignals(fss_body_signals) = &entry.content {
                    for (_, spawn_source) in &mut spawn_sources {
                        spawn_source.feed_fss_body_signals_event(fss_body_signals);
                    }
                }

                if let LogEventContent::Location(location) = &entry.content {
                    for (_, spawn_source) in &mut spawn_sources {
                        spawn_source.feed_location_event(location);
                    }
                }

                if let LogEventContent::FSDJump(fsd_jump) = &entry.content {
                    for (_, spawn_source) in &mut spawn_sources {
                        spawn_source.feed_fsd_jump_event(fsd_jump);
                    }
                }
            }
        }

        // Blacklisted bodies that should not be tested
        let blacklisted_bodies: Vec<String> = vec!["Syniechia CB-U d4-8 B 5".to_string()];

        // Check each spawn source to see if the calculated spawnable species match the expected species.
        for (body_name, expected_species) in expected_species
            .iter()
            .filter(|(body, _)| !blacklisted_bodies.contains(body))
        {
            let spawn_source = spawn_sources.get(body_name).unwrap();

            for species in expected_species {
                let conditions = species.spawn_conditions();

                let failing_conditions = conditions
                    .iter()
                    .filter(|condition| !spawn_source.satisfies_spawn_condition(condition))
                    .collect::<Vec<_>>();

                assert!(
                    &failing_conditions.is_empty(),
                    "The following conditions failed for '{:?}' on body '{}': {:?}\n{:#?}",
                    species,
                    body_name,
                    failing_conditions,
                    spawn_source
                );
            }
        }
    }
}
