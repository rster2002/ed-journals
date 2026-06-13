use crate::modules::state::{EventSink, SinkResult};
use ed_journals::civilization::LocationInfo;
use ed_journals::logs::LogEvent;
use ed_journals::partials::PartialSystemInfo;

#[derive(Debug, Clone)]
pub struct SystemState {
    /// The address of the system.
    pub partial_system_info: PartialSystemInfo,

    /// Information about the system.
    pub location_info: Option<LocationInfo>,
}

impl From<PartialSystemInfo> for SystemState {
    fn from(value: PartialSystemInfo) -> Self {
        SystemState {
            partial_system_info: value,
            location_info: None,
        }
    }
}

impl EventSink for SystemState {
    fn sink_log(&mut self, log_event: &LogEvent) -> SinkResult {
        if !log_event
            .content
            .system_address()
            .is_some_and(|address| address == self.partial_system_info.system_address)
        {
            return SinkResult::Ignored;
        }

        if let Some(location_info) = log_event.content.location_info() && self.location_info.is_none() {
            self.location_info = Some(location_info.clone());
        }

        todo!()
    }
}
