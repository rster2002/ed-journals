use std::collections::{HashMap, HashSet};

use serde::Serialize;
use strum::IntoEnumIterator;

use crate::{
    logs::content::{
        log_event_content::{
            fsd_jump_event::FSDJumpEvent,
            fss_body_signals_event::FSSBodySignalsEvent,
            location_event::LocationEvent,
            scan_event::{
                DistanceLs, ScanEvent, ScanEventKind, ScanEventParent, ScanEventPlanet,
                ScanEventStar,
            },
        },
        LogEvent, LogEventContent,
    },
    models::exploration::planetary_signal_type::PlanetarySignalType,
    modules::{
        exobiology::models::spawn_source::{SpawnSource, SpawnSourceStar, TargetPlanet},
        exploration::Nebula,
    },
};

use super::feed_result::FeedResult;

#[derive(Debug, Serialize)]
pub struct ExobiologyState {
    /// Map of body names to spawn sources.
    pub spawn_sources: HashMap<String, SpawnSource>,
    planet_scan_events: Vec<(ScanEvent, ScanEventPlanet)>,
    star_scan_events: Vec<(ScanEvent, ScanEventStar)>,
    fss_body_signals_events: Vec<FSSBodySignalsEvent>,
    location_events: Vec<LocationEvent>,
    fsd_jump_events: Vec<FSDJumpEvent>,
}

impl ExobiologyState {
    pub fn new() -> Self {
        ExobiologyState {
            spawn_sources: HashMap::new(),
            planet_scan_events: Vec::new(),
            star_scan_events: Vec::new(),
            fss_body_signals_events: Vec::new(),
            location_events: Vec::new(),
            fsd_jump_events: Vec::new(),
        }
    }

    /// Feeds an event into the ExobiologyState's pool of information to construct a SpawnSource.
    pub fn feed_event(&mut self, event: &LogEvent) -> FeedResult {
        match &event.content {
            LogEventContent::Scan(scan) => match &scan.kind {
                ScanEventKind::Planet(planet) => {
                    self.planet_scan_events.push((scan.clone(), planet.clone()));
                }
                ScanEventKind::Star(star) => {
                    self.star_scan_events.push((scan.clone(), star.clone()));
                }
                _ => return FeedResult::Skipped,
            },
            LogEventContent::FSSBodySignals(fss_body_signals) => {
                self.fss_body_signals_events.push(fss_body_signals.clone());
            }
            LogEventContent::Location(location) => {
                self.location_events.push(location.clone());
            }
            LogEventContent::FSDJump(fsd_jump) => {
                self.fsd_jump_events.push(fsd_jump.clone());
            }
            _ => return FeedResult::Skipped,
        }

        FeedResult::Accepted
    }

    /// Constructs a SpawnSource from the ExobiologyState's pool of information.
    pub fn for_body(&self, body_name: impl Into<String>) -> SpawnSource {
        let body_name = body_name.into();

        let event_is_applicable = |star_system: &String| body_name.starts_with(star_system);

        // Filter events to only include those that are applicable to the body.
        let star_scan_events = self
            .star_scan_events
            .iter()
            .filter(|(scan, _)| event_is_applicable(&scan.star_system))
            .collect::<Vec<&(ScanEvent, ScanEventStar)>>();

        let planet_scan_events = self
            .planet_scan_events
            .iter()
            .filter(|(scan, _)| event_is_applicable(&scan.star_system))
            .collect::<Vec<&(ScanEvent, ScanEventPlanet)>>();

        let target_body = planet_scan_events
            .iter()
            .filter(|(scan, _)| scan.body_name == body_name)
            .next()
            .map(|(scan, planet)| (scan, planet));

        let fss_body_signal = self
            .fss_body_signals_events
            .iter()
            .filter(|fss_body_signals| fss_body_signals.body_name == body_name)
            .next();

        let star_pos_from_location = self
            .location_events
            .iter()
            .filter(|location| event_is_applicable(&location.system_info.star_system))
            .map(|event| event.system_info.star_pos)
            .next();

        let star_pos_from_jump = self
            .fsd_jump_events
            .iter()
            .filter(|fsd_jump| event_is_applicable(&fsd_jump.system_info.star_system))
            .map(|event| event.system_info.star_pos)
            .next();

        // Construct the SpawnSource
        SpawnSource {
            body_name: body_name.clone(),
            star_system_position: star_pos_from_location.or(star_pos_from_jump),
            parent_stars: target_body
                .map(|(scan, _)| {
                    let parent_ids = scan
                        .parents
                        .iter()
                        .filter_map(|parent| match parent {
                            ScanEventParent::Star(star) => Some(star.clone()),
                            _ => None,
                        })
                        .collect::<Vec<u8>>();

                    star_scan_events
                        .iter()
                        .filter(|(scan, _)| parent_ids.contains(&scan.body_id))
                        .map(|(scan, star)| SpawnSourceStar {
                            class: star.star_type.clone(),
                            body_id: scan.body_id.clone(),
                            luminosity: star.luminosity.clone(),
                        })
                        .collect::<Vec<SpawnSourceStar>>()
                })
                .unwrap_or_default(),
            target_planet: target_body.map(|(_, planet)| TargetPlanet {
                atmosphere: planet.atmosphere.clone(),
                gravity: planet.surface_gravity.clone(),
                class: planet.planet_class.clone(),
                surface_temperature: planet.surface_temperature.clone(),
                volcanism: planet.volcanism.clone(),
                materials: HashSet::from_iter(planet.materials.iter().map(|m| m.name.clone())),
                composition: planet
                    .composition
                    .as_ref()
                    .map(|composition| composition.clone())
                    .unwrap_or_default(),
            }),
            geological_signals_present: fss_body_signal.map(|fss_body_signal| {
                fss_body_signal
                    .signals
                    .iter()
                    .any(|signal| signal.kind == PlanetarySignalType::Geological)
            }),
            distance_from_star: target_body.map(|body| body.0.distance_from_arrival.clone()),
            distance_from_nebula: star_pos_from_location
                .or(star_pos_from_jump)
                .map(|pos| Nebula::closest_to(pos).1),

            planet_classes_in_system: HashSet::from_iter(
                planet_scan_events
                    .iter()
                    .map(|(_, planet)| planet.planet_class.clone()),
            ),
            stars_in_system: HashSet::from_iter(star_scan_events.iter().map(|(scan, star)| {
                SpawnSourceStar {
                    class: star.star_type.clone(),
                    body_id: scan.body_id.clone(),
                    luminosity: star.luminosity.clone(),
                }
            })),
        }
    }
}

