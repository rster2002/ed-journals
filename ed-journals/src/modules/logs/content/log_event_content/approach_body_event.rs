//! Fired whenever the player approaches a body.

use serde::{Deserialize, Serialize};

/// Fired whenever the player approaches a body. This is usually when the game also performs a scan
/// which fires a [ScanEvent](crate::logs::scan_event::ScanEvent).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ApproachBodyEvent {
    /// The star system the approached body is part of.
    pub star_system: String,

    /// The address of the system this approached body is in.
    pub system_address: u64,

    /// The name of the body which the player is approaching.
    pub body: String,

    /// The id of the approached body.
    #[serde(rename = "BodyID")]
    pub body_id: u8,
}
