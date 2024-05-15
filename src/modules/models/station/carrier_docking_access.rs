use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CarrierDockingAccess {
    #[serde(rename = "all")]
    All,

    #[serde(rename = "none")]
    None,

    #[serde(rename = "squadron")]
    Squadron,

    #[serde(rename = "squadronfriends")]
    SquadronAndFriends,
}
