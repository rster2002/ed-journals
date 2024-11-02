//! Always the first event that is fired for a log file.

use serde::{Deserialize, Serialize};
use crate::logs::continued_event::ContinuedEvent;

/// Always the first event that is fired for a log file. In the case where a log file is
/// [Continued](ContinuedEvent), the continued log file gets its own file header with a new part
/// number.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FileHeaderEvent {
    /// The part number for the log file. When just starting the game this will always be 1 and
    /// this is incremented with each continued log.
    pub part: u8,

    /// The language the player has configured.
    pub language: String,

    /// Whether the player has Odyssey enabled.
    #[serde(default, rename = "Odyssey")]
    pub odyssey: bool,

    /// The version of the game.
    #[serde(rename = "gameversion")]
    pub game_version: String,

    /// The internal build number for the version of the game.
    pub build: String,
}
