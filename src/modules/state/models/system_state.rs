use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::logs::content::{LogEvent, LogEventContent};
use crate::logs::content::log_event_content::fss_signal_discovered_event::FSSSignalDiscoveredEvent;
use crate::modules::civilization::LocationInfo;
use crate::state::ExobiologyState;
use crate::state::models::body_state::BodyState;
use crate::state::models::feed_result::FeedResult;

#[derive(Serialize)]
pub struct SystemState {
    pub location_info: LocationInfo,
    pub bodies: HashMap<u8, BodyState>,
    pub visits: Vec<DateTime<Utc>>,
    pub carrier_visits: Vec<DateTime<Utc>>,
    pub number_of_bodies: Option<u8>,
    pub progress: f32,
    pub all_found: bool,
    pub station_signals: Vec<FSSSignalDiscoveredEvent>,
    pub exobiology: ExobiologyState,
}

impl SystemState {
    pub fn feed_log_event(&mut self, log_event: &LogEvent) -> FeedResult {
        let Some(system_address) = log_event.content.system_address() else {
            return FeedResult::Skipped;
        };

        if system_address != self.location_info.system_address {
            return FeedResult::Skipped;
        }

        self.exobiology.feed_event(log_event);

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
            LogEventContent::Scan(event) => {
                self.bodies
                    .entry(event.body_id)
                    .or_insert_with(|| BodyState::new(event.clone()));
            }

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

    pub fn clear_organic_progress(&mut self) {
        for body in self.bodies.values_mut() {
            body.clear_organic_process();
        }
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
            exobiology: ExobiologyState::new(),
        }
    }
}
