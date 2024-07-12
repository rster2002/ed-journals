use serde::{Deserialize, Serialize};

use crate::modules::ship::ShipModule;

/// Fired whenever the player uses an AFMU to repair another module in their ship.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct AFMURepairsEvent {
    /// The module that is being repaired by the AFMU.
    pub module: ShipModule,

    /// The localized name of the module.
    #[serde(rename = "Module_Localised")]
    pub module_localized: Option<String>,

    /// Whether the target module has been fully repaired.
    pub fully_repaired: bool,

    /// The current health of the module that is being repaired.
    pub health: f32,
}
