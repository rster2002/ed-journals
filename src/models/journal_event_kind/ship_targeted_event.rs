use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct ShipTargetedEvent {
    pub target_locked: bool,

    #[serde(flatten)]
    pub scan_stage: ShipTargetedEventScanStage,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase", untagged)]
pub enum ShipTargetedEventScanStage {
    Locked(ShipTargetedEventScanStageLocked),
    StageOne(ShipTargetedEventScanStageOne),
    StageTwo(ShipTargetedEventScanStageTwo),
    StageThree(ShipTargetedEventScanStageThree),
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct ShipTargetedEventScanStageLocked {
    // TODO replace this with an enum
    pub ship: String,

    #[serde(rename = "Ship_Localised")]
    pub ship_localised: Option<String>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct ShipTargetedEventScanStageOne {
    // TODO replace this with an enum
    pub ship: String,

    #[serde(rename = "Ship_Localised")]
    pub ship_localised: String,

    // TODO replace this with an enum
    pub pilot_name: String,

    #[serde(rename = "PilotName_Localised")]
    pub pilot_name_localised: String,

    pub pilot_rank: String,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct ShipTargetedEventScanStageTwo {
    // TODO replace this with an enum
    pub ship: String,

    #[serde(rename = "Ship_Localised")]
    pub ship_localised: String,

    // TODO replace this with an enum
    pub pilot_name: String,

    #[serde(rename = "PilotName_Localised")]
    pub pilot_name_localised: String,

    pub pilot_rank: String,
    pub shield_health: f32,
    pub hull_health: f32,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct ShipTargetedEventScanStageThree {
    // TODO replace this with an enum
    pub ship: String,

    #[serde(rename = "Ship_Localised")]
    pub ship_localised: String,

    // TODO replace this with an enum
    pub pilot_name: String,

    #[serde(rename = "PilotName_Localised")]
    pub pilot_name_localised: String,

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
