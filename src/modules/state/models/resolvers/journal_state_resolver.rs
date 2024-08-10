pub mod journal_commander_entry;

use std::collections::HashMap;

use serde::Serialize;

use crate::journal::{JournalEvent, JournalEventKind};
use crate::logs::LogEventContent;
use crate::state::models::feed_result::FeedResult;
use crate::state::models::resolvers::journal_state_resolver::journal_commander_entry::JournalCommanderEntry;
use crate::state::traits::state_resolver::StateResolver;
use crate::state::{LiveState, LogState};

/// State which tracks both log events and events that are fired when a json file updates.
#[derive(Serialize, Default)]
pub struct JournalStateResolver {
    pub commanders: HashMap<String, JournalCommanderEntry>,
    current_commander_id: Option<String>,
}

impl StateResolver<JournalEvent> for JournalStateResolver {
    fn feed(&mut self, input: &JournalEvent) -> FeedResult {
        if let JournalEventKind::LogEvent(log_event) = &input.kind {
            if let LogEventContent::Commander(commander) = &log_event.content {
                self.current_commander_id = Some(commander.fid.to_string());

                if !self.commanders.contains_key(&commander.fid) {
                    self.commanders
                        .insert(commander.fid.to_string(), JournalCommanderEntry::default());
                }
            }

            let Some(current_commander) = self.current_commander_mut() else {
                return FeedResult::Later;
            };

            current_commander.log_state.feed(log_event);
        }

        let Some(current_commander) = self.current_commander_mut() else {
            return FeedResult::Later;
        };

        current_commander.live_state.feed(input);

        FeedResult::Accepted
    }
}

impl From<HashMap<String, LiveState>> for JournalStateResolver {
    fn from(value: HashMap<String, LiveState>) -> Self {
        Self {
            commanders: value
                .into_iter()
                .map(|(key, state)| {
                    (
                        key,
                        JournalCommanderEntry {
                            log_state: LogState::default(),
                            live_state: state,
                        },
                    )
                })
                .collect(),
            current_commander_id: None,
        }
    }
}

impl JournalStateResolver {
    pub fn current_commander(&self) -> Option<&JournalCommanderEntry> {
        self.current_commander_id
            .as_ref()
            .and_then(|commander_id| self.commanders.get(commander_id))
    }

    pub fn current_commander_mut(&mut self) -> Option<&mut JournalCommanderEntry> {
        self.current_commander_id
            .as_ref()
            .and_then(|commander_id| self.commanders.get_mut(commander_id))
    }

    pub fn all_live_state(&self) -> HashMap<&String, &LiveState> {
        self.commanders
            .iter()
            .map(|(key, value)| (key, &value.live_state))
            .collect()
    }
}
