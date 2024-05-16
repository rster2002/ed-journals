use serde::{Serialize, Deserialize};
use crate::modules::models::galaxy::star_class::StarClass;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct StartJumpEvent {
    #[serde(flatten)]
    pub jump: StartJumpType,

    #[serde(default)]
    pub taxi: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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
