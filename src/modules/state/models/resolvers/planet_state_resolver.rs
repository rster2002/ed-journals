pub mod planet_species_entry;
mod signal_counts;

use std::collections::{HashMap, HashSet};

use crate::exobiology::{SpawnSource, TargetPlanet, TargetSystem};
use crate::exploration::{CodexEntry, PlanetarySignalType};
use serde::Serialize;
use thiserror::Error;

use crate::logs::saa_scan_complete_event::SAAScanCompleteEvent;
use crate::logs::saa_signals_found_event::SAASignalsFoundEventSignal;
use crate::logs::scan_event::{
    ScanEvent, ScanEventKind, ScanEventPlanet,
};
use crate::logs::touchdown_event::TouchdownEvent;
use crate::logs::{LogEvent, LogEventContent};
use crate::logs::scan_organic_event::ScanOrganicEventScanType;
use crate::modules::exobiology::{Genus, Species};
use crate::state::models::feed_result::FeedResult;
use crate::state::models::resolvers::planet_state_resolver::planet_species_entry::{PlanetSpeciesEntry, WillSpawn};
use crate::state::models::resolvers::planet_state_resolver::signal_counts::SignalCounts;
use crate::state::traits::state_resolver::StateResolver;
use crate::trading::Commodity;

#[derive(Debug, Serialize)]
pub struct PlanetStateResolver {
    pub scan: ScanEvent,

    pub saa_scan: Option<SAAScanCompleteEvent>,
    pub saa_signals: Vec<SAASignalsFoundEventSignal>,
    pub saa_genuses: Option<HashSet<Genus>>,

    pub scanned_species: HashSet<Species>,
    pub logged_species: HashSet<Species>,

    pub touchdowns: Vec<TouchdownEvent>,
    pub exobiology_body: TargetPlanet,

    pub signal_counts: Option<SignalCounts>,
    // pub human_signal_count: Option<usize>,
    // pub biological_signal_count: Option<usize>,
    // pub geological_signal_count: Option<usize>,
    // pub thargoid_signal_count: Option<usize>,
    // pub guardian_signal_count: Option<usize>,
    // pub other_signal_count: Option<usize>,
    pub commodity_signals: Vec<Commodity>,
}

#[derive(Debug, Error)]
pub enum PlanetStateError {
    #[error("The provided scan event is not that of a planet")]
    NotAPlanetScan,
}

impl StateResolver<LogEvent> for PlanetStateResolver {
    fn feed(&mut self, input: &LogEvent) -> FeedResult {
        let Some(body_id) = input.content.body_id() else {
            return FeedResult::Skipped;
        };

        if body_id != self.scan.body_id {
            return FeedResult::Skipped;
        }

        match &input.content {
            LogEventContent::SAAScanComplete(scan_complete) => {
                self.saa_scan = Some(scan_complete.clone());
            }
            LogEventContent::SAASignalsFound(signals) => {
                self.saa_signals.clone_from(&signals.signals);

                self.saa_genuses = Some(signals
                    .genuses
                    .iter()
                    .map(|signal| signal.genus.clone())
                    .collect());
            }
            LogEventContent::FSSBodySignals(body_signals) => {
                let mut signal_counts = SignalCounts {
                    human_signal_count: 0,
                    biological_signal_count: 0,
                    geological_signal_count: 0,
                    thargoid_signal_count: 0,
                    guardian_signal_count: 0,
                    other_signal_count: 0,
                };

                for signal in &body_signals.signals {
                    match &signal.kind {
                        PlanetarySignalType::Human => { signal_counts.human_signal_count += 1; }
                        PlanetarySignalType::Biological => { signal_counts.biological_signal_count += 1; }
                        PlanetarySignalType::Geological => { signal_counts.geological_signal_count += 1; }
                        PlanetarySignalType::Thargoid => { signal_counts.thargoid_signal_count += 1; }
                        PlanetarySignalType::Guardian => { signal_counts.guardian_signal_count += 1; }
                        PlanetarySignalType::Other => { signal_counts.other_signal_count += 1; }
                        PlanetarySignalType::Commodity(commodity) => { self.commodity_signals.push(commodity.clone()); }
                        _ => {},
                    }
                }

                self.signal_counts = Some(signal_counts);
            }
            LogEventContent::Touchdown(touchdown) => {
                if touchdown.on_planet {
                    self.touchdowns.push(touchdown.clone());
                }
            },
            LogEventContent::ScanOrganic(scanned_organic) => {
                self.scanned_species.insert(scanned_organic.species.clone());

                if let ScanOrganicEventScanType::Log = scanned_organic.scan_type {
                    self.logged_species.insert(scanned_organic.species.clone());
                }
            },
            LogEventContent::CodexEntry(codex_entry) => {
                match &codex_entry.name {
                    CodexEntry::Species(species) => {
                        self.scanned_species.insert(species.clone());
                    },
                    CodexEntry::Variant(variant) => {
                        self.scanned_species.insert(variant.species.clone());
                    },
                    _ => {},
                }
            },
            _ => {}
        }

        FeedResult::Accepted
    }
}

