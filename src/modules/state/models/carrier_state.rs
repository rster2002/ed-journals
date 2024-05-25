use chrono::{DateTime, Utc};
use serde::Serialize;
use crate::logs::content::log_event_content::carrier_stats_event::CarrierStatsEvent;
use crate::logs::content::{LogEvent, LogEventContent};
use crate::small::SmallSystemInfo;
use crate::state::models::feed_result::FeedResult;

#[derive(Serialize)]
pub struct CarrierState {
    pub stats: CarrierStatsEvent,
    pub getting_decommissioned: bool,
    pub flight_history: Vec<SmallSystemInfo>,
    pub last_location_update: DateTime<Utc>,
}

impl CarrierState {
    pub fn feed_log_event(&mut self, log_event: &LogEvent) -> FeedResult {
        match &log_event.content {
            LogEventContent::CarrierStats(stats) => {
                self.stats = stats.clone();
            },
            LogEventContent::CarrierDecommission(_) => {
                self.getting_decommissioned = true;
            },
            LogEventContent::CarrierCancelDecommission(_) => {
                self.getting_decommissioned = false;
            },
            _ => {},
        }

        if log_event.timestamp > self.last_location_update {
            if let Some(small_info) = log_event.content.small_system_info() {
                let Some(last) = self.flight_history.last() else {
                    return FeedResult::Accepted;
                };

                if last.system_address != small_info.system_address {
                    self.flight_history.push(small_info);
                    self.last_location_update = log_event.timestamp;
                }
            }
        }

        FeedResult::Accepted
    }
}

impl From<CarrierStatsEvent> for CarrierState {
    fn from(value: CarrierStatsEvent) -> Self {
        CarrierState {
            stats: value,
            getting_decommissioned: false,
            flight_history: Vec::new(),
            last_location_update: Default::default(),
        }
    }
}
