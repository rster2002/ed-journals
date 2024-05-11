use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CarrierStatsEvent {
    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,
    pub callsign: String,
    pub name: String,

    pub docking_access: CarrierStatsEventDockingAccess,
    pub allow_notorious: bool,

    // Between 0 and 1000
    pub fuel_level: u16,
    pub jump_range_curr: f32,
    pub jump_range_max: f32,
    pub pending_decommission: bool,
    pub space_usage: CarrierStatsEventSpaceUsage,
    pub finance: CarrierStatsEventFinance,
    pub crew: Vec<CarrierStatsEventCrewEntry>,
    pub ship_packs: Vec<CarrierStatsEventPack>,
    pub module_packs: Vec<CarrierStatsEventPack>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum CarrierStatsEventDockingAccess {
    #[serde(rename = "all")]
    All,

    #[serde(rename = "none")]
    None,

    #[serde(rename = "squadron")]
    Squadron,

    #[serde(rename = "squadronfriends")]
    SquadronAndFriends,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CarrierStatsEventSpaceUsage {
    pub total_capacity: u16,
    pub crew: u16,
    pub cargo: u16,
    pub cargo_space_reserved: u16,
    pub ship_packs: u16,
    pub module_packs: u16,
    pub free_space: u16,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CarrierStatsEventFinance {
    pub carrier_balance: u64,
    pub reserve_balance: u64,
    pub available_balance: u64,

    #[serde(default)]
    pub reserve_percent: f32,

    #[serde(default, rename = "TaxRate_rearm")]
    pub tax_rate_rearm: f32,

    #[serde(default, rename = "TaxRate_refuel")]
    pub tax_rate_refuel: u64,

    #[serde(default, rename = "TaxRate_repair")]
    pub tax_rate_repair: u64,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CarrierStatsEventCrewEntry {
    #[serde(default)]
    pub crew_role: CarrierStatsEventCrewRole,
    pub activated: bool,

    #[serde(default)]
    pub enabled: bool,
    pub crew_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
#[derive(Default)]
pub enum CarrierStatsEventCrewRole {
    BlackMarket,
    Captain,
    Refuel,
    Repair,
    Rearm,
    Commodities,
    VoucherRedemption,
    Exploration,
    Shipyard,
    Outfitting,
    CarrierFuel,
    VistaGenomics,
    PioneerSupplies,
    Bartender,
    #[default]
    Unknown,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CarrierStatsEventPack {
    pub pack_theme: String,
    pub pack_tier: u8,
}
