use crate::modules::state::{EventSink, SinkResult};
use crate::system::models::planet_organic::{PlanetOrganic, PlanetOrganicScan};
use crate::system::models::signal_counts::SignalCounts;
use ed_journals::exobiology::Genus;
use ed_journals::exploration::{CodexEntry, PlanetarySignalType};
use ed_journals::logs::saa_scan_complete_event::SAAScanCompleteEvent;
use ed_journals::logs::saa_signals_found_event::SAASignalsFoundEventSignal;
use ed_journals::logs::scan_event::{ScanEvent};
use ed_journals::logs::scan_organic_event::ScanOrganicEventScanType;
use ed_journals::logs::touchdown_event::TouchdownEvent;
use ed_journals::logs::{LogEvent, LogEventContent};
use ed_journals::status::{PlanetStatus, Status};
use ed_journals::trading::Commodity;
use std::collections::{HashMap, HashSet};

cfg_select! {
    feature = "exobiology" => {
        use ed_exobiology::{SpawnSource, TargetSystem, TargetPlanet};
        use ed_journals::galaxy::LocalDistance;
        use ed_journals::logs::scan_event::ScanEventKind;
        use crate::system::{PlanetSpeciesEntry, WillSpawn};
    }
    _ => {}
}

#[derive(Debug, Default, Clone)]
pub struct PlanetState {
    pub body_id: u8,
    pub scan: Option<ScanEvent>,

    /// The SAA scan for this planet, if any.
    pub saa_scan: Option<SAAScanCompleteEvent>,

    /// List of SAA signals for this planet.
    pub saa_signals: Vec<SAASignalsFoundEventSignal>,

    /// List of genuses found by SAA, if any.
    pub saa_genuses: Option<HashSet<Genus>>,

    /// Organics that were scanned on this planet.
    pub organics: HashMap<Genus, PlanetOrganic>,
    //
    // // /// Species that have been scanned on the planet.
    // // pub scanned_species: HashSet<Species>,
    // //
    // // /// Species that have been fully logged on the planet.
    // // pub logged_species: HashSet<Species>,
    //
    // /// The actual locations if the different organics that were scanned on the planet.
    // pub organic_locations: Vec<PlanetOrganic>,
    /// List of touchdowns on the planet.
    pub touchdowns: Vec<TouchdownEvent>,

    /// Signals found on the planet.
    pub signal_counts: Option<SignalCounts>,

    /// Commodity signals that have been found on the planet.
    pub commodity_signals: Vec<Commodity>,

    /// The status of the player on the current planet.
    pub planet_status: Option<PlanetStatus>,

    /// Information about the planet needed for exobiology predictions.
    #[cfg(feature = "exobiology")]
    pub exobiology_body: Option<TargetPlanet>,
}

impl PlanetState {
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

    #[cfg(feature = "exobiology")]
    pub fn target_planet(&self) -> &Option<TargetPlanet> {
        &self.exobiology_body
    }

