use crate::models::journal_event_kind::shared::civilization::system_info::SystemInfo;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct FSDJumpEvent {
    pub jump_dist: f32,
    pub fuel_used: f32,
    pub fuel_level: f32,

    #[serde(flatten)]
    pub system_info: SystemInfo,
    // pub star_system: String,
    // pub system_address: u64,
    // pub star_pos: [f32; 3],
    // pub system_allegiance: String,
    // pub system_economy: String,
    //
    // #[serde(rename = "SystemEconomy_Localised")]
    // pub system_economy_localised: String,
    // pub system_second_economy: String,
    //
    // #[serde(rename = "SystemSecondEconomy_Localised")]
    // pub system_second_economy_localised: String,
    // pub system_government: String,
    //
    // #[serde(rename = "SystemGovernment_Localised")]
    // pub system_government_localised: String,
    // pub system_security: String,
    //
    // #[serde(rename = "SystemSecurity_Localised")]
    // pub system_security_localised: String,
    // pub population: u64,
    //
    //
    // #[serde(default)]
    // pub factions: Vec<FSDJumpEventFaction>,
    // pub system_faction: Option<FSDJumpEventSystemFaction>,
    //
    // #[serde(default)]
    // pub conflicts: Vec<FSDJumpEventConflict>,
    // pub thargoid_war: Option<FSDJumpEventThargoidWar>,
}

// #[derive(Debug, Deserialize)]
// #[cfg_attr(test, derive(PartialEq))]
// #[serde(rename_all = "PascalCase")]
// pub struct FSDJumpEventFaction {
//     pub name: String,
//     pub faction_state: String,
//     pub government: String,
//     pub influence: f32,
//     pub allegiance: String,
//     pub happiness: String,
//
//     #[serde(rename = "Happiness_Localised")]
//     pub happiness_localised: String,
//     pub my_reputation: f32,
//
//     #[serde(default)]
//     pub pending_states: Vec<FSDJumpEventActiveState>,
//
//     #[serde(default)]
//     pub recovering_states: Vec<FSDJumpEventActiveState>,
//
//     #[serde(default)]
//     pub active_states: Vec<FSDJumpEventActiveState>,
//
//     #[serde(default)]
//     pub squadron_faction: bool,
//
//     #[serde(default)]
//     pub happiest_system: bool,
//
//     #[serde(default)]
//     pub home_system: bool,
// }

// #[derive(Debug, Deserialize)]
// #[cfg_attr(test, derive(PartialEq))]
// #[serde(rename_all = "PascalCase")]
// pub struct FSDJumpEventActiveState {
//     // TODO make this an enum
//     pub state: String,
// }
//
// #[derive(Debug, Deserialize)]
// #[cfg_attr(test, derive(PartialEq))]
// #[serde(rename_all = "PascalCase")]
// pub struct FSDJumpEventSystemFaction {
//     pub name: String,
//
//     // TODO check what it means when this is [None]
//     pub faction_state: Option<String>,
// }
//
// #[derive(Debug, Deserialize)]
// #[cfg_attr(test, derive(PartialEq))]
// #[serde(rename_all = "PascalCase")]
// pub struct FSDJumpEventConflict {
//     pub war_type: String,
//     pub status: String,
//     pub faction_1: FSDJumpEventConflictFaction,
//     pub faction_2: FSDJumpEventConflictFaction,
// }
//
// #[derive(Debug, Deserialize)]
// #[cfg_attr(test, derive(PartialEq))]
// #[serde(rename_all = "PascalCase")]
// pub struct FSDJumpEventConflictFaction {
//     pub name: String,
//     pub stake: String,
//     pub won_days: u32,
// }
//
// #[derive(Debug, Deserialize)]
// #[cfg_attr(test, derive(PartialEq))]
// #[serde(rename_all = "PascalCase")]
// pub struct FSDJumpEventPowerplay {
//     pub powers: Vec<String>,
//     pub powerplay_state: FSDJumpEventPowerplayState,
// }
//
// #[derive(Debug, Deserialize)]
// #[cfg_attr(test, derive(PartialEq))]
// #[serde(rename_all = "PascalCase", untagged)]
// pub enum FSDJumpEventPowerplayState {
//     InPrepareRadius,
//     Prepared,
//     Exploited,
//     Contested,
//     Controlled,
//     Turmoil,
//     HomeSystem,
//     Other(String),
// }
//
// #[derive(Debug, Deserialize)]
// #[cfg_attr(test, derive(PartialEq))]
// #[serde(rename_all = "PascalCase")]
// pub struct FSDJumpEventThargoidWar {
//     pub current_state: String,
//     pub next_state_success: String,
//     pub next_state_failure: String,
//     pub success_state_reached: String,
//     pub war_progress: u8,
//     pub remaining_ports: u8,
//     pub estimated_remaining_time: u32,
// }

