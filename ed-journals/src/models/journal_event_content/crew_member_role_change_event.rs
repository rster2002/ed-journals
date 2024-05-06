use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CrewMemberRoleChangeEvent {
    pub name: String,

    #[serde(default)]
    pub telepresence: bool,
    pub role: CrewMemberRoleChangeEventRole,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum CrewMemberRoleChangeEventRole {
    Idle,
    FireCon,
    FighterCon,
}
