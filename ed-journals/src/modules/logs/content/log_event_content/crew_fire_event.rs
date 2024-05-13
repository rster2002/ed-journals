use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CrewFireEvent {
    #[serde(rename = "CrewID")]
    pub crew_id: u64,
    pub name: String,
}
