use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CommunityGoalJoinEvent {
    #[serde(rename = "CGID")]
    pub cgid: u64,
    pub name: String,
    pub system: String,
}
