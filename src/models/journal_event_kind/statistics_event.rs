use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StatisticsEvent {
    #[serde(rename = "Bank_Account")]
    pub bank_account: StatisticsBankAccount,
    pub combat: StatisticsCombat,
    pub crime: StatisticsCrime,
    pub smuggling: StatisticsSmuggling,
    pub trading: StatisticsTrading,
    pub mining: StatisticsMining,
    pub exploration: StatisticsExploration,
    pub passengers: StatisticsPassengers,

    #[serde(rename = "Search_And_Rescue")]
    pub search_and_rescue: StatisticsSearchAndRescue,
    pub crafting: StatisticsCrafting,
    pub crew: StatisticsCrew,
    pub multicrew: StatisticsMulticrew,
}

#[derive(Debug, Deserialize)]
pub struct StatisticsBankAccount {
    #[serde(rename = "Current_Wealth")]
    pub current_wealth: u64,

    #[serde(rename = "Spent_On_Ships")]
    pub spent_on_ships: u64,

    #[serde(rename = "Spent_On_Outfitting")]
    pub spent_on_outfitting: u64,

    #[serde(rename = "Spent_On_Repairs")]
    pub spent_on_repairs: u64,

    #[serde(rename = "Spent_On_Fuel")]
    pub spent_on_fuel: u64,

    #[serde(rename = "Spent_On_Ammo_Consumables")]
    pub spent_on_ammo_consumables: u64,

    #[serde(rename = "Insurance_Claims")]
    pub insurance_claims: u64,

    #[serde(rename = "Spent_On_Insurance")]
    pub spent_on_insurance: u64,
}

#[derive(Debug, Deserialize)]
pub struct StatisticsCombat {
    #[serde(rename = "Bounties_Claimed")]
    pub bounties_claimed: u32,

    #[serde(rename = "Bounty_Hunting_Profit")]
    pub bounty_hunting_profit: u64,

    #[serde(rename = "Combat_Bonds")]
    pub combat_bonds: u32,

    #[serde(rename = "Combat_Bond_Profits")]
    pub combat_bond_profits: u64,

    #[serde(rename = "Assassinations")]
    pub assassinations: u32,

    #[serde(rename = "Assassination_Profits")]
    pub assassination_profits: u64,

    #[serde(rename = "Highest_Single_Reward")]
    pub highest_single_reward: u64,

    #[serde(rename = "Skimmers_Killed")]
    pub skimmers_killed: u32,
}

#[derive(Debug, Deserialize)]
pub struct StatisticsCrime {
    #[serde(rename = "Fines")]
    pub fines: u64,

    #[serde(rename = "Total_Fines")]
    pub total_fines: u64,

    #[serde(rename = "Bounties_Received")]
    pub bounties_received: u32,

    #[serde(rename = "Total_Bounties")]
    pub total_bounties: u64,

    #[serde(rename = "Highest_Bounty")]
    pub highest_bounty: u64,
}

#[derive(Debug, Deserialize)]
pub struct StatisticsSmuggling {
    #[serde(rename = "Black_Markets_Traded_With")]
    pub black_markets_traded_with: u32,

    #[serde(rename = "Black_Markets_Profits")]
    pub black_markets_profits: u64,

    #[serde(rename = "Resources_Smuggled")]
    pub resources_smuggled: u32,

    #[serde(rename = "Average_Profit")]
    pub average_profit: u64,

    #[serde(rename = "Highest_Single_Transaction")]
    pub highest_single_transaction: u64,
}

#[derive(Debug, Deserialize)]
pub struct StatisticsTrading {
    #[serde(rename = "Markets_Traded_With")]
    pub markets_traded_with: u32,

    #[serde(rename = "Market_Profits")]
    pub market_profits: u64,

    #[serde(rename = "Resources_Traded")]
    pub resources_traded: u32,

    #[serde(rename = "Average_Profit")]
    pub average_profit: u64,

    #[serde(rename = "Highest_Single_Transaction")]
    pub highest_single_transaction: u64,
}

#[derive(Debug, Deserialize)]
pub struct StatisticsMining {
    #[serde(rename = "Mining_Profits")]
    mining_profits: u64,

    #[serde(rename = "Quantity_Mined")]
    quantity_mined: u32,

    #[serde(rename = "Materials_Collected")]
    materials_collected: u32,
}

