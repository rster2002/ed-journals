pub mod planet_species_entry;

use std::collections::{HashMap, HashSet};

use crate::exobiology::{SpawnSource, TargetPlanet, TargetSystem};
use crate::exploration::PlanetarySignalType;
use serde::Serialize;
use thiserror::Error;

use crate::logs::content::log_event_content::fss_body_signals_event::FSSBodySignalEventSignal;
use crate::logs::content::log_event_content::saa_scan_complete_event::SAAScanCompleteEvent;
use crate::logs::content::log_event_content::saa_signals_found_event::SAASignalsFoundEventSignal;
use crate::logs::content::log_event_content::scan_event::{
    ScanEvent, ScanEventKind, ScanEventPlanet,
};
use crate::logs::content::log_event_content::touchdown_event::TouchdownEvent;
use crate::logs::content::{LogEvent, LogEventContent};
use crate::logs::content::log_event_content::scan_organic_event::ScanOrganicEventScanType;
use crate::modules::exobiology::{Genus, Species};
use crate::state::models::feed_result::FeedResult;
use crate::state::models::planet_state::planet_species_entry::{PlanetSpeciesEntry, WillSpawn};
use crate::trading::Commodity;

#[derive(Debug, Serialize)]
pub struct PlanetState {
    pub scan: ScanEvent,

    pub saa_scan: Option<SAAScanCompleteEvent>,
    pub saa_signals: Vec<SAASignalsFoundEventSignal>,
    pub saa_genuses: Option<HashSet<Genus>>,

    pub scanned_species: HashSet<Species>,
    pub logged_species: HashSet<Species>,

    pub touchdowns: Vec<TouchdownEvent>,
    pub exobiology_body: TargetPlanet,

    pub human_signal_count: Option<usize>,
    pub biological_signal_count: Option<usize>,
    pub geological_signal_count: Option<usize>,
    pub thargoid_signal_count: Option<usize>,
    pub guardian_signal_count: Option<usize>,
    pub other_signal_count: Option<usize>,
    pub commodity_signals: Vec<Commodity>,
}

#[derive(Debug, Error)]
pub enum BodyStateError {
    #[error("The provided scan event is not that of a planet")]
    NotAPlanetScan,
}

impl PlanetState {
    pub fn feed_log_event(&mut self, log_event: &LogEvent) -> FeedResult {
        let Some(body_id) = log_event.content.body_id() else {
            return FeedResult::Skipped;
        };

        if body_id != self.scan.body_id {
            return FeedResult::Skipped;
        }

        match &log_event.content {
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
                self.human_signal_count = Some(body_signals
                    .signals
                    .iter()
                    .filter(|signal| signal.kind == PlanetarySignalType::Human)
                    .count());

                self.biological_signal_count = Some(body_signals
                    .signals
                    .iter()
                    .filter(|signal| signal.kind == PlanetarySignalType::Biological)
                    .count());

                self.geological_signal_count = Some(body_signals
                    .signals
                    .iter()
                    .filter(|signal| signal.kind == PlanetarySignalType::Geological)
                    .count());

                self.thargoid_signal_count = Some(body_signals
                    .signals
                    .iter()
                    .filter(|signal| signal.kind == PlanetarySignalType::Thargoid)
                    .count());

                self.guardian_signal_count = Some(body_signals
                    .signals
                    .iter()
                    .filter(|signal| signal.kind == PlanetarySignalType::Guardian)
                    .count());

                self.other_signal_count = Some(body_signals
                    .signals
                    .iter()
                    .filter(|signal| signal.kind == PlanetarySignalType::Other)
                    .count());

                self.commodity_signals = body_signals
                    .signals
                    .iter()
                    .filter_map(|entry | {
                        if let PlanetarySignalType::Commodity(commodity) = &entry.kind {
                            Some(commodity.clone())
                        } else {
                            None
                        }
                    })
                    .collect()
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
            _ => {}
        }

        FeedResult::Accepted
    }

    /// Returns entries for all species that could theoretically spawn on the planet and indicates
    /// if they can actually spawn or not.
    pub fn get_planet_species(&self, target_system: &TargetSystem) -> Vec<PlanetSpeciesEntry> {
        let spawn_source = SpawnSource {
            target_system,
            target_planet: &self.exobiology_body,
        };

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
                    _ if self.biological_signal_count.is_some_and(|count| count == number_of_species) => WillSpawn::Yes,

                    // If the current species has not been scanned yet (checked by the first if
                    // statement), but there already is another species of the same genus, then
                    // this species does not have a chance to spawn.
                    _ if self.scanned_species.iter().find(|scanned| scanned.genus() == species.genus()).is_none() => WillSpawn::No,

                    // If the planet has not been scanned yet and the genuses are still unknown, it
                    // will count any species that hasn't already been flagged as a maybe.
                    _ if self.saa_genuses.is_none() => WillSpawn::Maybe,

                    // If the planet has been scanned, but the species' genus does not appear in the
                    // list of scanned genuses that can spawn, then the current species will not
                    // spawn.
                    _ if self.saa_genuses.as_ref().is_some_and(|genuses| !genuses.contains(&species.genus())) => WillSpawn::No,

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
        let remaining_unknowns = match self.biological_signal_count {
            Some(max) => max - known_values.len(),
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

impl From<(&ScanEvent, &ScanEventPlanet)> for PlanetState {
    fn from(value: (&ScanEvent, &ScanEventPlanet)) -> Self {
        PlanetState {
            scan: value.0.clone(),
            saa_scan: None,
            saa_signals: Vec::new(),
            saa_genuses: None,
            scanned_species: HashSet::new(),
            touchdowns: Vec::new(),
            logged_species: HashSet::new(),
            human_signal_count: None,
            biological_signal_count: None,
            geological_signal_count: None,
            thargoid_signal_count: None,
            guardian_signal_count: None,
            other_signal_count: None,
            commodity_signals: Vec::new(),
            exobiology_body: TargetPlanet {
                atmosphere: value.1.atmosphere.clone(),
                gravity: value.1.surface_gravity.clone(),
                class: value.1.planet_class.clone(),
                surface_temperature: value.1.surface_temperature,
                volcanism: value.1.volcanism.clone(),
                materials: HashSet::from_iter(
                    value
                        .1
                        .materials
                        .clone()
                        .into_iter()
                        .map(|entry| entry.name),
                ),
                composition: value.1.composition.clone(),
                parents: value.0.parents.clone(),
                semi_major_axis: value.1.orbit_info.semi_major_axis,
                geological_signals_present: false,
            },
        }
    }
}

impl TryFrom<&ScanEvent> for PlanetState {
    type Error = BodyStateError;

    fn try_from(value: &ScanEvent) -> Result<Self, Self::Error> {
        let ScanEventKind::Planet(planet) = &value.kind else {
            return Err(BodyStateError::NotAPlanetScan);
        };

        Ok(PlanetState::from((value, planet)))
    }
}
