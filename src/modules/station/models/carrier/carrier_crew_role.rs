use serde::{Deserialize, Serialize};

/// The given service the event is for.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CarrierCrewRole {
    Captain,
    CarrierFuel,
    Refuel,
    Repair,
    Rearm,
    Shipyard,

    #[serde(rename = "Commodities")]
    Market,

    BlackMarket,

    #[serde(rename = "Exploration")]
    UniversalCartographers,
    VistaGenomics,
    PioneerSupplies,
    Bartender,
    Outfitting,

    #[serde(rename = "VoucherRedemption")]
    RedemptionOffice,
}