    /// Returns entries for all species that could theoretically spawn on the planet and indicates
    /// if they can actually spawn or not.
    #[cfg(feature = "exobiology")]
    pub fn get_planet_species(&self, target_system: &TargetSystem) -> Vec<PlanetSpeciesEntry> {
        let Some(exobiology_body) = &self.exobiology_body else {
            return Vec::new();
        };

        if !exobiology_body.landable || !self.has_biological_signals() {
            return Vec::new();
        }

        let spawn_source = SpawnSource {
            target_system,
            target_planet: &exobiology_body,
        };

        let predictions = spawn_source.get_spawnable_species();
        let number_of_species = predictions.len();

        predictions
            .into_iter()
            .map(|species| {
                let will_spawn: WillSpawn = match true {
                    // If the species has been scanned, will_spawn will be set to yes to keep it
                    // from switching from a maybe to a no.
                    _ if self
                        .organics
                        .get(&species.genus())
                        .is_some_and(|organic| organic.species == species) =>
                    {
                        WillSpawn::Yes
                    }

                    // If the possible number of species is the same as the number of biological
                    // signals it counts all of them as yes.
                    _ if self.signal_counts.as_ref().is_some_and(|signals| {
                        signals.biological_signal_count == number_of_species
                    }) =>
                    {
                        WillSpawn::Yes
                    }

                    // If the current species has not been scanned yet (checked by the first if
                    // statement), but there already is another species of the same genus, then
                    // this species does not have a chance to spawn.
                    _ if self.organics.contains_key(&species.genus()) => WillSpawn::No,

                    // If the planet has not been scanned yet and the genuses are still unknown, it
                    // will count any species that hasn't already been flagged as a maybe.
                    _ if self.saa_genuses.is_none() => WillSpawn::Maybe,

                    // If the planet has been scanned, but the species' genus does not appear in the
                    // list of scanned genuses that can spawn, then the current species will not
                    // spawn.
                    _ if self
                        .saa_genuses
                        .as_ref()
                        .is_some_and(|genuses| !genuses.contains(&species.genus())) =>
                    {
                        WillSpawn::No
                    }

                    // If the species is not handled by any of the special cases above, then the
                    // species is still under consideration.
                    _ => WillSpawn::Maybe,
                };

                PlanetSpeciesEntry {
                    will_spawn,

                    confirmed: self
                        .organics
                        .get(&species.genus())
                        .is_some_and(|organic| organic.species == species),

                    completed: self.organics.get(&species.genus()).is_some_and(|organic| {
                        organic.species == species && organic.is_completed()
                    }),

                    species,
                }
            })
            .collect()
    }

    /// Calculates the lowest exobiology value based on the current information about the planet.
    #[cfg(feature = "exobiology")]
    pub fn get_lowest_exobiology_value(&self, target_system: &TargetSystem) -> u64 {
        let mut known_values = Vec::new();
        let mut maybe_values = Vec::new();

        for entry in self.get_planet_species(target_system) {
            match entry.will_spawn {
                WillSpawn::Yes => known_values.push(entry.species.base_value()),
                WillSpawn::Maybe => maybe_values.push(entry.species.base_value()),
                WillSpawn::No => {}
            }
        }

        // Calculates how many of the maybe entries could still be found.
        let remaining_unknowns = match &self.signal_counts {
            Some(signals) => signals.biological_signal_count - known_values.len(),
            None => known_values.len(),
        };

        maybe_values.sort();

        let known_total: u64 = known_values.iter().sum();
        let maybe_total: u64 = maybe_values.iter().take(remaining_unknowns).sum();

        known_total + maybe_total
    }
}

impl From<u8> for PlanetState {
    fn from(value: u8) -> Self {
        PlanetState {
            body_id: value,
            ..Default::default()
        }
    }
}

