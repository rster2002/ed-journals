use std::cmp::Ordering;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GalacticDistance(f32);

impl GalacticDistance {
    pub fn from_ly(ly: f32) -> Self {
        GalacticDistance(ly)
    }

    pub fn as_ly(&self) -> f32 {
        self.0
    }

    pub fn between(a: [f32; 3], b: [f32; 3]) -> Self {
        let x = a[0] - b[0];
        let y = a[1] - b[1];
        let z = a[2] - b[2];

        GalacticDistance((x * x + y * y + z * z).sqrt())
    }
}

impl PartialOrd for GalacticDistance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
