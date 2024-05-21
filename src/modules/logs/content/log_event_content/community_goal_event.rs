use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CommunityGoalEvent {
    pub current_goals: Vec<CommunityGoalEventGoal>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CommunityGoalEventGoal {
    #[serde(rename = "CGID")]
    pub cgid: u64,
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

#[cfg(test)]
mod tests {
    use crate::logs::content::log_event_content::community_goal_event::{
        CommunityGoalEvent, CommunityGoalEventGoal,
    };

    #[test]
    fn community_goal_event_is_parsed_correctly() {
        let goal: CommunityGoalEvent = serde_json::from_str(
            r#"
            {
              "timestamp": "2024-04-02T16:09:59Z",
              "event": "CommunityGoal",
              "CurrentGoals": [
                {
                  "CGID": 804,
                  "Title": "Aid Achilles Aerospace in Researching Titan Travel Technology",
                  "SystemName": "Ethgreze",
                  "MarketName": "Bloch Station",
                  "Expiry": "2024-04-04T07:00:00Z",
                  "IsComplete": false,
                  "CurrentTotal": 15321,
                  "PlayerContribution": 0,
                  "NumContributors": 5410,
                  "TopTier": {
                    "Name": "Tier 2",
                    "Bonus": ""
                  },
                  "TopRankSize": 10,
                  "PlayerInTopRank": false,
                  "TierReached": "Tier 1",
                  "PlayerPercentileBand": 100,
                  "Bonus": 500000
                }
              ]
            }
        "#,
        )
        .unwrap();
    }

    #[test]
    fn community_goal_is_parsed_correctly() {
        let goal: CommunityGoalEventGoal = serde_json::from_str(
            r#"
            {
              "CGID": 804,
              "Title": "Aid Achilles Aerospace in Researching Titan Travel Technology",
              "SystemName": "Ethgreze",
              "MarketName": "Bloch Station",
              "Expiry": "2024-04-04T07:00:00Z",
              "IsComplete": false,
              "CurrentTotal": 15321,
              "PlayerContribution": 0,
              "NumContributors": 5410,
              "TopTier": {
                "Name": "Tier 2",
                "Bonus": ""
              },
              "TopRankSize": 10,
              "PlayerInTopRank": false,
              "TierReached": "Tier 1",
              "PlayerPercentileBand": 100,
              "Bonus": 500000
            }
        "#,
        )
        .unwrap();
    }
}
