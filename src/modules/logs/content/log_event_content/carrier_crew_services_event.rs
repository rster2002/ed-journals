use serde::{Deserialize, Serialize};

/// Fired when changes were made to the crew of a service.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CarrierCrewServicesEvent {
    /// The ID of the carrier of which the crew was changed.
    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,

    /// The service for which the crew has been changed.
    pub crew_role: CarrierCrewServicesEventCrewRole,

    /// The current operational status of the target service.
    pub operation: CarrierCrewServicesEventOperation,

    /// The name of the crew member for the [crew_role].
    pub crew_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CarrierCrewServicesEventCrewRole {
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CarrierCrewServicesEventOperation {
    Active,
    Activate,
    Deactivate,

    #[serde(rename = "Pause")]
    Suspended,
    Replace,
}
