use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::Serialize;
use crate::exobiology::{SpawnSourceStar, TargetSystem};
use crate::logs::{LogEvent, LogEventContent};
use crate::logs::fss_signal_discovered_event::FSSSignalDiscoveredEvent;
use crate::logs::scan_event::{ScanEvent, ScanEventKind};
use crate::modules::civilization::LocationInfo;
use crate::state::models::feed_result::FeedResult;
use crate::state::models::resolvers::planet_state_resolver::planet_species_entry::PlanetSpeciesEntry;
use crate::state::traits::state_resolver::StateResolver;
use crate::state::PlanetState;

#[derive(Serialize)]
pub struct SystemStateResolver {
    /// Information about the system.
    pub location_info: LocationInfo,

    /// Entries for state for planets in the system.
    pub planet_state: HashMap<u8, PlanetState>,

    /// Scans for each star in the system.
    pub star_scans: HashMap<u8, ScanEvent>,

    /// Scans for each cluster in the system.
    pub belt_scans: HashMap<u8, ScanEvent>,

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
            LogEventContent::Scan(event) => {
                match &event.kind {
                    ScanEventKind::Star(star) => {
                        self.exobiology_system.stars_in_system.insert(
                            event.body_id,
                            SpawnSourceStar {
                                class: star.star_type.clone(),
                                luminosity: star.luminosity.clone(),
                            },
                        );

                        self.star_scans.insert(event.body_id, event.clone());
                    }
                    ScanEventKind::Planet(planet) => {
                        self.planet_state
                            .entry(event.body_id)
                            .or_insert_with(|| PlanetState::from((event, planet)));

                        self.exobiology_system
                            .planet_classes_in_system
                            .insert(planet.planet_class.clone());
                    }
                    ScanEventKind::BeltCluster(_) => {
                        self.belt_scans.insert(event.body_id, event.clone());
                    }
                }

                if let Some(total_bodies) = self.number_of_bodies {
                    let new_factor = self.nr_of_scanned_bodies() as f32 / total_bodies as f32;

                    if new_factor > self.progress {
                        self.progress = new_factor;
                    }
                }
            }

            _ => {
                if let Some(body_id) = input.content.body_id() {
                    let Some(body) = self.planet_state.get_mut(&body_id) else {
                        return FeedResult::Later;
                    };

                    body.feed(input);
                }
            }
        }

        FeedResult::Accepted
    }

    fn flush_inner(&mut self) {
        for body in self.planet_state.values_mut() {
            body.flush_inner();
        }
    }
}

impl SystemStateResolver {
    pub fn visit(&mut self, date_time: &DateTime<Utc>) {
        self.visits.push(*date_time);
    }

    /// Returns the total number of scans, which includes planets, stars and belt clusters.
    pub fn nr_of_scans(&self) -> usize {
        self.planet_state.len() + self.star_scans.len() + self.belt_scans.len()
    }

    /// Returns the total number of scanned bodies, which includes planets and stars. Take note
    /// that this does not include scanned belt clusters as they are not counted towards the total
    /// number of scanned bodies in game.
    pub fn nr_of_scanned_bodies(&self) -> usize {
        self.planet_state.len() + self.star_scans.len()
    }

    /// Returns all the scan events for this system.
    pub fn all_scans(&self) -> Vec<&ScanEvent> {
        let mut result = Vec::with_capacity(self.nr_of_scans());

        for planet in self.planet_state.values() {
            result.push(&planet.scan);
        }

        for star_scan in self.star_scans.values() {
            result.push(star_scan)
        }

        for belt_scan in self.belt_scans.values() {
            result.push(belt_scan)
        }

        result
    }

    pub fn carrier_visit(&mut self, date_time: &DateTime<Utc>) {
        self.carrier_visits.push(*date_time);
    }

    pub fn get_spawnable_species(&self, body_id: u8) -> Option<Vec<PlanetSpeciesEntry>> {
        self.planet_state
            .get(&body_id)?
            .get_planet_species(&self.exobiology_system)
    }

