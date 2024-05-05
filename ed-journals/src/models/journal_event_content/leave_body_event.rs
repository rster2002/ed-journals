use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct LeaveBodyEvent {
    star_system: String,
    system_address: u64,
    body: String,

    #[serde(rename = "BodyID")]
    body_id: u8,
}