#[derive(Debug, Deserialize)]
pub struct StatisticsExploration {
    #[serde(rename = "Systems_Visited")]
    pub systems_visited: u32,

    #[serde(rename = "Fuel_Scooped")]
    pub fuel_scooped: u32,

    #[serde(rename = "Fuel_Purchased")]
    pub fuel_purchased: u32,

    #[serde(rename = "Exploration_Profits")]
    pub exploration_profits: u64,

    #[serde(rename = "Planets_Scanned_To_Level_2")]
    pub planets_scanned_to_level_2: u32,

    #[serde(rename = "Planets_Scanned_To_Level_3")]
    pub planets_scanned_to_level_3: u32,

    #[serde(rename = "Highest_Payout")]
    pub highest_payout: u64,

    #[serde(rename = "Total_Hyperspace_Distance")]
    pub total_hyperspace_distance: u32,

    #[serde(rename = "Total_Hyperspace_Jumps")]
    pub total_hyperspace_jumps: u32,

    #[serde(rename = "Greatest_Distance_From_Start")]
    pub greatest_distance_from_start: f32,

    #[serde(rename = "Time_Played")]
    pub time_played: u32,
}

#[derive(Debug, Deserialize)]
pub struct StatisticsPassengers {
    #[serde(rename = "Passengers_Missions_Bulk")]
    pub passengers_missions_bulk: u32,

    #[serde(rename = "Passengers_Missions_VIP")]
    pub passengers_missions_vip: u32,

    #[serde(rename = "Passengers_Missions_Delivered")]
    pub passengers_missions_delivered: u32,

