use serde::{Deserialize, Serialize};

use crate::modules::galaxy::StarClass;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct StartJumpEvent {
    #[serde(flatten)]
    pub jump: StartJumpType,

    #[serde(default)]
    pub taxi: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase", tag = "JumpType")]
pub enum StartJumpType {
    #[serde(rename_all = "PascalCase")]
    Hyperspace {
        star_system: String,
        system_address: u64,
        star_class: StarClass,
    },
    Supercruise,
}
