use serde::{Deserialize, Serialize};

/// Financial details for a given fleet carrier.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CarrierFinance {
    /// The current balance of the carrier in credits.
    pub carrier_balance: u64,

    /// The reserve balance of the carrier in credits.
    pub reserve_balance: u64,

    /// The total available balance of the carrier in credits.
    pub available_balance: u64,

    /// The percentage of the available balance that is reserved.
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
