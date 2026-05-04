//! Fired when the player receives a text message.

use serde::{Deserialize, Serialize};

/// Fired when the player receives a text message.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct ReceiveTextEvent {
    /// The sender of the message.
    pub from: String,

    /// The message itself. Depending on the origin of the sender this could either be the actual
    /// displayed message or a formatted string and the actual contents of the message would then be
    /// in `message_localized`.
    pub message: String,

    /// The localized message, if possible.
    #[serde(rename = "Message_Localised")]
    pub message_localized: Option<String>,

    // TODO replace with enum
    /// The channel the message was received on.
    pub channel: String,
}
