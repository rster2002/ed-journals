use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct SupercruiseExitEvent {
    pub star_system: String,
    pub system_address: u64,
    pub body: String,

    #[serde(rename = "BodyID")]
    pub body_id: u8,

    // TODO look into turning this into an enum
    pub body_type: String,

    #[serde(default)]
    pub taxi: bool,

    #[serde(default)]
    pub multicrew: bool,
}
