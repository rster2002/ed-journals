use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct FSDTargetEvent {
    // TODO check when this is included
    #[serde(rename = "Starsystem")]
    star_system: Option<String>,
    name: String,

    #[serde(default)]
    remaining_jumps_in_route: u8,
    star_class: String,
}
