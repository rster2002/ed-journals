use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct FighterDestroyedEvent {
    #[serde(rename = "ID")]
    pub id: u8,
}
