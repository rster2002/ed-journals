use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CommunityGoalRewardEvent {
    #[serde(rename = "CGID")]
    pub cgid: u64,
    pub name: String,

    #[serde(alias = "System")]
    pub system_name: String,
    pub reward: u64,
}
