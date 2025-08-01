//! Fired whenever the player cracks open an asteroid.

use serde::{Deserialize, Serialize};

/// Fired whenever the player cracks open an asteroid.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct AsteroidCrackedEvent {
    /// The body of the ring where the player cracked the asteroid.
    pub body: String,
}
