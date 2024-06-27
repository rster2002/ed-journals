use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::exobiology::{SpawnSourceStar, TargetSystem};
use crate::logs::{LogEvent, LogEventContent};
use crate::logs::fss_signal_discovered_event::FSSSignalDiscoveredEvent;
use crate::logs::scan_event::ScanEventKind;
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
        self.bodies
            .get(&body_id)?
            .get_planet_species(&self.exobiology_system)
    }

    pub fn get_possible_spawnable_species(&self, body_id: u8) -> Option<Vec<PlanetSpeciesEntry>> {
        Some(
            self.bodies
                .get(&body_id)?
                .get_possible_planet_species(&self.exobiology_system),
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

    use crate::exobiology::Species;
    use crate::logs::blocking::LogDirReader;
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
            Species::TussockIgnis,
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
    ];

    #[test]
    fn spawnable_species() {
        let dir_path = current_dir()
            .unwrap()
            .join("test-files")
            .join("species-journals");

        let log_dir = LogDirReader::open(dir_path);

        let mut state = GameState::new();

        for entry in log_dir {
            state.feed_log_event(&entry.unwrap());
        }

        state.flush();

        for expected in KNOWN_SPECIES {
            for commander in state.commanders.values() {
                let Some(system) = commander.system_by_address(expected.0) else {
                    continue;
                };

                let possible_species = system.get_possible_spawnable_species(expected.1)
                    .unwrap();

                dbg!(&system.bodies.get(&expected.1).unwrap().scan, &possible_species, &expected);

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
