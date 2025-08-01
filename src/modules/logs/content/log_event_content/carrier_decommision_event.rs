//! Fired when the player schedules their carrier for decommission.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Fired when the player schedules their carrier for decommission.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct CarrierDecommissionEvent {
    /// The id of the carrier that is being decommissioned. This is functionally the same as the
    /// market id.
    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,

    /// The amount of credits the player receives back after the carrier has been decommissioned.
    pub scrap_refund: u64,

    /// The timestamp after which the decommission is final.
    pub scrap_time: DateTime<Utc>,
}
