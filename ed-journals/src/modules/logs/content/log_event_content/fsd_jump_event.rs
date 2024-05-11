use crate::modules::shared::civilization::system_info::SystemInfo;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct FSDJumpEvent {
    pub jump_dist: f32,
    pub fuel_used: f32,
    pub fuel_level: f32,

    #[serde(flatten)]
    pub system_info: SystemInfo,
}
