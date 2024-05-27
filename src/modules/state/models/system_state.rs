use std::collections::HashMap;

use crate::exobiology::{SpawnSourceStar, Species, TargetSystem};
use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::logs::fss_signal_discovered_event::FSSSignalDiscoveredEvent;
use crate::logs::scan_event::ScanEventKind;
use crate::logs::{LogEvent, LogEventContent};
use crate::modules::civilization::LocationInfo;
use crate::state::models::feed_result::FeedResult;
use crate::state::models::planet_state::planet_species_entry::PlanetSpeciesEntry;
use crate::state::models::planet_state::PlanetState;

#[derive(Serialize)]
pub struct SystemState {
    pub location_info: LocationInfo,
    pub bodies: HashMap<u8, PlanetState>,
    pub visits: Vec<DateTime<Utc>>,
    pub carrier_visits: Vec<DateTime<Utc>>,
    pub number_of_bodies: Option<u8>,
    pub progress: f32,
    pub all_found: bool,
    pub station_signals: Vec<FSSSignalDiscoveredEvent>,
    pub exobiology_system: TargetSystem,
}

impl SystemState {
    pub fn feed_log_event(&mut self, log_event: &LogEvent) -> FeedResult {
        let Some(system_address) = log_event.content.system_address() else {
            return FeedResult::Skipped;
        };

        if system_address != self.location_info.system_address {
            return FeedResult::Skipped;
        }

        match &log_event.content {
            LogEventContent::FSSDiscoveryScan(event) => {
                self.number_of_bodies = Some(event.body_count);
                self.progress = event.progress;
            }
            LogEventContent::FSSAllBodiesFound(event) => {
                self.number_of_bodies = Some(event.count);
                self.all_found = true;
            }
            LogEventContent::FSSSignalDiscovered(event) => {
                if event.is_station {
                    self.station_signals.push(event.clone());
                }
            }
            LogEventContent::Scan(event) => match &event.kind {
                ScanEventKind::Star(star) => {
                    self.exobiology_system.stars_in_system.insert(
                        event.body_id,
                        SpawnSourceStar {
                            class: star.star_type.clone(),
                            luminosity: star.luminosity.clone(),
                        },
                    );
                }
                ScanEventKind::Planet(planet) => {
                    self.bodies
                        .entry(event.body_id)
                        .or_insert_with(|| PlanetState::from((event, planet)));

                    self.exobiology_system
                        .planet_classes_in_system
                        .insert(planet.planet_class.clone());
                }
                ScanEventKind::BeltCluster(_) => {}
            },

            _ => {
                if let Some(body_id) = log_event.content.body_id() {
                    let Some(body) = self.bodies.get_mut(&body_id) else {
                        return FeedResult::Later;
                    };

                    return body.feed_log_event(log_event);
                }
            }
        }

        FeedResult::Accepted
    }

    pub fn visit(&mut self, date_time: &DateTime<Utc>) {
        self.visits.push(*date_time);
    }

    pub fn carrier_visit(&mut self, date_time: &DateTime<Utc>) {
        self.carrier_visits.push(*date_time);
    }

    pub fn get_spawnable_species(&self, body_id: u8) -> Option<Vec<PlanetSpeciesEntry>> {
        Some(
            self.bodies
                .get(&body_id)?
                .get_planet_species(&self.exobiology_system),
        )
    }
}