    #[serde(rename = "Passengers_Missions_Ejected")]
    pub passengers_missions_ejected: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StatisticsSearchAndRescue {
    #[serde(rename = "SearchRescue_Traded")]
    pub search_rescue_traded: u32,

    #[serde(rename = "SearchRescue_Profit")]
    pub search_rescue_profit: u64,

    #[serde(rename = "SearchRescue_Count")]
    pub search_rescue_count: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StatisticsCrafting {
    #[serde(rename = "Spent_On_Crafting")]
    pub spent_on_crafting: u64,

    #[serde(rename = "Count_Of_Used_Engineers")]
    pub count_of_used_engineers: u32,

    #[serde(rename = "Recipes_Generated")]
    pub recipes_generated: u32,

    #[serde(rename = "Recipes_Generated_Rank_1")]
    pub recipes_generated_rank_1: u32,

    #[serde(rename = "Recipes_Generated_Rank_2")]
    pub recipes_generated_rank_2: u32,

    #[serde(rename = "Recipes_Generated_Rank_3")]
    pub recipes_generated_rank_3: u32,

    #[serde(rename = "Recipes_Generated_Rank_4")]
    pub recipes_generated_rank_4: u32,

    #[serde(rename = "Recipes_Generated_Rank_5")]
    pub recipes_generated_rank_5: u32,

    #[serde(rename = "Recipes_Applied")]
    pub recipes_applied: u32,

    #[serde(rename = "Recipes_Applied_Rank_1")]
    pub recipes_applied_rank_1: u32,

    #[serde(rename = "Recipes_Applied_Rank_2")]
    pub recipes_applied_rank_2: u32,

    #[serde(rename = "Recipes_Applied_Rank_3")]
    pub recipes_applied_rank_3: u32,

    #[serde(rename = "Recipes_Applied_Rank_4")]
    pub recipes_applied_rank_4: u32,

    #[serde(rename = "Recipes_Applied_Rank_5")]
    pub recipes_applied_rank_5: u32,

    #[serde(rename = "Recipes_Applied_On_Previously_Modified_Modules")]
    pub recipes_applied_on_previously_modified_modules: u32,
}

#[derive(Debug, Deserialize)]
pub struct StatisticsCrew {
    #[serde(rename = "NpcCrew_TotalWages")]
    pub npc_crew_total_wages: u64,

    #[serde(rename = "NpcCrew_Hired")]
    pub npc_crew_hired: u32,

    #[serde(rename = "NpcCrew_Fired")]
    pub npc_crew_fired: u32,

    #[serde(rename = "NpcCrew_Died")]
    pub npc_crew_died: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StatisticsMulticrew {
    #[serde(rename = "Multicrew_Time_Total")]
    pub multicrew_time_total: u32,

    #[serde(rename = "Multicrew_Gunner_Time_Total")]
    pub multicrew_gunner_time_total: u32,

    #[serde(rename = "Multicrew_Fighter_Time_Total")]
    pub multicrew_fighter_time_total: u32,

    #[serde(rename = "Multicrew_Credits_Total")]
    pub multicrew_credits_total: u64,

    #[serde(rename = "Multicrew_Fines_Total")]
    pub multicrew_fines_total: u64,
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use crate::models::journal_event_kind::statistics_event::StatisticsEvent;

    #[test]
    fn statistics_event_is_parsed_correctly() {
        let value = serde_json::from_value::<StatisticsEvent>(json!({
            "Bank_Account": {
                "Current_Wealth": 148827050,
                "Spent_On_Ships": 14499177,
                "Spent_On_Outfitting": 30785093,
                "Spent_On_Repairs": 17716,
                "Spent_On_Fuel": 1647,
                "Spent_On_Ammo_Consumables": 0,
                "Insurance_Claims": 4,
                "Spent_On_Insurance": 1397620
            },
            "Combat": {
                "Bounties_Claimed": 0,
                "Bounty_Hunting_Profit": 0,
                "Combat_Bonds": 0,
                "Combat_Bond_Profits": 0,
                "Assassinations": 0,
                "Assassination_Profits": 0,
                "Highest_Single_Reward": 0,
                "Skimmers_Killed": 0
            },
            "Crime": {
                "Fines": 0,
                "Total_Fines": 0,
                "Bounties_Received": 0,
                "Total_Bounties": 0,
                "Highest_Bounty": 0
            },
            "Smuggling": {
                "Black_Markets_Traded_With": 0,
                "Black_Markets_Profits": 0,
                "Resources_Smuggled": 0,
                "Average_Profit": 0,
                "Highest_Single_Transaction": 0
            },
            "Trading": {
                "Markets_Traded_With": 3,
                "Market_Profits": 40700,
                "Resources_Traded": 23,
                "Average_Profit": 4070,
                "Highest_Single_Transaction": 17961
            },
            "Mining": {
                "Mining_Profits": 0,
                "Quantity_Mined": 0,
                "Materials_Collected": 100
            },
            "Exploration": {
                "Systems_Visited": 228,
                "Fuel_Scooped": 111,
                "Fuel_Purchased": 0,
                "Exploration_Profits": 304469,
                "Planets_Scanned_To_Level_2": 39,
                "Planets_Scanned_To_Level_3": 15,
                "Highest_Payout": 52503,
                "Total_Hyperspace_Distance": 844927,
                "Total_Hyperspace_Jumps": 295,
                "Greatest_Distance_From_Start": 65222.47204614,
                "Time_Played": 651060
            },
            "Passengers": {
                "Passengers_Missions_Bulk": 0,
                "Passengers_Missions_VIP": 0,
                "Passengers_Missions_Delivered": 0,
                "Passengers_Missions_Ejected": 0
            },
            "Search_And_Rescue": {
                "SearchRescue_Traded": 12,
                "SearchRescue_Profit": 19467,
                "SearchRescue_Count": 8
            },
            "Crafting": {
                "Spent_On_Crafting": 0,
                "Count_Of_Used_Engineers": 2,
                "Recipes_Generated": 28,
                "Recipes_Generated_Rank_1": 9,
                "Recipes_Generated_Rank_2": 6,
                "Recipes_Generated_Rank_3": 9,
                "Recipes_Generated_Rank_4": 4,
                "Recipes_Generated_Rank_5": 0,
                "Recipes_Applied": 21,
                "Recipes_Applied_Rank_1": 8,
                "Recipes_Applied_Rank_2": 5,
                "Recipes_Applied_Rank_3": 7,
                "Recipes_Applied_Rank_4": 1,
                "Recipes_Applied_Rank_5": 0,
                "Recipes_Applied_On_Previously_Modified_Modules": 0
            },
            "Crew": {
                "NpcCrew_TotalWages": 0,
                "NpcCrew_Hired": 0,
                "NpcCrew_Fired": 0,
                "NpcCrew_Died": 0
            },
            "Multicrew": {
                "Multicrew_Time_Total": 23327,
                "Multicrew_Gunner_Time_Total": 14241,
                "Multicrew_Fighter_Time_Total": 6070,
                "Multicrew_Credits_Total": 0,
                "Multicrew_Fines_Total": 0
            }
        }));

        assert!(value.is_ok());
    }
}
