//! Fired when there is any update related to the carrier's finances.

use crate::station::CarrierFinance;
use serde::{Deserialize, Serialize};

/// Fired when there is any update related to the carrier's finances.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct CarrierFinanceEvent {
    /// The id of the carrier that the player deposited fuel to. This is functionally the same as
    /// the market id.
    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,

    /// Financial details for a given fleet carrier.
    #[serde(flatten)]
    pub finance: CarrierFinance,
}
