mod sightseeing_mission;

use crate::models::journal_event_kind::shared::station::mission_type::sightseeing_mission::SightseeingMission;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum MissionType {
    #[serde(alias = "Mission_Salvage")]
    BlackBoxSalvageOperation,

    // TODO replace with specific struct, as this is currently incorrect
    #[serde(
        alias = "Mission_Courier_War_name",
        alias = "Mission_Courier_RankFed",
        alias = "Mission_Courier_RankFed_name",
        alias = "Mission_Courier_Democracy",
        alias = "Mission_Courier_Democracy_name"
    )]
    DataCourier,

    #[serde(alias = "Mission_Massacre", alias = "Mission_Massacre_name")]
    MassacreMission,

    #[serde(alias = "Mission_MassacreWing", alias = "Mission_MassacreWing_name")]
    WingMassacreMission,

    // TODO replace with specific struct, as this is currently incorrect
    #[serde(
        alias = "Mission_AltruismCredits",
        alias = "Mission_AltruismCredits_name",
        alias = "Mission_AltruismCredits_Bust",
        alias = "Mission_AltruismCredits_Bust_name"
    )]
    DonationMission,

    #[serde(
        alias = "Mission_OnFoot_Smuggle_Contact_006",
        alias = "Mission_OnFoot_Smuggle_Contact_006_name"
    )]
    OnFootSmuggleContract,

    #[serde(
        alias = "Mission_OnFoot_Massacre_MB",
        alias = "Mission_OnFoot_Massacre_MB_name"
    )]
    OnFootMassacreMission,

    #[serde(
        alias = "Mission_OnFoot_Assassination_MB",
        alias = "Mission_OnFoot_Assassination_MB_name"
    )]
    OnFootAssassinationMission,

    #[serde(alias = "Mission_OnFoot_Assassination_Hard_005")]
    OnFootAssassinationHard,

    #[serde(alias = "Mission_OnFoot_Sabotage_Production_002")]
    OnFootDisruptionMission,

    #[serde(alias = "Mission_Mining", alias = "Mission_Mining_name")]
    MiningMission,

    #[serde(alias = "Mission_Sightseeing", alias = "Mission_Sightseeing_name")]
    UnspecifiedSightseeingMission,

    #[serde(
        alias = "Mission_OnFoot_RebootRestore_MB",
        alias = "Mission_OnFoot_RebootRestore_MB_name"
    )]
    OnFootRestorationMission,

    #[serde(untagged)]
    SightseeingMission(SightseeingMission),

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}
