use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CarrierFinanceEvent {
    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,
    pub carrier_balance: u64,
    pub reserve_balance: u64,
    pub available_balance: u64,
    pub reserve_percent: f32,

    #[serde(rename = "TaxRate_rearm")]
    pub tax_rate_rearm: f32,

    #[serde(rename = "TaxRate_refuel")]
    pub tax_rate_refuel: f32,

    #[serde(rename = "TaxRate_repair")]
    pub tax_rate_repair: f32,
}
