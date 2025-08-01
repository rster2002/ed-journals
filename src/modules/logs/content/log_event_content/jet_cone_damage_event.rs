//! Fired when a module receives damage from a jet cone.

use serde::{Deserialize, Serialize};

use crate::modules::ship::ShipModule;

/// Fired when a module receives damage from a jet cone.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct JetConeDamageEvent {
    /// The kind of module that was damaged.
    pub module: ShipModule,
}
