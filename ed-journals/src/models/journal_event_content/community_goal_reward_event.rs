use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CommunityGoalRewardEvent {
    #[serde(rename = "GCID")]
    pub gcid: u64,
    pub name: String,
    pub system_name: String,
    pub reward: u64,
}
