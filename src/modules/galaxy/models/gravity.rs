use std::fmt::Debug;
use serde::{Deserialize, Serialize};

/// Gravity in m/s².
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Gravity(pub f32);

const G_CONSTANT: f32 = 9.812;

impl Gravity {
    pub fn from_g(g: f32) -> Gravity {
        Gravity(g * G_CONSTANT)
    }

    /// Returns the value of gravity in G.
    pub fn as_g(&self) -> f32 {
        // Round on two decimal points
        (self.0 / G_CONSTANT * 100.0).round() / 100.0
    }

    /// Returns the value of gravity in m/s².
    pub fn as_ms2(&self) -> f32 {
        self.0
    }

    pub fn is_low(&self) -> bool {
        self.as_g() <= 0.5
    }

    pub fn is_high(&self) -> bool {
        self.as_g() >= 2.0
    }
}

impl Debug for Gravity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}m/s² ({}g)", self.0, self.as_g())
    }
}
