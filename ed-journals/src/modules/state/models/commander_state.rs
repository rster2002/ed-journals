use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use chrono::NaiveDateTime;
use serde::Serialize;
use crate::logs::content::log_event_content::commander_event::CommanderEvent;
use crate::logs::content::{LogEvent, LogEventContent};
use crate::shared::civilization::system_info::SystemInfo;
use crate::state::models::feed_result::FeedResult;
use crate::state::SystemState;

pub struct CommanderState {
    pub fid: String,
    pub name: String,
    pub systems: HashMap<u64, SystemState>,
    pub current_system: Option<u64>,
}

impl CommanderState {
    pub fn feed_log_event(&mut self, log_event: &LogEvent) -> FeedResult {
        match &log_event.content {
            LogEventContent::Location(location) => {
                self.current_system = Some(location.system_info.system_address);

                let system = self.upset_system(&location.system_info);
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

            _ => {
                if let Some(address) = log_event.content.system_address() {
                    let Some(system) = self.systems.get_mut(&address) else {
                        return FeedResult::Later;
                    };

                    return system.feed_log_event(log_event);
                }
            },
        }

        FeedResult::Accepted
    }

    pub fn upset_system(&mut self, system_info: &SystemInfo) -> &mut SystemState {
        if !self.systems.contains_key(&system_info.system_address) {
            self.systems.insert(system_info.system_address, system_info.into());
        }

        self.systems.get_mut(&system_info.system_address)
            .expect("Should have been added")
    }

    pub fn current_system(&self) -> Option<&SystemState> {
        self.systems.get(&self.current_system?)
    }
}


impl Debug for CommanderState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "{")?;

        write!(f, "fid: {}, ", self.fid)?;
        write!(f, "name: {}, ", self.name)?;

        write!(f, "systems({}): [", self.systems.len())?;

        for system in self.systems.values() {
            write!(f, "{:?}, ", system)?;
        }

        write!(f, "]")?;

        write!(f, "{}", "}")?;

        Ok(())
    }
}

impl From<&CommanderEvent> for CommanderState {
    fn from(value: &CommanderEvent) -> Self {
        CommanderState {
            fid: value.fid.to_string(),
            name: value.name.to_string(),
            systems: HashMap::new(),
            current_system: None,
        }
    }
}
