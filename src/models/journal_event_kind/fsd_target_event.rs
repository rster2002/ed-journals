use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct FSDTargetEvent {
    #[serde(rename = "Starsystem")]
    star_system: String,
    name: String,
    remaining_jumps_in_route: u8,
    star_class: String,
}