impl From<&LocationInfo> for SystemState {
    fn from(value: &LocationInfo) -> Self {
        SystemState {
            location_info: value.clone(),
            bodies: HashMap::new(),
            visits: Vec::new(),
            carrier_visits: Vec::new(),
            number_of_bodies: None,
            progress: 0.0,
            all_found: false,
            station_signals: Vec::new(),
            exobiology_system: TargetSystem {
                star_system_position: value.star_pos,
                planet_classes_in_system: Default::default(),
                stars_in_system: Default::default(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};
    use std::env::current_dir;
    use strum::IntoEnumIterator;

    use crate::exobiology::{SpawnCondition, SpawnSource, Species};
    use crate::logs::blocking::LogDirReader;
    use crate::state::models::planet_state::planet_species_entry::WillSpawn;
    use crate::state::GameState;

    #[test]
    fn spawnable_species() {
        let dir_path = current_dir().unwrap().join("test-files").join("journals");

        let log_dir = LogDirReader::open(dir_path);

        let mut state = GameState::new();

        for entry in log_dir {
            state.feed_log_event(&entry.unwrap());
        }

        let mut processed_species_count = 0;

        // A false positive is when a species is calculated to spawn on a body, but it was not observed to spawn
        let mut false_positives_count = 0;

        // A false negative is when a species is observed to spawn on a body, but it was not calculated to spawn
        let mut false_negatives_count = 0;

        // Blacklisted bodies that should not be tested
        let blacklisted_bodies: Vec<String> = vec![
            "Syniechia CB-U d4-8 B 5".to_string(), // Commander did not scan the body before landing
            "Prie Chraea VL-L c21-0 1 c".to_string(), // OsseusDiscus spawned on a body with a non-thin-water atmosphere
            "Syniechou RZ-Z c16-0 7 b a".to_string(), // OsseusDiscus spawned on a body with a non-thin-water atmosphere
            "Flyeia Prou RH-C b46-0 A 8".to_string(), // TubusSororibus spawned on a body with a gravity of 0.52g and temperature of 260K
            "Graea Proae OT-O d7-15 A 4".to_string(), // FrutexaMetallicum, OsseusPellebantus and TussockPropagito spawning on a body that's 0.4K too warm
            "Ruvoe HW-E c11-5 3 b".to_string(), // BacteriumOmentum spawning on a body with a non-neon atmosphere
            "Phoi Aed OH-C d2 3 c".to_string(), // ClypeusSpeculum spawning on a body 0.01au away from the star
            "Prie Chraea UP-G d10-9 C 11 a".to_string(), // CrystallineShards spawning on a body 0.002au away from the star
        ];

        for commander in state.commanders.values() {
            for system in commander.systems.values() {
                for (body_id, planet_state) in &system.bodies {
                    if blacklisted_bodies.contains(&planet_state.scan.body_name) {
                        continue;
                    }

                    let observed_species = &planet_state.scanned_species;

                    if observed_species.is_empty() {
                        continue;
                    }

                    let spawn_source = SpawnSource {
                        target_system: &system.exobiology_system,
                        target_planet: &planet_state.exobiology_body,
                    };

                    let calculated_species = system
                        .get_spawnable_species(*body_id)
                        .unwrap()
                        .iter()
                        .map(|entry| entry.specie.clone())
                        .collect::<HashSet<_>>();

                    // All the observed species that were not calculated to spawn on the body.
                    let false_negatives: Vec<&Species> = observed_species
                        .iter()
                        .filter(|species| !calculated_species.contains(species))
                        .collect();

                    // All the calculated species that were not observed to spawn on the body.
                    let false_positives: Vec<&Species> = calculated_species
                        .iter()
                        .filter(|species| !observed_species.contains(species))
                        .collect();

                    let false_negative_spawn_conditions: Vec<&SpawnCondition> = false_negatives
                        .iter()
                        .flat_map(|species| species.spawn_conditions())
                        .filter(|condition| !spawn_source.satisfies_spawn_condition(condition))
                        .collect();

                    let false_positive_spawn_conditions: Vec<&SpawnCondition> = false_positives
                        .iter()
                        .flat_map(|species| species.spawn_conditions())
                        .filter(|condition| !spawn_source.satisfies_spawn_condition(condition))
                        .collect();

                    if !false_negatives.is_empty() || !false_positives.is_empty() {
                        println!(
                            "Body: {}\nObserved: {:?}\nCalculated: {:?}\nFalse negatives: {:?}\nFalse positives: {:?}\nSpawn source: {:#?}\n",
                            planet_state.scan.body_name,
                            observed_species,
                            calculated_species,
                            false_negative_spawn_conditions,
                            false_positive_spawn_conditions,
                            spawn_source
                        );
                    }

                    processed_species_count += observed_species.len();
                    false_positives_count += false_positives.len();
                    false_negatives_count += false_negatives.len();
                }
            }
        }

        let false_positives_percentage =
            (false_positives_count as f32 / processed_species_count as f32 * 100.0 * 100.0).round()
                / 100.0;
        let false_negatives_percentage =
            (false_negatives_count as f32 / processed_species_count as f32 * 100.0 * 100.0).round()
                / 100.0;

        // Correctness check: 1% or more is considered a failure.
        if false_negatives_percentage > 1.0 {
            println!("False negatives: {}%; {} species were observed to spawn on a body, but were not calculated to spawn.",
                false_negatives_percentage, false_negatives_count
            );
        }

        if false_positives_percentage > 1.0 {
            println!("False positives: {}%; {} species were calculated to spawn on a body, but were not observed to spawn.",
                false_positives_percentage, false_positives_count
            );
        }

        assert!(false_negatives_percentage <= 1.0);
        assert!(false_positives_percentage <= 1.0);
    }
}
