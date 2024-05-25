use serde::{Deserialize, Serialize};

/// Engineering blueprint that can be applied to certain ship modules.
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

    #[serde(rename = "PowerDistributor_PriorityWeapons")]
    PowerDistributorPrioritizeWeapons,

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

    #[serde(rename = "ShieldCellBank_Rapid")]
    ShieldCellBankRapidCharge,

    #[serde(rename = "Weapon_LightWeight")]
    LightweightWeapon,

    #[serde(rename = "Weapon_Sturdy")]
    SturdyWeapon,

    #[serde(rename = "Weapon_Overcharged")]
    OverchargedWeapon,

    #[serde(rename = "Weapon_HighCapacity")]
    HighCapacityWeapon,

    #[serde(rename = "PowerPlant_Boosted")]
    PowerPlantOvercharged,

    #[serde(rename = "PowerPlant_Armoured")]
    PowerPlantArmored,

    #[serde(rename = "Misc_LightWeight")]
    Lightweight,

    #[serde(rename = "Misc_HeatSinkCapacity")]
    HeatSinkCapacity,

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}
