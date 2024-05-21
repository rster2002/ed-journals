use std::collections::HashMap;
use serde::Serialize;
use crate::logs::content::log_event_content::scan_event::ScanEvent;
use crate::logs::content::{LogEvent, LogEventContent};
use crate::logs::content::log_event_content::fss_body_signals_event::FSSBodySignalEventSignal;
use crate::logs::content::log_event_content::saa_scan_complete_event::SAAScanCompleteEvent;
use crate::logs::content::log_event_content::saa_signals_found_event::{SAASignalsFoundEventSignal};
use crate::logs::content::log_event_content::touchdown_event::TouchdownEvent;
use crate::modules::exobiology::{Genus, Species};
use crate::state::models::feed_result::FeedResult;
use crate::state::models::organic_state::OrganicState;

#[derive(Debug, Serialize)]
pub struct BodyState {
    pub scan: ScanEvent,

    pub fss_signals: Vec<FSSBodySignalEventSignal>,

    pub saa_scan: Option<SAAScanCompleteEvent>,
    pub saa_signals: Vec<SAASignalsFoundEventSignal>,
    pub saa_genuses: Vec<Genus>,

    pub touchdowns: Vec<TouchdownEvent>,
    pub organics: HashMap<Species, OrganicState>,

    // pub scanned_organics: HashMap<>

    // pub confirmed_species: Vec<Species>,
    // pub variants_species: Vec<Species>,
    // pub scanned_organics: Vec<>
}

impl BodyState {
    pub fn new(scan_event: ScanEvent) -> Self {
        BodyState {
            scan: scan_event,
            fss_signals: Vec::new(),
            saa_scan: None,
            saa_signals: Vec::new(),
            saa_genuses: Vec::new(),
            touchdowns: Vec::new(),
            organics: HashMap::new(),
        }
    }

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
            },
            LogEventContent::SAASignalsFound(signals) => {
                self.saa_signals.clone_from(&signals.signals);
                self.saa_genuses = signals.genuses.iter()
                    .map(|signal| signal.genus.clone())
                    .collect();
            },
            LogEventContent::FSSBodySignals(body_signals) => {
                self.fss_signals.clone_from(&body_signals.signals);
            },
            LogEventContent::Touchdown(touchdown) => {
                if touchdown.on_planet {
                    self.touchdowns.push(touchdown.clone());
                }
            },
            LogEventContent::ScanOrganic(scanned_organic) => {

            },

            _ => {},
        }

        FeedResult::Accepted
    }

    pub fn clear_organic_process(&mut self) {
        for organic in self.organics.values_mut() {
            organic.clear_process();
        }
    }
}
