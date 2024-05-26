use std::collections::HashMap;
use std::mem;

use serde::Serialize;

use crate::logs::{LogEvent, LogEventContent};
use crate::logs::file_header_event::FileHeaderEvent;
use crate::state::models::commander_state::CommanderState;
use crate::state::models::feed_result::FeedResult;

/// The complete state of the whole game. This includes potentially the different commanders that
/// use the same game installation. By feeding the state entries from the journal log files it
/// creates a state which makes it easier to read information about the game.
#[derive(Serialize)]
pub struct GameState {
    pub commanders: HashMap<String, CommanderState>,
    current_commander: Option<String>,
    file_header: Option<FileHeaderEvent>,
    header_count: u64,
    later: Vec<LogEvent>,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            commanders: HashMap::new(),
            current_commander: None,
            file_header: None,
            header_count: 0,
            later: Vec::new(),
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

    /// Takes the log events and processes it in the state. Note that it does not guarantee that the
    /// event will be processed immediately. In some situations the event will be queued when the
    /// state things it is better able to process the event, but it doesn't do this automatically.
    /// For those events to be processed, you need to call [GameState::flush]. This will go through
    /// the remaining events and tries to process them.
    pub fn feed_log_event(&mut self, event: &LogEvent) {
        let handle_result = self.handle(event);

        if let FeedResult::Later = handle_result {
            self.later.push(event.clone());
        }
    }

    /// Processes any left-over events that were scheduled for later processing. Call this sparingly
    /// especially not while you're also still reading a lot of events through
    /// [GameState::feed_log_event] as that will likely cause performance issues.
    pub fn flush(&mut self) {
        let queued = mem::take(&mut self.later);

        for item in queued {
            let _ = self.handle(&item);
        }
    }

    fn handle(&mut self, event: &LogEvent) -> FeedResult {
        match &event.content {
            LogEventContent::FileHeader(header) => {
                self.file_header = Some(header.clone());
                self.header_count += 1;
            }
            LogEventContent::Commander(commander) => {
                self.current_commander = Some(commander.fid.to_string());

                if !self.commanders.contains_key(&commander.fid) {
                    self.commanders
                        .insert(commander.fid.to_string(), commander.into());
                }
            }
            _ => {
                let Some(current) = self.current_commander_mut() else {
                    return FeedResult::Later;
                };

                if let FeedResult::Later = current.feed_log_event(event) {
                    return FeedResult::Later;
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

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::logs::blocking::LogDirReader;
    use crate::state::GameState;
    use std::env::current_dir;
    use std::time::Instant;

    #[test]
    fn state_is_correct() {
        let dir_path = current_dir().unwrap().join("test-files").join("journals");

        let log_dir = LogDirReader::open(dir_path);

        let mut state = GameState::new();
        let instant = Instant::now();

        for entry in log_dir {
            state.feed_log_event(&entry.unwrap());
        }

        state.flush();

        dbg!(instant.elapsed().as_nanos());

        // Confirms that there are only one species of each genus on each planet
        for commander in state.commanders.values() {
            for system in commander.systems.values() {
                for body in system.bodies.values() {
                    let mut genuses = HashSet::new();

                    for species in &body.scanned_species {
                        let inserted = genuses.insert(species.genus());

                        if !inserted {
                            panic!("Not here!");
                        }
                    }
                }
            }
        }
    }
}
