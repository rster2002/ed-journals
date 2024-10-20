use serde::{Deserialize, Serialize};

use crate::modules::galaxy::StarClass;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct FSDTargetEvent {
    // TODO check when this is included
    #[serde(rename = "Starsystem")]
    star_system: Option<String>,
    name: String,

    #[serde(default)]
    remaining_jumps_in_route: u32,
    star_class: StarClass,
}
