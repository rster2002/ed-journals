use crate::commander::models::materials_state::MaterialsState;
use crate::modules::state::{EventSink, SinkResult};
use crate::modules::system::SystemState;
use ed_journals::logs::commander_event::CommanderEvent;
use ed_journals::logs::touchdown_event::TouchdownEvent;
use ed_journals::logs::{LogEvent, LogEventContent};
use ed_journals::status::{Status, StatusContents};
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct CommanderState {
    pub commander: Option<CommanderEvent>,

    /// Systems that the player has visited.
    pub systems: HashMap<u64, SystemState>,

    /// The system address the player is currently in.
    pub current_system: Option<u64>,

    /// Current touchdown event. Is set back to none when the player lifts off or dies.
    pub touchdown: Option<TouchdownEvent>,

    /// Keeps track of the current materials the player has.
    pub material_state: MaterialsState,

    pub status: Option<StatusContents>,
}

impl CommanderState {
    pub fn current_system_state(&self) -> Option<&SystemState> {
        self.current_system
            .as_ref()
            .and_then(|id| self.systems.get(id))
    }
}

impl EventSink for CommanderState {
    fn sink_log(&mut self, log_event: &LogEvent) -> SinkResult {
        let mut result = SinkResult::default();

        if let Some(system_address) = log_event.content.system_address() {
            result.or_replace(
                self.systems
                    .entry(system_address)
                    .or_insert_with(|| SystemState::from(system_address))
                    .sink_log(log_event),
            );
        }

        result.or_replace(self.material_state.sink_log(log_event));

        match &log_event.content {
            LogEventContent::Commander(event) => {
                self.commander = Some(event.clone());
                result.accept();
            }
            LogEventContent::Location(location) => {
                self.current_system = Some(location.location_info.system_address);
                result.accept();
            }
            LogEventContent::FSDJump(fsd_jump) => {
                self.current_system = Some(fsd_jump.system_info.system_address);
                result.accept();
            }
            LogEventContent::Touchdown(event) => {
                self.touchdown = Some(event.clone());
                result.accept();
            }
            LogEventContent::Liftoff(_) => {
                self.touchdown = None;
                result.accept();
            }
            _ => {}
        }

        result
    }

    fn sink_status(&mut self, status: &Status) -> SinkResult {
        self.status = status.contents.clone();
        SinkResult::Accepted
    }
}
