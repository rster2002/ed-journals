use serde::{Serialize, Deserialize};

use crate::modules::models::ship::ship_type::ShipType;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ShipTargetedEvent {
    pub target_locked: bool,

    /// [None] is used when [target_locked] is false.
    #[serde(flatten, default)]
    pub scan_stage: Option<ShipTargetedEventScanStage>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase", untagged)]
pub enum ShipTargetedEventScanStage {
    Locked(ShipTargetedEventScanStageLocked),
    StageOne(ShipTargetedEventScanStageOne),
    StageTwo(ShipTargetedEventScanStageTwo),
    StageThree(ShipTargetedEventScanStageThree),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ShipTargetedEventScanStageLocked {
    pub ship: ShipType,

    #[serde(rename = "Ship_Localised")]
    pub ship_localized: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ShipTargetedEventScanStageOne {
    pub ship: ShipType,

    #[serde(rename = "Ship_Localised")]
    pub ship_localized: String,
    pub pilot_name: String,

    #[serde(rename = "PilotName_Localised")]
    pub pilot_name_localized: String,

    pub pilot_rank: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ShipTargetedEventScanStageTwo {
    pub ship: ShipType,

    #[serde(rename = "Ship_Localised")]
    pub ship_localized: String,

    // TODO replace this with an enum
    pub pilot_name: String,

    #[serde(rename = "PilotName_Localised")]
    pub pilot_name_localized: String,

    pub pilot_rank: String,
    pub shield_health: f32,
    pub hull_health: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ShipTargetedEventScanStageThree {
    pub ship: ShipType,

    #[serde(rename = "Ship_Localised")]
    pub ship_localized: String,
    pub pilot_name: String,

    #[serde(rename = "PilotName_Localised")]
    pub pilot_name_localized: String,

    pub pilot_rank: String,
    pub shield_health: f32,
    pub hull_health: f32,

    // TODO the wiki says this field should be present, but my logs don't seem to contain then.
    pub faction: String,

    // TODO replace this with an enum
    pub legal_status: String,

    // TODO maybe this can be replaced with an enum
    pub sub_system: String,
    pub sub_system_health: f32,

    pub power: Option<String>,
}
