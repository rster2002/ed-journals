//! Fired when the player is the target of a crime.

use crate::civilization::Crime;
use serde::{Deserialize, Serialize};

/// Fired when the player is the target of a crime.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CrimeVictimEvent {
    /// The name of the one that has commited the crime against the player.
    pub offender: String,

    /// The crime committed against the player.
    pub crime_type: Crime,

    /// The fine that was dealt.
    pub fine: Option<u64>,

    /// The bounty that was placed.
    pub bounty: Option<u64>,
}
