use serde::Deserialize;

/// Fired when changes were made to the crew of a service.
#[derive(Debug, Deserialize, Clone, PartialEq)]
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

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum CarrierCrewServicesEventCrewRole {
    Refuel,
    Repair,
    Rearm,
    Shipyard,

    #[serde(rename = "Exploration")]
    UniversalCartographers,
    VistaGenomics,
    Bartender,
    Outfitting,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum CarrierCrewServicesEventOperation {
    Active,
    Activate,
    #[serde(rename = "Pause")]
    Suspended,
}
