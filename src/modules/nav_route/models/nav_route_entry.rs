use serde::{Deserialize, Serialize};

use crate::modules::galaxy::StarClass;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct NavRouteEntry {
    pub star_system: String,
    pub system_address: u64,
    pub star_pos: [f32; 3],
    pub star_class: StarClass,
}
