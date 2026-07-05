//! Fired when opening the fleet carrier management panel.

use serde::{Deserialize, Serialize};

use crate::modules::station::CarrierDockingAccess;
use crate::station::{CarrierCrewRole, CarrierFinance};

/// Fired when opening the fleet carrier management panel.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CarrierStatsEvent {
    /// The id of the carrier that the player deposited fuel to. This is functionally the same as
    /// the market id.
    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,

    /// The callsign of the carrier.
    pub callsign: String,

    /// The name of the carrier.
    pub name: String,

    /// The current docking access.
    pub docking_access: CarrierDockingAccess,

    /// The current notorious docking option.
    pub allow_notorious: bool,

    /// The current fuel level of the carrier. This is a number between 0 and 1000 and does not
    /// include any tritium that might be stored in the cargo hold of the carrier.
    pub fuel_level: u16,

    /// The current jump range for the carrier.
    pub jump_range_curr: f32,

    /// The max jump range for the carrier. This is always 500.
    pub jump_range_max: f32,

    /// Whether the carrier is pending decommission.
    pub pending_decommission: bool,

    /// Details about how much of the internal cargo space is being used.
    pub space_usage: CarrierStatsEventSpaceUsage,

    /// Details about the current finances of the fleet carrier.
    pub finance: CarrierFinance,

    /// List of services and their status for the fleet carrier.
    pub crew: Vec<CarrierStatsEventCrewEntry>,

    /// List of loaded ship packs and their status.
    pub ship_packs: Vec<CarrierStatsEventPack>,

    /// List of loaded module packs and their status.
    pub module_packs: Vec<CarrierStatsEventPack>,
}

/// Details about how much of the internal cargo space is being used.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CarrierStatsEventSpaceUsage {
    /// The total available capacity for the fleet carrier. This is always 25000.
    pub total_capacity: u16,

    /// How much of the available capacity is being used by services.
    pub crew: u16,

    /// How much of the available capacity is being used by stored cargo.
    pub cargo: u16,

    /// How much of the available capacity is reserved for open buy orders.
    pub cargo_space_reserved: u16,

    /// How much of the available capacity is being used for storing ship packs.
    pub ship_packs: u16,

    /// How much of the available capacity is being used for storing module packs.
    pub module_packs: u16,

    /// How much free space there is left.
    pub free_space: u16,
}

/// An entry for a given fleet carrier service.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CarrierStatsEventCrewEntry {
    /// The service this entry is for.
    pub crew_role: CarrierCrewRole,

    /// Whether the given service is installed on the carrier.
    pub activated: bool,

    /// Whether the given service is currently enabled or is suspended.
    #[serde(default)]
    pub enabled: bool,

    /// The name of the crew member shown in the services tab.
    pub crew_name: Option<String>,
}

/// A pack that is currently stored on the carrier.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CarrierStatsEventPack {
    /// The name of the pack theme.
    pub pack_theme: String,

    /// The tier of the pack.
    pub pack_tier: u8,
}
