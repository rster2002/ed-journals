//! Fired when accessing information about currently active community goals.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Fired when accessing information about currently active community goals.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct CommunityGoalEvent {
    /// List of currently active community goals.
    pub current_goals: Vec<CommunityGoalEventGoal>,
}

/// An entry for a currently active community goal.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct CommunityGoalEventGoal {
    /// The id of the community goal.
    #[serde(rename = "CGID")]
    pub cgid: u64,

    /// The name of the community goal.
    pub title: String,

    /// The name of the system the community goal is located in.
    pub system_name: String,

    /// The market or station name the community goal is located at.
    pub market_name: String,

    /// When the community goal is planned to end.
    pub expiry: DateTime<Utc>,

    /// Whether the community goal has been completed.
    pub is_complete: bool,

    /// The current total contributions made by all players for this community goal.
    pub current_total: u64,

    /// How much the current player has contributed to the community goal.
    pub player_contribution: u64,

    /// Thw number of players that have contributed to the community goal.
    #[serde(rename = "NumContributors")]
    pub number_of_contributors: u64,

    /// The number of players that are considered part of the top rank, if the
    /// community goal has a fixed-size top rank.
    pub top_rank_size: Option<u8>,

    /// Whether the current player is currently in the top rank, if the community
    /// goal has a fixed-size top rank.
    pub player_in_top_rank: Option<bool>,

    /// The highest rank the current player has reached.
    pub tier_reached: Option<String>,
    pub player_percentile_band: u8,
    pub bonus: Option<u64>,
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

    #[test]
    fn community_goal_event_is_parsed_correctly_without_top_rank() {
        let goal: CommunityGoalEvent = serde_json::from_str(
            r#"
            {
              "timestamp": "2025-02-17T18:37:51Z",
              "event": "CommunityGoal",
              "CurrentGoals": [
                {
                  "CGID": 812,
                  "Title": "Brewer Corporation Planetary Survey Initiative",
                  "SystemName": "HIP 90578",
                  "MarketName": "Trailblazer Echo",
                  "Expiry": "2025-02-18T20:00:00Z",
                  "IsComplete": false,
                  "CurrentTotal": 7265965,
                  "PlayerContribution": 1431,
                  "NumContributors": 15065,
                  "TopTier": { "Name": "Tier 6", "Bonus": "" },
                  "TierReached": "Tier 5",
                  "PlayerPercentileBand": 25,
                  "Bonus": 125000000
                }
              ]
            }
          "#,
        )
        .unwrap();
    }
}