impl Default for ExobiologyState {
    fn default() -> Self {
        ExobiologyState::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::logs::content::LogEventContent;
    use crate::logs::LogDir;
    use crate::models::exploration::species::Species;
    use crate::state::ExobiologyState;
    use std::collections::{HashMap, HashSet};
    use std::env::current_dir;

    #[test]
    fn spawnable_species_no_false_negatives() {
        let dir_path = current_dir().unwrap().join("test-files").join("journals");
        let log_dir = LogDir::new(dir_path);
        let logs = log_dir.journal_logs().unwrap();

        let mut state = ExobiologyState::new();

        // Species found in the logs, grouped by body name.
        // These are the value we will compare against the calculated spawnable species.
        let mut expected_species = HashMap::<String, HashSet<Species>>::new();
        for journal in &logs {
            let reader = journal.create_blocking_reader().unwrap();

            let mut body_name = String::new();

            for entry in reader.flatten() {
                state.feed_event(&entry);

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

        // Blacklisted bodies that should not be tested
        let blacklisted_bodies: Vec<String> = vec![
            "Syniechia CB-U d4-8 B 5".to_string(), // Commander did not scan the body before landing
            "Prie Chraea VL-L c21-0 1 c".to_string(), // OsseusDiscus spawned on a body with a non-thin-water atmosphere
            "Syniechou RZ-Z c16-0 7 b a".to_string(), // OsseusDiscus spawned on a body with a non-thin-water atmosphere
            "Flyeia Prou RH-C b46-0 A 8".to_string(), // TubusSororibus spawned on a body with a gravity of 0.52g and temperature of 260K
            "Graea Proae OT-O d7-15 A 4".to_string(), // FrutexaMetallicum, OsseusPellebantus and TussockPropagito spawning on a body that's 0.4K too warm
            "Ruvoe HW-E c11-5 3 b".to_string(), // BacteriumOmentum spawning on a body with a non-neon atmosphere
        ];

        let mut failed = 0;

        // Check each spawn source to see if the calculated spawnable species match the expected species.
        for (body_name, expected_species) in expected_species
            .iter()
            .filter(|(body, _)| !blacklisted_bodies.contains(body))
        {
            let spawn_source = state.for_body(body_name);

            for species in expected_species {
                let conditions = species.spawn_conditions();

                let failing_conditions = conditions
                    .iter()
                    .filter(|condition| !spawn_source.satisfies_spawn_condition(condition))
                    .collect::<Vec<_>>();

                if !failing_conditions.is_empty() {
                    failed += 1;
                    println!(
                        "The following conditions failed for '{:?}' on body '{}': {:?}\n{:#?}",
                        species, body_name, failing_conditions, spawn_source
                    );
                }
            }
        }

        // In case of test failure, see the logs printed above.
        assert!(failed == 0);
    }
}
