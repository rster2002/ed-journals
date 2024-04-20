use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct StartJumpEvent {
    #[serde(flatten)]
    pub jump: StartJumpType,
    pub taxi: bool,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase", tag = "JumpType")]
pub enum StartJumpType {
    #[serde(rename_all = "PascalCase")]
    Hyperspace {
        star_system: String,
        system_address: u64,
        star_class: String,
    },
    Supercruise,
}
