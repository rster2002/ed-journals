use serde::Deserialize;

use crate::modules::shared::station::mission_type::sightseeing_mission::SightseeingMission;

mod sightseeing_mission;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum MissionType {
    #[serde(alias = "Mission_Salvage", alias = "Mission_Salvage_name")]
    BlackBoxSalvageOperation,

    // TODO replace with specific struct, as this is currently incorrect
    #[serde(
        alias = "Mission_Courier",
        alias = "Mission_Courier_name",
        alias = "Mission_Courier_War",
        alias = "Mission_Courier_War_name",
        alias = "Mission_Courier_RankFed",
        alias = "Mission_Courier_RankFed_name",
        alias = "Mission_Courier_Democracy",
        alias = "Mission_Courier_Democracy_name",
        alias = "Mission_Courier_Expansion",
        alias = "Mission_Courier_Expansion_name",
        alias = "Mission_Courier_Outbreak",
        alias = "Mission_Courier_Outbreak_name"
    )]
    DataCourier,

    #[serde(alias = "Mission_Massacre", alias = "Mission_Massacre_name")]
    MassacreMission,

    #[serde(alias = "Mission_MassacreWing", alias = "Mission_MassacreWing_name")]
    WingMassacreMission,

    // TODO replace with specific struct, as this is currently incorrect
    #[serde(
        alias = "Mission_Altruism",
        alias = "Mission_Altruism_name",
        alias = "Mission_AltruismCredits",
        alias = "Mission_AltruismCredits_name",
        alias = "Mission_AltruismCredits_Bust",
        alias = "Mission_AltruismCredits_Bust_name",
        alias = "Mission_AltruismCredits_Outbreak",
        alias = "Mission_AltruismCredits_Outbreak_name"
    )]
    DonationMission,

    #[serde(
        alias = "Mission_OnFoot_Smuggle_Contact_006",
        alias = "Mission_OnFoot_Smuggle_Contact_006_name"
    )]
    OnFootSmuggleContract,

    #[serde(
        alias = "Mission_OnFoot_Massacre_MB",
        alias = "Mission_OnFoot_Massacre_MB_name",
        alias = "Mission_OnFoot_Onslaught_MB",
        alias = "Mission_OnFoot_Onslaught_MB_name",
        alias = "Mission_OnFoot_Onslaught_Offline_MB",
        alias = "Mission_OnFoot_Onslaught_Offline_MB_name"
    )]
    OnFootMassacreMission,

    #[serde(
        alias = "Mission_OnFoot_Assassination_MB",
        alias = "Mission_OnFoot_Assassination_MB_name",
        alias = "Mission_Assassinate_Planetary",
        alias = "Mission_Assassinate_Planetary_name"
    )]
    OnFootAssassinationMission,

    #[serde(
        alias = "Mission_Assassinate_Legal_Corporate",
        alias = "Mission_Assassinate_Legal_Corporate_name"
    )]
    OnFootLegalAssassinationMission,

    #[serde(alias = "Mission_OnFoot_Assassination_Hard_005")]
    OnFootAssassinationHard,

    #[serde(
        alias = "Mission_OnFoot_Sabotage_Production_002",
        alias = "Mission_OnFoot_Sabotage_Production_002_name",
        alias = "Mission_OnFoot_Sabotage_Production_Covert_001",
        alias = "Mission_OnFoot_Sabotage_Production_Covert_001_name"
    )]
    OnFootDisruptionMission,

    #[serde(alias = "Mission_Mining", alias = "Mission_Mining_name")]
    MiningMission,

    #[serde(alias = "Mission_Sightseeing", alias = "Mission_Sightseeing_name")]
    UnspecifiedSightseeingMission,

    #[serde(alias = "Mission_HackMegaship", alias = "Mission_HackMegaship_name")]
    HackingMission,

    #[serde(
        alias = "Mission_Courier_Engineer",
        alias = "Mission_Courier_Engineer_name"
    )]
    EngineerInvitationMission,

    #[serde(
        alias = "Mission_OnFoot_Reboot_MB",
        alias = "Mission_OnFoot_Reboot_MB_name",
        alias = "Mission_OnFoot_RebootRestore_MB",
        alias = "Mission_OnFoot_RebootRestore_MB_name"
    )]
    OnFootRestorationMission,

    #[serde(
        alias = "Mission_OnFoot_Salvage_MB",
        alias = "Mission_OnFoot_Salvage_MB_name"
    )]
    OnFootSalvageMission,

    #[serde(
        alias = "Mission_OnFoot_Collect_002",
        alias = "Mission_OnFoot_Collect_002_name"
    )]
    OnFootCollectionMission,

    #[serde(
        alias = "Mission_OnFoot_Heist_POI_002",
        alias = "Mission_OnFoot_Heist_POI_002_name"
    )]
    OnFootHeistMission,

    #[serde(
        alias = "Mission_OnFoot_Hack_Download_007",
        alias = "Mission_OnFoot_Hack_Download_007_name",
        alias = "Mission_OnFoot_Hack_Download_Covert_007",
        alias = "Mission_OnFoot_Hack_Download_Covert_007_name"
    )]
    OnFootHackingMission,

    #[serde(untagged)]
    SightseeingMission(SightseeingMission),

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}
