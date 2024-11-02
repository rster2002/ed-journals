//! Fired when the player requested docking permission, but was denied.

use serde::{Deserialize, Serialize};

use crate::modules::station::StationType;

/// Fired when the player requested docking permission, but was denied.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DockingDeniedEvent {
    /// The name of the station or settlement the player was denied docking for.
    pub station_name: String,

    /// The kind of station the player wanted to dock at.
    pub station_type: StationType,

    /// The market id for the station.
    #[serde(rename = "MarketID")]
    pub market_id: u64,

    /// The reason why the player was denied docking access.
    pub reason: DockingDeniedReason,
}

/// The reason why the player was denied docking access.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum DockingDeniedReason {
    /// There are currently no landing pads available for the player to land on.
    NoSpace,

    /// There are (currently) no landing pads available for the player's ship size.
    TooLarge,

    /// The player is currently considered hostile to the station.
    Hostile,

    /// The player has outstanding offences (like recently acquired fines)
    Offences,

    /// The player is not within 7.5 km of the station.
    Distance,

    /// The player has an active fighter out.
    ActiveFighter,

    /// The player does not have docking permissions for the given fleet carrier.
    RestrictedAccess,

    /// The reason was not specified.
    NoReason,

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    Unknown(String),
}
