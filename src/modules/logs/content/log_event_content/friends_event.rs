use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct FriendsEvent {
    status: FriendsEventStatus,
    name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum FriendsEventStatus {
    Offline,
    Online,
    Lost,
    Requested,
    Added,
    Declined,
}
