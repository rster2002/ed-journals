use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialCollectedEvent {
    // TODO look into turning this into an enum
    pub category: String,

    // TODO look into turning this into an enum
    pub name: String,
    pub count: u8,
}
