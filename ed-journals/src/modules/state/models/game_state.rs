use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::mem;
use serde::Serialize;
use crate::journal::JournalEvent;
use crate::logs::content::log_event_content::file_header_event::FileHeaderEvent;
use crate::logs::content::{LogEvent, LogEventContent};
use crate::state::models::commander_state::CommanderState;
use crate::state::models::feed_result::FeedResult;

#[derive(Serialize)]
pub struct GameState {
    commanders: HashMap<String, CommanderState>,
    current_commander: Option<String>,
    file_header: Option<FileHeaderEvent>,
    header_count: u64,
    pub queued: Vec<LogEvent>,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            commanders: HashMap::new(),
            current_commander: None,
            file_header: None,
            header_count: 0,
            queued: Vec::new(),
        }
    }

    pub fn current_commander(&self) -> Option<&CommanderState> {
        let Some(commander_id) = &self.current_commander else {
            return None;
        };

        let Some(commander_entry) = self.commanders.get(commander_id) else {
            return None;
        };

        Some(commander_entry)
    }

    pub fn current_commander_mut(&mut self) -> Option<&mut CommanderState> {
        let Some(commander_id) = &self.current_commander else {
            return None;
        };

        let Some(commander_entry) = self.commanders.get_mut(commander_id) else {
            return None;
        };

        Some(commander_entry)
    }

    pub fn feed_log_event(&mut self, event: &LogEvent) {
        let handle_result = self.handle(event);

        if let FeedResult::Later = handle_result {
            self.queued.push(event.clone());
        }

        let queued = mem::take(&mut self.queued);

        for item in queued {
            let result = self.handle(&item);

            if !result.is_accepted() {
                self.queued.push(item.clone());
            }
        }

        // TODO make this actually decent
        // self.queued = self.queued
        //     .iter()
        //     .filter(|item| {
        //         !self.handle(item).is_accepted()
        //     })
        //     .map(|item| item.clone())
        //     .collect()
    }

    fn handle(&mut self, event: &LogEvent) -> FeedResult {
        match &event.content {
            LogEventContent::FileHeader(header) => {
                self.file_header = Some(header.clone());
                self.header_count += 1;
            },
            LogEventContent::Commander(commander) => {
                self.current_commander = Some(commander.fid.to_string());

                if !self.commanders.contains_key(&commander.fid) {
                    self.commanders.insert(commander.fid.to_string(), commander.into());
                }
            },
            _ => {
                let Some(current) = self.current_commander_mut() else {
                    return FeedResult::Later;
                };

                match current.feed_log_event(event) {
                    FeedResult::Later => return FeedResult::Later,
                    _ => {}
                }
            }
        }

        FeedResult::Accepted
    }
}

impl Default for GameState {
    fn default() -> Self {
        GameState::new()
    }
}
