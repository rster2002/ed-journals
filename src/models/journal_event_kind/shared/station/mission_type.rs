use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum MissionType {
    #[serde(alias = "Mission_Salvage")]
    BlackBoxSalvageOperation,

    #[serde(alias = "Mission_Courier_War_name")]
    DataCourier,

    #[serde(alias = "Mission_Massacre", alias = "Mission_Massacre_name")]
    MassacreMission,

    #[serde(alias = "Mission_MassacreWing", alias = "Mission_MassacreWing_name")]
    WingMassacreMission,

    #[serde(alias = "Mission_AltruismCredits", alias = "Mission_AltruismCredits_name")]
    DonationMission,

    #[serde(alias = "Mission_OnFoot_Smuggle_Contact_006", alias = "Mission_OnFoot_Smuggle_Contact_006_name")]
    OnFootSmuggleContract,

    #[serde(
        alias = "Mission_OnFoot_Massacre_MB",
        alias = "Mission_OnFoot_Massacre_MB_name",
    )]
    OnFootMassacreMission,

    #[serde(
        alias = "Mission_OnFoot_Assassination_MB",
        alias = "Mission_OnFoot_Assassination_MB_name",
    )]
    OnFootAssassinationMission,

    #[serde(alias = "Mission_OnFoot_Assassination_Hard_005")]
    OnFootAssassinationHard,

    #[serde(alias = "Mission_OnFoot_Sabotage_Production_002")]
    OnFootDisruptionMission,

    #[serde(alias = "Mission_Mining")]
    MiningMission,

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}
