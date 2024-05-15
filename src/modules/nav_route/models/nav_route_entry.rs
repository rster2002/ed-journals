use serde::Deserialize;
use crate::models::galaxy::star_class::StarClass;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct NavRouteEntry {
    pub star_system: String,
    pub system_address: u64,
    pub star_pos: [f32; 3],
    pub star_class: StarClass,
}
