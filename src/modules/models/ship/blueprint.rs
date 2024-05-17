use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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

    #[serde(rename = "Sensor_Expanded")]
    WideAngleSensors,

    #[serde(rename = "Sensor_LightWeight")]
    LightweightSensors,

    #[serde(rename = "PowerDistributor_HighFrequency")]
    PowerDistributorChargeEnhanced,

    #[serde(rename = "PowerDistributor_HighCapacity")]
    PowerDistributorHighCapacity,

    #[serde(rename = "FSD_LongRange")]
    FrameShiftDriveLongRange,

    #[serde(rename = "Armour_HeavyDuty")]
    HeavyDutyArmor,

    #[serde(rename = "Armour_Advanced")]
    LightweightArmor,

    #[serde(rename = "HullReinforcement_HeavyDuty")]
    HeavyDutyHullReinforcement,

    // TODO this is a guess
    #[serde(rename = "ShieldBooster_Resistive")]
    ShieldBoosterResistanceAugmented,

    #[serde(rename = "ShieldBooster_HeavyDuty")]
    ShieldBoosterHeavyDuty,

    #[serde(rename = "ShieldGenerator_Reinforced")]
    ShieldGeneratorReinforced,

    #[serde(rename = "ShieldGenerator_Kinetic")]
    ShieldGeneratorKineticResistant,

    #[serde(rename = "ShieldGenerator_Thermic")]
    ShieldGeneratorThermalResistant,

    #[serde(rename = "ShieldGenerator_Optimised")]
    ShieldGeneratorEnhancedLowPower,

    #[serde(rename = "Weapon_LightWeight")]
    LightweightWeapon,

    #[serde(rename = "Weapon_Sturdy")]
    SturdyWeapon,

    #[serde(rename = "Weapon_Overcharged")]
    OverchargedWeapon,

    #[serde(rename = "PowerPlant_Boosted")]
    PowerPlantOvercharged,

    #[serde(rename = "PowerPlant_Armoured")]
    PowerPlantArmored,

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}
