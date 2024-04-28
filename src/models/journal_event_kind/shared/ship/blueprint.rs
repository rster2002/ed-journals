use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Blueprint {
    #[serde(rename = "PowerDistributor_PriorityEngines")]
    PowerDistributorEngineFocussed,

    // TODO this is a guess
    #[serde(rename = "PowerPlant_Stealth")]
    PowerPlantLowEmissions,

    #[serde(rename = "Engine_Dirty")]
    ThrustersDirty,

    #[serde(rename = "Weapon_LongRange")]
    WeaponLongRange,

    #[serde(rename = "Sensor_LongRange")]
    LongRangeSensors,

    #[serde(rename = "PowerDistributor_HighFrequency")]
    PowerDistributorChargeEnhanced,

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}