#[cfg(test)]
mod tests {
    use crate::models::journal_event_kind::fsd_jump_event::FSDJumpEvent;

    #[ignore]
    #[test]
    fn fsd_jump_event_is_parsed_correctly() {
        let value = serde_json::from_str::<FSDJumpEvent>(
            r#"
            {
                "StarSystem": "Eranin",
                "SystemAddress": 2832631632594,
                "StarPos": [
                    -22.84375,
                    36.53125,
                    -1.1875
                ],
                "SystemAllegiance": "Independent",
                "SystemEconomy": "$economy_Agri;",
                "SystemEconomy_Localised": "Agriculture",
                "SystemSecondEconomy": "$economy_Refinery;",
                "SystemSecondEconomy_Localised": "Refinery",
                "SystemGovernment": "$government_Anarchy;",
                "SystemGovernment_Localised": "Anarchy",
                "SystemSecurity": "$GAlAXY_MAP_INFO_state_anarchy;",
                "SystemSecurity_Localised": "Anarchy",
                "Population": 450000,
                "JumpDist": 13.334,
                "FuelUsed": 0,
                "FuelLevel": 25.630281,
                "Factions": [
                    {
                        "Name": "Eranin Expeditionary Institute",
                        "FactionState": "None",
                        "Government": "Cooperative",
                        "Influence": 0.17,
                        "Allegiance": "Independent",
                        "Happiness": "$Faction_HappinessBand2;",
                        "Happiness_Localised": "Happy",
                        "MyReputation": 0
                    },
                    {
                        "Name": "Eranin Peoples Party",
                        "FactionState": "CivilWar",
                        "Government": "Communism",
                        "Influence": 0.226,
                        "Allegiance": "Independent",
                        "Happiness": "$Faction_HappinessBand2;",
                        "Happiness_Localised": "Happy",
                        "MyReputation": 29.9743,
                        "ActiveStates": [
                            {
                                "State": "CivilWar"
                            }
                        ]
                    },
                    {
                        "Name": "Pilots Federation Local Branch",
                        "FactionState": "None",
                        "Government": "Democracy",
                        "Influence": 0,
                        "Allegiance": "PilotsFederation",
                        "Happiness": "$Faction_HappinessBand2;",
                        "Happiness_Localised": "Happy",
                        "MyReputation": 82.918297
                    },
                    {
                        "Name": "Eranin Industry",
                        "FactionState": "Outbreak",
                        "Government": "Corporate",
                        "Influence": 0.209,
                        "Allegiance": "Independent",
                        "Happiness": "$Faction_HappinessBand3;",
                        "Happiness_Localised": "Discontented",
                        "MyReputation": 0,
                        "ActiveStates": [
                            {
                                "State": "Famine"
                            },
                            {
                                "State": "Lockdown"
                            },
                            {
                                "State": "Outbreak"
                            }
                        ]
                    },
                    {
                        "Name": "Eranin Federal Bridge",
                        "FactionState": "CivilWar",
                        "Government": "Dictatorship",
                        "Influence": 0.226,
                        "Allegiance": "Independent",
                        "Happiness": "$Faction_HappinessBand2;",
                        "Happiness_Localised": "Happy",
                        "MyReputation": 0,
                        "ActiveStates": [
                            {
                                "State": "CivilWar"
                            }
                        ]
                    },
                    {
                        "Name": "Mob of Eranin",
                        "FactionState": "CivilLiberty",
                        "Government": "Anarchy",
                        "Influence": 0.134,
                        "Allegiance": "Independent",
                        "Happiness": "$Faction_HappinessBand1;",
                        "Happiness_Localised": "Elated",
                        "MyReputation": 0,
                        "ActiveStates": [
                            {
                                "State": "Boom"
                            },
                            {
                                "State": "CivilLiberty"
                            }
                        ]
                    },
                    {
                        "Name": "Terran Colonial Forces",
                        "FactionState": "CivilUnrest",
                        "Government": "Confederacy",
                        "Influence": 0.035,
                        "Allegiance": "Alliance",
                        "Happiness": "$Faction_HappinessBand2;",
                        "Happiness_Localised": "Happy",
                        "MyReputation": 0,
                        "ActiveStates": [
                            {
                                "State": "Boom"
                            },
                            {
                                "State": "CivilUnrest"
                            }
                        ]
                    }
                ],
                "SystemFaction": {
                    "Name": "Mob of Eranin",
                    "FactionState": "CivilLiberty"
                }
            }
        "#,
        );

        assert!(value.is_ok());
    }
}
