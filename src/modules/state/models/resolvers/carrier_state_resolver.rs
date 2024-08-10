use crate::logs::carrier_jump_request_event::CarrierJumpRequestEvent;
use crate::logs::carrier_stats_event::CarrierStatsEvent;
use crate::logs::{LogEvent, LogEventContent};
use crate::partials::PartialSystemInfo;
use crate::state::models::feed_result::FeedResult;
use crate::state::traits::state_resolver::StateResolver;
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct CarrierStateResolver {
    pub stats: CarrierStatsEvent,
    pub flight_history: Vec<PartialSystemInfo>,
    pub last_location_update: DateTime<Utc>,
    pub scheduled_jump: Option<CarrierJumpRequestEvent>,
    pub scrap_time: Option<DateTime<Utc>>,
}

impl StateResolver<LogEvent> for CarrierStateResolver {
    fn feed(&mut self, input: &LogEvent) -> FeedResult {
        match &input.content {
            LogEventContent::CarrierStats(stats) => {
                self.stats = stats.clone();
            }
            LogEventContent::CarrierDecommission(decommission) => {
                self.scrap_time = Some(decommission.scrap_time);
            }
            LogEventContent::CarrierCancelDecommission(_) => {
                self.scrap_time = None;
            }
            LogEventContent::CarrierJumpRequest(request) => {
                self.scheduled_jump = Some(request.clone());
            }
            LogEventContent::CarrierBuy(_) => {
                self.update_location(input);
            }
            LogEventContent::CarrierJump(_) => {
                self.update_location(input);
            }
            _ => {}
        }

        FeedResult::Accepted
    }
}

impl From<CarrierStatsEvent> for CarrierStateResolver {
    fn from(value: CarrierStatsEvent) -> Self {
        CarrierStateResolver {
            stats: value,
            flight_history: Vec::new(),
            last_location_update: Default::default(),
            scheduled_jump: None,
            scrap_time: None,
        }
    }
}

impl CarrierStateResolver {
    /// Returns the scheduled jump. This is a bit more reliable than just checking if
    /// `scheduled_jump` is Some value as this also checks the departure time. There could be
    /// instances where the scheduled jump might not be unset for example when not logged in.
    pub fn get_scheduled_jump(
        &self,
        target_time: &DateTime<Utc>,
    ) -> Option<CarrierJumpRequestEvent> {
        let Some(request) = &self.scheduled_jump else {
            return None;
        };

        if &request.departure_time <= target_time {
            return None;
        }

        Some(request.clone())
    }

    /// Whether the carrier has been marked for decommission and the scrap time has been reached.
    /// If this returns true, then the carrier should be considered decommissioned and not available
    /// anymore.
    pub fn has_been_scrapped(&self, target_time: &DateTime<Utc>) -> bool {
        self.scrap_time
            .as_ref()
            .is_some_and(|scrap_time| scrap_time >= target_time)
    }

    fn update_location(&mut self, log_event: &LogEvent) {
        if log_event.timestamp > self.last_location_update {
            if let Some(small_info) = log_event.content.small_system_info() {
                let Some(last) = self.flight_history.last() else {
                    return;
                };

                if last.system_address != small_info.system_address {
                    self.flight_history.push(small_info);
                    self.last_location_update = log_event.timestamp;
                }
            }
        }
    }
}
