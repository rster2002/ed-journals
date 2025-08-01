use serde::{Deserialize, Serialize};

/// Docking access for a fleet carrier. This, along with the 'allow notorious' option, determines
/// who can and who cannot dock at the given fleet carrier.
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CarrierDockingAccess {
    /// All commanders can dock.
    #[serde(rename = "all")]
    All,

    /// Commanders that are part of the same squadron that the owner is part of can dock.
    #[serde(rename = "squadron")]
    Squadron,

    /// Commanders that are friends with the owner and commanders that are part of the same squadron
    /// as the owner are allowed to dock.
    #[serde(rename = "squadronfriends")]
    SquadronAndFriends,

    /// No-one other than the owner of the fleer carrier can dock.
    #[serde(rename = "none")]
    None,
}
