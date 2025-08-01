//! Fired when the player boosts their FSD using a jet cone.

use serde::{Deserialize, Serialize};

/// Fired when the player boosts their FSD using a jet cone.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct JetConeBoostEvent {
    /// The factor the FSD range was boosted.
    pub boost_value: f32,
}
