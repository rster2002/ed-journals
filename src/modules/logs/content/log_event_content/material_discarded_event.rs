use serde::{Deserialize, Serialize};

use crate::modules::materials::{Material, MaterialCategory};

/// Fired when the player discards a material.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialDiscarded {
    /// The kind of material discarded.
    pub name: Material,

    /// The category the material belongs to.
    pub category: MaterialCategory,

    /// The amount of the given material discarded.
    pub count: u16,
}
