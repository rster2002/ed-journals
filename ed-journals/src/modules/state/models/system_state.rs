use std::collections::{HashMap, VecDeque};
use std::fmt::{Debug, Formatter};
use chrono::{DateTime, NaiveDateTime, Utc};
use crate::logs::content::{LogEvent, LogEventContent};
use crate::logs::content::log_event_content::fss_signal_discovered_event::FSSSignalDiscoveredEvent;
use crate::shared::civilization::system_info::SystemInfo;
use crate::state::models::body_state::BodyState;
use crate::state::models::feed_result::FeedResult;

pub struct SystemState {
    pub system_info: SystemInfo,
    pub bodies: HashMap<u8, BodyState>,
    pub visits: Vec<DateTime<Utc>>,
    pub carrier_visits: Vec<DateTime<Utc>>,
    pub number_of_bodies: Option<u8>,
    pub progress: f32,
    pub all_found: bool,
    pub station_signals: Vec<FSSSignalDiscoveredEvent>,
}

impl SystemState {
    pub fn feed_log_event(&mut self, log_event: &LogEvent) -> FeedResult {
        let Some(system_address) = log_event.content.system_address() else {
            return FeedResult::Skipped;
        };

        if system_address != self.system_info.system_address {
            return FeedResult::Skipped;
        }

        match &log_event.content {
            LogEventContent::FSSDiscoveryScan(event) => {
                self.number_of_bodies = Some(event.body_count);
                self.progress = event.progress;
            },
            LogEventContent::FSSAllBodiesFound(event) => {
                self.number_of_bodies = Some(event.count);
                self.all_found = true;
            },
            LogEventContent::FSSSignalDiscovered(event) => {
                if event.is_station {
                    self.station_signals.push(event.clone());
                }
            },
            LogEventContent::Scan(event) => {
                if !self.bodies.contains_key(&event.body_id) {
                    self.bodies.insert(event.body_id, BodyState::new(event.clone()));
                }
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
        self.visits.push(date_time.clone());
    }

    pub fn carrier_visit(&mut self, date_time: &DateTime<Utc>) {
        self.carrier_visits.push(date_time.clone());
    }
}

impl From<&SystemInfo> for SystemState {
    fn from(value: &SystemInfo) -> Self {
        SystemState {
            system_info: value.clone(),
            bodies: HashMap::new(),
            visits: Vec::new(),
            carrier_visits: Vec::new(),
            number_of_bodies: None,
            progress: 0.0,
            all_found: false,
            station_signals: Vec::new()
        }
    }
}

impl Debug for SystemState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "{")?;

        write!(f, "system_info: {:?}, ", self.system_info)?;
        write!(f, "visits: {:?}, ", self.visits)?;
        write!(f, "carrier_visits: {:?}, ", self.carrier_visits)?;
        write!(f, "number_of_bodies: {:?}, ", self.number_of_bodies)?;
        write!(f, "progress: {:?}, ", self.progress)?;
        write!(f, "all_found: {:?}, ", self.all_found)?;

        write!(f, "bodies({}): [", self.bodies.len())?;

        for body in self.bodies.values() {
            write!(f, "{:?}, ", body)?;
        }

        write!(f, "]")?;

        write!(f, "{}", "}")?;

        Ok(())
    }
}
