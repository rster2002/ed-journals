use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct MaterialCollectedEvent {
    // TODO look into turning this into an enum
    pub category: String,

    // TODO look into turning this into an enum
    pub name: String,
    pub count: u8,
}
