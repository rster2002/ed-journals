use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum MissionType {
    #[serde(rename = "Mission_Salvage")]
    BlackBoxSalvageOperation,

    #[serde(rename = "Mission_Courier_War_name")]
    DataCourier,

    #[serde(rename = "Mission_Massacre", alias = "Mission_Massacre_name")]
    MassacreMission,

    #[serde(rename = "Mission_MassacreWing", alias = "Mission_MassacreWing_name")]
    WingMassacreMission,

    #[serde(rename = "Mission_AltruismCredits", alias = "Mission_AltruismCredits_name")]
    DonationMission,

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}
