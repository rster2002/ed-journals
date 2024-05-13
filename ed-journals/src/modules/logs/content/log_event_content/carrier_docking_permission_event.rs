use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CarrierDockingPermissionEvent {
    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,
    pub docking_access: CarrierDockingPermissionEventDockingAccess,
    pub allow_notorious: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CarrierDockingPermissionEventDockingAccess {
    #[serde(rename = "all")]
    All,

    #[serde(rename = "none")]
    None,

    #[serde(rename = "squadron")]
    Squadron,

    #[serde(rename = "squadronfriends")]
    SquadronAndFriends,
}
