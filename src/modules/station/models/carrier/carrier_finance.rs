use serde::{Deserialize, Serialize};

/// Financial details for a given fleet carrier.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct CarrierFinance {
    /// The current balance of the carrier in credits. Note that this can be negative if the carrier
    /// is in debt.
    pub carrier_balance: i64,

    /// The reserve balance of the carrier in credits reserved for weekly upkeep.
    pub reserve_balance: u64,

    /// The total available balance of the carrier in credits. This balance may be used for active
    /// buy orders. Note that this can be negative if the carrier is in debt.
    pub available_balance: i64,

    /// The percentage of the available balance that is reserved for weekly upkeep.
    #[serde(default)]
    pub reserve_percent: f32,

    /// The tariff rate for re-arming at the carrier.
    #[serde(default, rename = "TaxRate_rearm")]
    pub tax_rate_rearm: f32,

    /// The tariff rate for refueling at the carrier.
    #[serde(default, rename = "TaxRate_refuel")]
    pub tax_rate_refuel: u64,

    /// The tariff rate for repairing at the carrier.
    #[serde(default, rename = "TaxRate_repair")]
    pub tax_rate_repair: u64,
}
