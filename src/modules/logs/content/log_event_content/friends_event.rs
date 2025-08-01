use serde::{Deserialize, Serialize};

/// Fired for events related to friends.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct FriendsEvent {
    /// The name of the friend.
    name: String,

    /// The status of the event related to the friend.
    status: FriendsEventStatus,
}

/// The status of the event related to the friend.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub enum FriendsEventStatus {
    /// The friend has gone offline.
    Offline,

    /// The friend has gone online.
    Online,

    /// Fired when the current player removes a friend from their friend list.
    Lost,

    /// The current player has sent a friend request to the given player.
    Requested,

    /// A friend was added to the current player's friend list.
    Added,

    /// Fired when a friend request was declined.
    Declined,
}
