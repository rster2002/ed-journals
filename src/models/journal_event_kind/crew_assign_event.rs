use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct CrewAssignEvent {
    pub name: String,

    #[serde(rename = "CrewID")]
    pub crew_id: u64,
    pub role: CrewAssignEventRole,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub enum CrewAssignEventRole {
    Active,
}
