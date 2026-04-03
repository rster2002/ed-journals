use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// Model for working with local (system) distances. Expects the value to be in LS.
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct LocalDistance(pub f32);

impl LocalDistance {
    /// Returns the distance in light seconds.
    pub fn as_ls(&self) -> f32 {
        self.0
    }

    /// Returns the distance in astronomical units.
    pub fn as_au(&self) -> f32 {
        self.0 / 500.0
    }

    /// Returns the distance in light years.
    pub fn as_ly(&self) -> f32 {
        self.0 / 31555695.8031
    }

    /// Creates a new distance from the given amount of light years.
    pub fn from_ly(ly: f32) -> Self {
        LocalDistance(ly * 31555695.8031)
    }
}

impl Debug for LocalDistance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ls ({} au / {} ly)",
            self.0,
            self.as_au(),
            self.as_ly()
        )
    }
}
