//! Fired when the player collects an engineering material.

use crate::materials::{Material, MaterialCategory};
use serde::{Deserialize, Serialize};

/// Fired when the player collects an engineering material.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialCollectedEvent {
    /// The kind of material collected.
    pub name: Material,

    /// The category the material belongs to.
    pub category: MaterialCategory,

    /// The amount of the given material collected.
    pub count: u16,
}
