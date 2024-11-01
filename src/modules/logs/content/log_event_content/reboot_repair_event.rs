use serde::{Deserialize, Serialize};

use crate::modules::ship::ShipSlot;

/// Fired when performing a reboot and repair.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct RebootRepairEvent {
    /// List of slots of the modules that were affected.
    pub modules: Vec<ShipSlot>,
}
