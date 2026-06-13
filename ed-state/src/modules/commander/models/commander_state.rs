use crate::modules::state::{EventSink, SinkResult};
use crate::modules::system::SystemState;
use ed_journals::logs::LogEvent;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct CommanderState {
    pub systems: HashMap<u64, SystemState>,
}

impl EventSink for CommanderState {
    fn sink_log(&mut self, log_event: &LogEvent) -> SinkResult {
        let mut result = SinkResult::default();

        if let Some(partial_system_info) = log_event.content.partial_system_info() {
            result.or_replace(
                self.systems
                    .entry(partial_system_info.system_address)
                    .or_insert_with(|| SystemState::from(partial_system_info))
                    .sink_log(log_event),
            );
        }

        result
    }
}
