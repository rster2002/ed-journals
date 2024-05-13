use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CommunityGoalEvent {
    pub current_goals: Vec<CommunityGoalEventGoal>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CommunityGoalEventGoal {
    #[serde(rename = "GCID")]
    pub gcid: u64,
    pub title: String,
    pub system_name: String,
    pub market_name: String,
    pub expiry: DateTime<Utc>,
    pub is_complete: bool,
    pub current_total: u64,
    pub player_contribution: u64,

    #[serde(rename = "NumContributors")]
    pub number_of_contributors: u64,
    pub top_rank_size: u8,
    pub player_in_top_rank: bool,
    pub tier_reached: Option<String>,
    pub player_percentile_band: u8,
    pub bonus: u64,
}
