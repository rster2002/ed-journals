//! Fired when the carrier performs the jump after the jump countdown.

use serde::{Deserialize, Serialize};

use crate::modules::civilization::LocationInfo;

// TODO check when this is fired during the jump. Is it when it lockdown happens, the actual jump,
//  after the jump?
/// Fired when the carrier performs the jump after the jump countdown.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CarrierJumpEvent {
    // TODO check when this is true, specifically if this is true if you're on the ship when jumping
    //  or if this is also true when the ship is just docked and on the pad and the player is
    //  on-foot?
    pub docked: bool,

    /// Whether the player is currently on-foot when the jump occurs.
    #[serde(default)]
    pub on_foot: bool,

    /// Information about the system the carrier has jumped to.
    #[serde(flatten)]
    pub system_info: LocationInfo,
}
