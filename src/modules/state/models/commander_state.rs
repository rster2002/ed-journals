use std::collections::HashMap;
use serde::Serialize;
use crate::logs::content::log_event_content::commander_event::CommanderEvent;
use crate::logs::content::{LogEvent, LogEventContent};
use crate::logs::content::log_event_content::scan_organic_event::ScanOrganicEventScanType;
use crate::modules::civilization::LocationInfo;
use crate::state::models::current_organic::CurrentOrganic;
use crate::state::models::feed_result::FeedResult;
use crate::state::SystemState;

#[derive(Serialize)]
pub struct CommanderState {
    pub fid: String,
    pub name: String,
    pub systems: HashMap<u64, SystemState>,
    pub current_system: Option<u64>,
    pub current_organic: Option<CurrentOrganic>,
}

impl CommanderState {
    pub fn feed_log_event(&mut self, log_event: &LogEvent) -> FeedResult {
        match &log_event.content {
            LogEventContent::Location(location) => {
                self.current_system = Some(location.location_info.system_address);

                let system = self.upset_system(&location.location_info);
                system.visit(&log_event.timestamp);
            },
            LogEventContent::CarrierJump(carrier_jump) => {
                let system = self.upset_system(&carrier_jump.system_info);
                system.carrier_visit(&log_event.timestamp);
            },
            LogEventContent::FSDJump(fsd_jump) => {
                self.current_system = Some(fsd_jump.system_info.system_address);

                let system = self.upset_system(&fsd_jump.system_info);
                system.visit(&log_event.timestamp);
            },
            LogEventContent::ScanOrganic(scan_organic) => {
                match &scan_organic.scan_type {
                    ScanOrganicEventScanType::Sample => {}
                    ScanOrganicEventScanType::Analyse => {}
                    ScanOrganicEventScanType::Log => {
                        self.current_organic = None;
                    }
                }
            },

            _ => {},
        }

        if let Some(address) = log_event.content.system_address() {
            let Some(system) = self.systems.get_mut(&address) else {
                return FeedResult::Later;
            };

            return system.feed_log_event(log_event);
        }

        FeedResult::Accepted
    }

    pub fn upset_system(&mut self, location_info: &LocationInfo) -> &mut SystemState {
        self.systems.entry(location_info.system_address).or_insert_with(|| location_info.into());

        self.systems.get_mut(&location_info.system_address)
            .expect("Should have been added")
    }

    pub fn current_system(&self) -> Option<&SystemState> {
        self.systems.get(&self.current_system?)
    }
}

impl From<&CommanderEvent> for CommanderState {
    fn from(value: &CommanderEvent) -> Self {
        CommanderState {
            fid: value.fid.to_string(),
            name: value.name.to_string(),
            systems: HashMap::new(),
            current_system: None,
            current_organic: None,
        }
    }
}
