use std::collections::HashMap;
use std::mem;

use serde::Serialize;

use crate::logs::{LogEvent, LogEventContent};
use crate::logs::file_header_event::FileHeaderEvent;
use crate::state::LogState;
use crate::state::models::feed_result::FeedResult;
use crate::state::traits::state_resolver::StateResolver;

#[derive(Serialize, Default)]
pub struct GameStateResolver {
    pub commanders: HashMap<String, LogState>,
    current_commander_id: Option<String>,
}

impl StateResolver<LogEvent> for GameStateResolver {
    fn feed(&mut self, input: &LogEvent) -> FeedResult {
        match &input.content {
            LogEventContent::Commander(commander) => {
                self.current_commander_id = Some(commander.fid.to_string());

                if !self.commanders.contains_key(&commander.fid) {
                    self.commanders
                        .insert(commander.fid.to_string(), LogState::default());
                }
            }
            _ => {
                let Some(current) = self.current_commander_mut() else {
                    return FeedResult::Later;
                };

                current.feed(input);
            }
        }

        FeedResult::Accepted
    }

    fn flush_inner(&mut self) {
        for commander in self.commanders.values_mut() {
            commander.flush();
        }
    }
}

impl GameStateResolver {
    /// Returns the current commander which is active in the logs.
    pub fn current_commander(&self) -> Option<&LogState> {
        self.current_commander_id
            .as_ref()
            .and_then(|commander_id| self.commanders.get(commander_id))
    }

    /// Returns a mutable reference to the current active commander in the logs.
    pub fn current_commander_mut(&mut self) -> Option<&mut LogState> {
        self.current_commander_id
            .as_ref()
            .and_then(|commander_id| self.commanders.get_mut(commander_id))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::logs::blocking::LogDirReader;
    use crate::state::GameState;
    use std::env::current_dir;
    use std::time::Instant;
    use crate::state::traits::state_resolver::StateResolver;

    #[test]
    fn state_is_correct() {
        let dir_path = current_dir().unwrap().join("test-files").join("journals");

        let log_dir = LogDirReader::open(dir_path);

        let mut state = GameState::default();
        let instant = Instant::now();

        for entry in log_dir {
            state.feed(&entry.unwrap());
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
