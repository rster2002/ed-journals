use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// Model for working with local (system) distances. Expects the value to be in LS.
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct LocalDistance(pub f32);

const LS_IN_M: f32 = 299792458.0;
const LS_IN_AU: f32 = 500.0;

impl LocalDistance {
    /// Creates a new distance from the given amount of meters.
    pub fn from_m(m: f32) -> Self {
        LocalDistance(m / LS_IN_M)
    }

    /// Returns the distance in meters.
    pub fn as_m(&self) -> f32 {
        self.0 * LS_IN_M
    }

    /// Creates a new distance from the given amount of light seconds.
    pub fn from_ls(ls: f32) -> Self {
        LocalDistance(ls)
    }

    /// Returns the distance in light seconds.
    pub fn as_ls(&self) -> f32 {
        self.0
    }

    /// Creates a new distance from the given amount of astronomical units.
    pub fn from_au(au: f32) -> Self {
        LocalDistance(au * LS_IN_AU)
    }

    /// Returns the distance in astronomical units.
    pub fn as_au(&self) -> f32 {
        self.0 / LS_IN_AU
    }
}

impl Debug for LocalDistance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ls ({} au / {} m)", self.0, self.as_au(), self.as_m())
    }
}
