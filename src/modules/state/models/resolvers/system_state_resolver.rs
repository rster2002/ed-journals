use std::collections::HashMap;

use crate::exobiology::{SpawnSourceStar, Species, TargetSystem};
use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::logs::fss_signal_discovered_event::FSSSignalDiscoveredEvent;
use crate::logs::scan_event::ScanEventKind;
use crate::logs::{LogEvent, LogEventContent};
use crate::modules::civilization::LocationInfo;
use crate::state::models::feed_result::FeedResult;
use crate::state::models::resolvers::planet_state_resolver::planet_species_entry::PlanetSpeciesEntry;
use crate::state::PlanetState;
use crate::state::traits::state_resolver::StateResolver;

#[derive(Serialize)]
pub struct SystemStateResolver {
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

impl StateResolver<LogEvent> for SystemStateResolver {
    fn feed(&mut self, input: &LogEvent) -> FeedResult {
        let Some(system_address) = input.content.system_address() else {
            return FeedResult::Skipped;
        };

        if system_address != self.location_info.system_address {
            return FeedResult::Skipped;
        }

        match &input.content {
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
                if let Some(body_id) = input.content.body_id() {
                    let Some(body) = self.bodies.get_mut(&body_id) else {
                        return FeedResult::Later;
                    };

                    body.feed(input);
                }
            }
        }

        FeedResult::Accepted
    }

    fn flush_inner(&mut self) {
        for body in self.bodies.values_mut() {
            body.flush_inner();
        }
    }
}

impl SystemStateResolver {
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

#[cfg(test)]
mod tests {
    use std::env::current_dir;

    use crate::exobiology::SpawnSource;
    use crate::logs::blocking::LogDirReader;
    use crate::state::GameState;
    use crate::state::traits::state_resolver::StateResolver;

    #[test]
    fn spawnable_species_no_false_negatives() {
        let dir_path = current_dir().unwrap().join("test-files").join("journals");

        let log_dir = LogDirReader::open(dir_path);

        let mut state = GameState::default();

        for entry in log_dir {
            state.feed(&entry.unwrap());
        }

        let mut failed = 0;

        // Blacklisted bodies that should not be tested
        let blacklisted_bodies: Vec<String> = vec![
            "Syniechia CB-U d4-8 B 5".to_string(), // Commander did not scan the body before landing
            "Prie Chraea VL-L c21-0 1 c".to_string(), // OsseusDiscus spawned on a body with a non-thin-water atmosphere
            "Syniechou RZ-Z c16-0 7 b a".to_string(), // OsseusDiscus spawned on a body with a non-thin-water atmosphere
            "Flyeia Prou RH-C b46-0 A 8".to_string(), // TubusSororibus spawned on a body with a gravity of 0.52g and temperature of 260K
            "Graea Proae OT-O d7-15 A 4".to_string(), // FrutexaMetallicum, OsseusPellebantus and TussockPropagito spawning on a body that's 0.4K too warm
            "Ruvoe HW-E c11-5 3 b".to_string(), // BacteriumOmentum spawning on a body with a non-neon atmosphere
        ];

        for commander in state.commanders.values() {
            for system in commander.systems.values() {
                for (body_id, planet_state) in &system.bodies {
                    if blacklisted_bodies.contains(&planet_state.scan.body_name) {
                        continue;
                    }

                    let expected_species = system.get_spawnable_species(*body_id).unwrap();

                    let spawn_source = SpawnSource {
                        target_system: &system.exobiology_system,
                        target_planet: &planet_state.exobiology_body,
                    };

                    for species in expected_species {
                        let conditions = species.specie.spawn_conditions();

                        let failing_conditions = conditions
                            .iter()
                            .filter(|condition| !spawn_source.satisfies_spawn_condition(condition))
                            .collect::<Vec<_>>();

                        if !failing_conditions.is_empty() {
                            failed += 1;
                            println!(
                                "The following conditions failed for '{:?}' on body '{}': {:?}\n{:#?}",
                                species, planet_state.scan.body_name, failing_conditions, spawn_source
                            );
                        }
                    }
                }
            }
        }

        // In case of test failure, see the logs printed above.
        assert_eq!(failed, 0);

        // let logs = log_dir.journal_logs().unwrap();

        // // let mut state = ExobiologyState::new();
        //
        // // Species found in the logs, grouped by body name.
        // // These are the value we will compare against the calculated spawnable species.
        // let mut expected_species = HashMap::<String, HashSet<Species>>::new();
        // for journal in &logs {
        //     let reader = journal.create_blocking_reader().unwrap();
        //
        //     let mut body_name = String::new();
        //
        //     for entry in reader.flatten() {
        //         state.feed_event(&entry);
        //
        //         if let LogEventContent::Location(location) = &entry.content {
        //             body_name.clone_from(&location.location_info.body)
        //         }
        //
        //         if let LogEventContent::Touchdown(touchdown) = &entry.content {
        //             body_name.clone_from(&touchdown.body);
        //         }
        //
        //         if let LogEventContent::ScanOrganic(organic) = &entry.content {
        //             expected_species
        //                 .entry(body_name.clone())
        //                 .or_default()
        //                 .insert(organic.species.clone());
        //         }
        //     }
        // }
        //

        //
        // let mut failed = 0;
        //
        // // Check each spawn source to see if the calculated spawnable species match the expected species.
        // for (body_name, expected_species) in expected_species
        //     .iter()
        //     .filter(|(body, _)| !blacklisted_bodies.contains(body))
        // {
        //     let spawn_source = state.for_body(body_name);
        //

        // }
        //
    }
}
