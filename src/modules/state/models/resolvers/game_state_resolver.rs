//! High level state resolver which encapsulates [LogState] and in turn
//! [LogStateResolver](super::log_state_resolver::LogStateResolver). Like the LogStateResolver, this
//! resolver handles all log events, but keep track of which commander the logs belong to.

pub mod game_commander_entry;

use std::collections::HashMap;

use serde::Serialize;

use crate::logs::{LogEvent, LogEventContent};
use crate::state::models::feed_result::FeedResult;
use crate::state::traits::state_resolver::StateResolver;
use crate::state::LogState;
use crate::state::resolvers::game_state_resolver::game_commander_entry::GameCommanderEntry;

/// High level state resolver which encapsulates [LogState] and in turn
/// [LogStateResolver](super::log_state_resolver::LogStateResolver). Like the LogStateResolver, this
/// resolver handles all log events, but keep track of which commander the logs belong to.
#[derive(Serialize, Default)]
pub struct GameStateResolver {
    /// A map of commanders that are tracked, where the key is the Frontier ID of the commander.
    pub commanders: HashMap<String, GameCommanderEntry>,
    current_commander_id: Option<String>,
}

impl StateResolver<LogEvent> for GameStateResolver {
    fn feed(&mut self, input: &LogEvent) -> FeedResult {
        match &input.content {
            LogEventContent::Commander(commander) => {
                self.current_commander_id = Some(commander.fid.to_string());

                if !self.commanders.contains_key(&commander.fid) {
                    self.commanders
                        .insert(commander.fid.to_string(), GameCommanderEntry {
                            name: commander.name.to_string(),
                            log_state: LogState::default(),
                        });
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
            commander.log_state.flush();
        }
    }
}

impl GameStateResolver {
    /// Returns the current commander which is active in the logs.
    pub fn current_commander(&self) -> Option<&LogState> {
        self.current_commander_id
            .as_ref()
            .and_then(|commander_id| self.commanders.get(commander_id))
            .map(|entry| &entry.log_state)
    }

    /// Returns a mutable reference to the current active commander in the logs.
    pub fn current_commander_mut(&mut self) -> Option<&mut LogState> {
        self.current_commander_id
            .as_ref()
            .and_then(|commander_id| self.commanders.get_mut(commander_id))
            .map(|entry| &mut entry.log_state)
    }
}

#[cfg(test)]
mod tests {
    use crate::logs::blocking::LogDirReader;
    use crate::state::traits::state_resolver::StateResolver;
    use crate::state::GameState;
    use std::collections::HashSet;
    use std::env::current_dir;
    use std::time::Instant;

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
            for system in commander.log_state.systems.values() {
                for body in system.planet_state.values() {
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
