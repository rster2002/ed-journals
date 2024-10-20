use serde::{Deserialize, Serialize};

/// Fired when the game changes the music track.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct MusicEvent {
    /// The name of the track that is now playing.
    pub music_track: String,
}