impl EventSink for PlanetState {
    fn sink_log(&mut self, log_event: &LogEvent) -> SinkResult {
        let mut result = SinkResult::Ignored;

        if log_event
            .content
            .body_id()
            .is_none_or(|body_id| body_id != self.body_id)
        {
            return result;
        }

        match &log_event.content {
            LogEventContent::Scan(event) if event.kind.is_planet() => {
                self.scan = Some(event.clone());

                #[cfg(feature = "exobiology")]
                if let ScanEventKind::Planet(planet) = &event.kind {
                    self.exobiology_body = Some(TargetPlanet {
                        landable: planet.landable,
                        class: planet.planet_class.clone(),
                        atmosphere: planet.atmosphere.clone(),
                        surface_gravity: planet.surface_gravity.clone(),
                        surface_temperature: planet.surface_temperature,
                        surface_pressure: planet.surface_pressure,
                        volcanism: planet.volcanism.clone(),
                        materials: HashSet::from_iter(
                            planet.materials.clone().into_iter().map(|entry| entry.name),
                        ),
                        composition: planet.composition.clone(),
                        parents: event.parents.clone(),
                        semi_major_axis: LocalDistance::from_m(planet.orbit_info.semi_major_axis),
                        geological_signals_present: false,
                    });
                }

                result.accept();
            }
            LogEventContent::SAAScanComplete(scan_complete) => {
                self.saa_scan = Some(scan_complete.clone());
                result.accept();
            }
            LogEventContent::SAASignalsFound(signals) => {
                self.saa_signals.clone_from(&signals.signals);

                self.saa_genuses = Some(
                    signals
                        .genuses
                        .iter()
                        .map(|signal| signal.genus.clone())
                        .collect(),
                );

                result.accept();
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
                        PlanetarySignalType::Human => {
                            signal_counts.human_signal_count += signal.count as usize;
                        }
                        PlanetarySignalType::Biological => {
                            signal_counts.biological_signal_count += signal.count as usize;
                        }
                        PlanetarySignalType::Geological => {
                            signal_counts.geological_signal_count += signal.count as usize;
                        }
                        PlanetarySignalType::Thargoid => {
                            signal_counts.thargoid_signal_count += signal.count as usize;
                        }
                        PlanetarySignalType::Guardian => {
                            signal_counts.guardian_signal_count += signal.count as usize;
                        }
                        PlanetarySignalType::Other => {
                            signal_counts.other_signal_count += signal.count as usize;
                        }
                        PlanetarySignalType::Commodity(commodity) => {
                            self.commodity_signals.push(commodity.clone());
                        }
                        _ => {}
                    }
                }

                self.signal_counts = Some(signal_counts);
                result.accept();
            }
            LogEventContent::Touchdown(touchdown) if touchdown.on_planet => {
                self.touchdowns.push(touchdown.clone());
                result.accept();
            }
            LogEventContent::ScanOrganic(scanned_organic) => {
                let entry = self
                    .organics
                    .entry(scanned_organic.genus.clone())
                    .or_insert_with(|| PlanetOrganic::from(scanned_organic.species.clone()));

                entry.variant = entry.variant.clone().or(scanned_organic.variant.clone());

                let location = self
                    .planet_status
                    .as_ref()
                    .map(|status| (status.latitude, status.longitude));

                match scanned_organic.scan_type {
                    ScanOrganicEventScanType::Log => {
                        entry.first_scan = Some(PlanetOrganicScan {
                            scan: scanned_organic.clone(),
                            location,
                        });
                    }
                    ScanOrganicEventScanType::Sample => {
                        entry.second_scan = Some(PlanetOrganicScan {
                            scan: scanned_organic.clone(),
                            location,
                        });
                    }
                    ScanOrganicEventScanType::Analyse => {
                        entry.third_scan_scan = Some(PlanetOrganicScan {
                            scan: scanned_organic.clone(),
                            location,
                        });
                    }
                }

                if let Some(status) = &self.planet_status {
                    entry
                        .scan_locations
                        .push((status.latitude, status.longitude));
                }
            }
            LogEventContent::CodexEntry(codex_entry) => match &codex_entry.name {
                CodexEntry::Species(species) => {
                    self.organics.entry(species.genus())
                        .or_insert_with(|| PlanetOrganic::from(species.clone()));

                    result.accept();
                }
                CodexEntry::Variant(variant) => {
                    let entry = self.organics.entry(variant.genus())
                        .or_insert_with(|| PlanetOrganic::from(variant.species.clone()));

                    // Set here to handle already existing entry for the species.
                    entry.variant = Some(variant.clone());

                    result.accept();
                }
                _ => {}
            },
            _ => {}
        }

        result
    }

    fn sink_status(&mut self, status: &Status) -> SinkResult {
        self.planet_status = status
            .contents
            .as_ref()
            .and_then(|status| status.planet_status.clone());

        SinkResult::from(self.planet_status.is_some())
    }
}