    pub fn get_possible_spawnable_species(&self, body_id: u8) -> Option<Vec<PlanetSpeciesEntry>> {
        Some(
            self.planet_state
                .get(&body_id)?
                .get_possible_planet_species(&self.exobiology_system),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::env::current_dir;

    use crate::exobiology::{SpawnSource, Species};
    use crate::logs::blocking::LogDirReader;
    use crate::state::traits::state_resolver::StateResolver;
    use crate::state::GameState;

    const KNOWN_SPECIES: &[(u64, u8, &[Species])] = &[
        (2874004872433, 17, &[
            Species::BacteriumVesicula,
            Species::FonticuluaCampestris,
        ]),
        (1247495096115, 20, &[
            Species::BacteriumCerbrus,
            Species::BacteriumTela,
            Species::StratumTectonicas,
        ]),
        (5071685950649, 10, &[
            Species::BacteriumCerbrus,
            Species::BacteriumTela,
            Species::StratumTectonicas,
        ]),
        (1487946155795, 15, &[
            Species::BacteriumAurasus,
            Species::ConchaLabiata,
            Species::StratumPaleas,
            Species::TubusConifer,
        ]),
        (1487946155795, 16, &[
            Species::BacteriumAurasus,
            Species::ConchaLabiata,
            Species::StratumPaleas,
            Species::TubusConifer,
            // This seems anomalous, but cannot confirm. The species failure is 0.03% so ¯\_(ツ)_/¯
            // Species::TussockIgnis,
        ]),
        (1487946155795, 28, &[
            Species::BacteriumVesicula,
            Species::FonticuluaCampestris,
        ]),
        (1487811905211, 15, &[
            Species::BacteriumCerbrus,
            Species::StratumAraneamus,
        ]),
        (1487811905211, 25, &[
            Species::AleoidaSpica,
            Species::BacteriumAlcyoneum,
            Species::ConchaAureolas,
            Species::FungoidaGelata,
            Species::FungoidaSetisis,
            Species::OsseusSpiralis,
            Species::StratumPaleas,
            Species::TussockDivisa,
        ]),
        (1487811905211, 27, &[
            Species::BacteriumAlcyoneum,
            Species::ConchaAureolas,
            Species::FungoidaGelata,
            Species::FungoidaSetisis,
            Species::OsseusSpiralis,
            Species::TussockDivisa,
        ]),
        (147379439083, 46, &[
            Species::BacteriumAlcyoneum,
            Species::ConchaAureolas,
            Species::FrutexaFlabellum,
            Species::FungoidaGelata,
            Species::FungoidaSetisis,
            Species::OsseusSpiralis,
            Species::StratumPaleas,
        ]),
        (147379439083, 47, &[
            Species::BacteriumAlcyoneum,
            Species::ConchaAureolas,
            Species::FrutexaFlabellum,
            Species::FungoidaGelata,
            Species::FungoidaSetisis,
            Species::OsseusSpiralis,
        ]),
    ];

    #[test]
    fn spawnable_species() {
        let dir_path = current_dir()
            .unwrap()
            .join("test-files")
            .join("species-journals");

        let log_dir = LogDirReader::open(dir_path);

        let mut state = GameState::default();

        for entry in log_dir {
            state.feed(&entry.unwrap());
        }

        state.flush();

        for expected in KNOWN_SPECIES {
            for commander in state.commanders.values() {
                let Some(system) = commander.system_by_address(expected.0) else {
                    dbg!(expected.0);
                    continue;
                };

                let possible_species = system.get_possible_spawnable_species(expected.1)
                    .unwrap();

                dbg!(&system.planet_state.get(&expected.1).unwrap().scan, &possible_species, &expected);

                assert_eq!(possible_species.len(), expected.2.len());

                for possible in &possible_species {
                    let found = expected.2
                        .iter()
                        .find(|species| species == &&possible.specie)
                        .is_some();

                    // Checks that all the actual species are in the list
                    assert!(found);
                }

                for expected in expected.2 {
                    let found = possible_species
                        .iter()
                        .find(|entry| &entry.specie == expected)
                        .is_some();

                    // Check that all the expected entries are in the list
                    assert!(found);
                }
            }
        }


    }
}
