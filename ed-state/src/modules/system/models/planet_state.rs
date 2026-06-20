use crate::modules::state::{EventSink, SinkResult};
use crate::system::models::signal_counts::SignalCounts;
use ed_journals::exobiology::{Genus, Species};
use ed_journals::exploration::{CodexEntry, PlanetarySignalType};
use ed_journals::logs::saa_scan_complete_event::SAAScanCompleteEvent;
use ed_journals::logs::saa_signals_found_event::SAASignalsFoundEventSignal;
use ed_journals::logs::scan_event::ScanEvent;
use ed_journals::logs::scan_organic_event::ScanOrganicEventScanType;
use ed_journals::logs::touchdown_event::TouchdownEvent;
use ed_journals::logs::{LogEvent, LogEventContent};
use ed_journals::trading::Commodity;
use std::collections::HashSet;

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

    /// Species that have been scanned on the planet.
    pub scanned_species: HashSet<Species>,

    /// Species that have been fully logged on the planet.
    pub logged_species: HashSet<Species>,

    /// List of touchdowns on the planet.
    pub touchdowns: Vec<TouchdownEvent>,

    /// Signals found on the planet.
    pub signal_counts: Option<SignalCounts>,

    /// Commodity signals that have been found on the planet.
    pub commodity_signals: Vec<Commodity>,
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
                            signal_counts.human_signal_count += 1;
                        }
                        PlanetarySignalType::Biological => {
                            signal_counts.biological_signal_count += 1;
                        }
                        PlanetarySignalType::Geological => {
                            signal_counts.geological_signal_count += 1;
                        }
                        PlanetarySignalType::Thargoid => {
                            signal_counts.thargoid_signal_count += 1;
                        }
                        PlanetarySignalType::Guardian => {
                            signal_counts.guardian_signal_count += 1;
                        }
                        PlanetarySignalType::Other => {
                            signal_counts.other_signal_count += 1;
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
                self.scanned_species.insert(scanned_organic.species.clone());

                if let ScanOrganicEventScanType::Log = scanned_organic.scan_type {
                    self.logged_species.insert(scanned_organic.species.clone());
                }

                result.accept();
            }
            LogEventContent::CodexEntry(codex_entry) => match &codex_entry.name {
                CodexEntry::Species(species) => {
                    self.scanned_species.insert(species.clone());
                    result.accept();
                }
                CodexEntry::Variant(variant) => {
                    self.scanned_species.insert(variant.species.clone());
                    result.accept();
                }
                _ => {}
            },
            _ => {}
        }

        result
    }
}
