use serde::{Deserialize, Serialize};
use crate::exobiology::{Species, Variant};

/// A confirmed location of a specific organic on a planet.
#[derive(Debug, Serialize, Deserialize)]
pub struct OrganicLocation {
    /// The system address of the body the organic has been scanned.
    pub system_address: u64,

    /// The body id of the body where the organic has been scanned.
    pub body_id: u8,

    /// The specific species that has been scanned.
    pub species: Species,

    /// Specific species variant, if any.
    pub variant: Option<Variant>,

    /// The coordinates of the *player* when the organic was scanned.
    pub coordinates: (f32, f32),
}
