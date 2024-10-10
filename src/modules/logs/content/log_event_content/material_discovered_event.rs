use serde::{Deserialize, Serialize};

use crate::modules::materials::{Material, MaterialCategory};

/// Fired when the player collects a specific material for the first time.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialDiscoveredEvent {
    /// The kind of material discovered.
    pub name: Material,

    /// The category the material belongs to.
    pub category: MaterialCategory,

    /// The number assigned to the discovery.
    pub discovery_number: u16,
}