impl PlanetStateResolver {
    pub fn has_human_signals(&self) -> bool {
        self.signal_counts
            .as_ref()
            .is_some_and(|signals| signals.human_signal_count != 0)
    }

    pub fn has_biological_signals(&self) -> bool {
        self.signal_counts
            .as_ref()
            .is_some_and(|signals| signals.biological_signal_count != 0)
    }

    pub fn has_geological_signals(&self) -> bool {
        self.signal_counts
            .as_ref()
            .is_some_and(|signals| signals.geological_signal_count != 0)
    }

    pub fn has_thargoid_signals(&self) -> bool {
        self.signal_counts
            .as_ref()
            .is_some_and(|signals| signals.thargoid_signal_count != 0)
    }

    pub fn has_guardian_signals(&self) -> bool {
        self.signal_counts
            .as_ref()
            .is_some_and(|signals| signals.guardian_signal_count != 0)
    }

    pub fn has_other_signals(&self) -> bool {
        self.signal_counts
            .as_ref()
            .is_some_and(|signals| signals.other_signal_count != 0)
    }

    /// Returns entries for all species that could theoretically spawn on the planet and indicates
    /// if they can actually spawn or not.
    pub fn get_planet_species(&self, target_system: &TargetSystem) -> Vec<PlanetSpeciesEntry> {
        let spawn_source = SpawnSource {
            target_system,
            target_planet: &self.exobiology_body,
        };

        if !self.has_biological_signals() {
            return Vec::new();
        }

        let species = spawn_source.get_spawnable_species();
        let number_of_species = species.len();

        species
            .into_iter()
            .map(|species| {
                let will_spawn: WillSpawn = match true {
                    // If the species has been scanned, will_spawn will be set to yes to keep it
                    // from switching from a maybe to a no.
                    _ if self.scanned_species.contains(&species) => WillSpawn::Yes,

                    // If the possible number of species is the same as the number of biological
                    // signals it counts all of them as yes.
                    _ if self.signal_counts
                        .as_ref()
                        .is_some_and(|signals| signals.biological_signal_count == number_of_species) => WillSpawn::Yes,

                    // If the current species has not been scanned yet (checked by the first if
                    // statement), but there already is another species of the same genus, then
                    // this species does not have a chance to spawn.
                    _ if self.scanned_species
                        .iter()
                        .find(|scanned| scanned.genus() == species.genus())
                        .is_some() => WillSpawn::No,

                    // If the planet has not been scanned yet and the genuses are still unknown, it
                    // will count any species that hasn't already been flagged as a maybe.
                    _ if self.saa_genuses.is_none() => WillSpawn::Maybe,

                    // If the planet has been scanned, but the species' genus does not appear in the
                    // list of scanned genuses that can spawn, then the current species will not
                    // spawn.
                    _ if self.saa_genuses
                        .as_ref()
                        .is_some_and(|genuses| !genuses.contains(&species.genus())) => WillSpawn::No,

                    // If the species is not handled by any of the special cases above, then the
                    // species is still under consideration.
                    _ => WillSpawn::Maybe,
                };

                PlanetSpeciesEntry {
                    will_spawn,
                    scanned: self.scanned_species.contains(&species),
                    logged: self.logged_species.contains(&species),
                    specie: species,
                }
            })
            .collect()
    }

    /// Calculates the lowest exobiology value based on the current information about the planet.
    pub fn get_lowest_exobiology_value(&self, target_system: &TargetSystem) -> u64 {
        let mut known_values = Vec::new();
        let mut maybe_values = Vec::new();

        for entry in self.get_planet_species(target_system) {
            match entry.will_spawn {
                WillSpawn::Yes => known_values.push(entry.specie.base_value()),
                WillSpawn::Maybe => maybe_values.push(entry.specie.base_value()),
                WillSpawn::No => {}
            }
        }

        // Calculates how many of the maybe entries could still be found.
        let remaining_unknowns = match &self.signal_counts {
            Some(signals) => signals.biological_signal_count - known_values.len(),
            None => known_values.len(),
        };

        maybe_values.sort();

        let known_total: u64 = maybe_values
            .iter()
            .sum();

        let maybe_total: u64 = maybe_values
            .iter()
            .take(remaining_unknowns)
            .sum();

        known_total + maybe_total
    }
}
