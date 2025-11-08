use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// Engineering blueprint that can be applied to certain ship modules.
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Blueprint {
    // Armor
    #[serde(rename = "Armour_Explosive")]
    ArmorBlastResistant,

    #[serde(rename = "Armour_HeavyDuty")]
    ArmorHeavyDuty,

    #[serde(rename = "Armour_Kinetic")]
    ArmorKineticResistant,

    #[serde(rename = "Armour_Advanced")]
    ArmorLightweight,

    #[serde(rename = "Armour_Thermic")]
    ArmorThermalResistant,

    // FSD Interdictor
    #[serde(rename = "FSDinterdictor_Expanded")]
    FSDInterdictorExpandedCaptureArc,

    #[serde(rename = "FSDinterdictor_LongRange")]
    FSDInterdictorLongRange,

    // Frame shift drive
    #[serde(rename = "FSD_LongRange")]
    FrameShiftDriveLongRange,

    #[serde(rename = "FSD_FastBoot")]
    FrameShiftDriveFasterBootSequence,

    #[serde(rename = "FSD_Shielded")]
    FrameShiftDriveShielded,

    // Hull reinforcement
    #[serde(rename = "HullReinforcement_Explosive")]
    HullReinforcementBlastResistant,

    #[serde(rename = "HullReinforcement_HeavyDuty")]
    HullReinforcementHeavyDuty,

    #[serde(rename = "HullReinforcement_Kinetic")]
    HullReinforcementKineticResistant,

    #[serde(rename = "HullReinforcement_Advanced")]
    HullReinforcementLightweight,

    #[serde(rename = "HullReinforcement_Thermic")]
    HullReinforcementThermalResistant,

    // Misc
    #[serde(rename = "Misc_LightWeight")]
    MiscLightweight,

    #[serde(rename = "Misc_Reinforced")]
    MiscReinforced,

    #[serde(rename = "Misc_Shielded")]
    MiscShielded,

    // Power distributor
    #[serde(rename = "PowerDistributor_HighFrequency")]
    PowerDistributorChargeEnhanced,

    #[serde(rename = "PowerDistributor_PriorityEngines")]
    PowerDistributorEngineFocussed,

    #[serde(rename = "PowerDistributor_HighCapacity")]
    PowerDistributorHighCapacity,

    #[serde(rename = "PowerDistributor_PriorityEngines")]
    PowerDistributorPrioritizeEngines,

    #[serde(rename = "PowerDistributor_PriorityWeapons")]
    PowerDistributorPrioritizeWeapons,

    #[serde(rename = "PowerDistributor_PrioritySystems")]
    PowerDistributorPrioritizeSystems,

    #[serde(rename = "PowerDistributor_Shielded")]
    PowerDistributorShielded,

    // Powerplant
    #[serde(rename = "PowerPlant_Stealth")]
    PowerPlantLowEmissions,

    #[serde(rename = "PowerPlant_Boosted")]
    PowerPlantOvercharged,

    #[serde(rename = "PowerPlant_Armoured")]
    PowerPlantArmored,

    // Sensors
    #[serde(rename = "Sensor_FastScan")]
    SensorsFastScan,

    #[serde(rename = "Sensor_LongRange")]
    SensorsLongRange,

    #[serde(rename = "Sensor_WideAngle")]
    SensorsWideAngle,

    #[serde(rename = "Sensor_LightWeight")]
    SensorsLightweight,

    #[serde(rename = "Sensor_Expanded")]
    SensorsExpandedRadius,

    // Shield booster
    #[serde(rename = "ShieldBooster_Explosive")]
    ShieldBoosterBlastResistant,

    #[serde(rename = "ShieldBooster_Resistive")]
    ShieldBoosterResistanceAugmented,

    #[serde(rename = "ShieldBooster_HeavyDuty")]
    ShieldBoosterHeavyDuty,

    #[serde(rename = "ShieldBooster_Kinetic")]
    ShieldBoosterKineticResistant,

    #[serde(rename = "ShieldBooster_Thermic")]
    ShieldBoosterThermalResistant,

    // Shield cell bank
    #[serde(rename = "ShieldCellBank_Rapid")]
    ShieldCellBankRapidCharge,

    #[serde(rename = "ShieldCellBank_Specialised")]
    ShieldCellBankSpecialized,

    // Shield generator
    #[serde(rename = "ShieldGenerator_Reinforced")]
    ShieldGeneratorReinforced,

    #[serde(rename = "ShieldGenerator_Kinetic")]
    ShieldGeneratorKineticResistant,

    #[serde(rename = "ShieldGenerator_Thermic")]
    ShieldGeneratorThermalResistant,

    #[serde(rename = "ShieldGenerator_Optimised")]
    ShieldGeneratorEnhancedLowPower,

    // Thrusters
    #[serde(rename = "Engine_Dirty")]
    ThrustersDirtyTuning,

    #[serde(rename = "Engine_Tuned")]
    ThrustersCleanTuning,

    #[serde(rename = "Engine_Reinforced")]
    ThrustersStrengthened,

    // Weapons
    #[serde(rename = "Weapon_LightWeight")]
    WeaponLightweight,

    #[serde(rename = "Weapon_Sturdy")]
    WeaponSturdy,

    #[serde(rename = "Weapon_Overcharged")]
    WeaponOvercharged,

    #[serde(rename = "Weapon_HighCapacity")]
    WeaponHighCapacity,

    #[serde(rename = "Weapon_DoubleShot")]
    WeaponDoubleShot,

    #[serde(rename = "Weapon_Efficient")]
    WeaponEfficient,

    #[serde(rename = "Weapon_Focused")]
    WeaponFocussed,

    #[serde(rename = "Weapon_LongRange")]
    WeaponLongRange,

    #[serde(rename = "Weapon_ShortRange")]
    WeaponShortRange,

    #[serde(rename = "Weapon_RapidFire")]
    WeaponRapidFire,

    // Capacity
    #[serde(rename = "Misc_HeatSinkCapacity")]
    HeatSinkCapacity,

    #[serde(rename = "Misc_PointDefenseCapacity")]
    PointDefenceCapacity,

    #[serde(rename = "Misc_ChaffCapacity")]
    ChaffCapacity,

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    #[serde(untagged)]
    Unknown(String),
}

impl Display for Blueprint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Blueprint::ArmorBlastResistant => "Blast Resistant",
                Blueprint::ArmorHeavyDuty => "Heavy Duty",
                Blueprint::ArmorKineticResistant => "Kinetic Resistant",
                Blueprint::ArmorLightweight => "Lightweight",
                Blueprint::ArmorThermalResistant => "Thermal Resistant",
                Blueprint::ChaffCapacity => "Ammo capacity",
                Blueprint::FSDInterdictorExpandedCaptureArc => "Expanded Capture Arc",
                Blueprint::FSDInterdictorLongRange => "Long Range",
                Blueprint::FrameShiftDriveFasterBootSequence => "Faster Boot Sequence",
                Blueprint::FrameShiftDriveLongRange => "Long Range",
                Blueprint::FrameShiftDriveShielded => "Shielded",
                Blueprint::HeatSinkCapacity => "Ammo capacity",
                Blueprint::HullReinforcementBlastResistant => "Blast Resistant",
                Blueprint::HullReinforcementHeavyDuty => "Heavy Duty",
                Blueprint::HullReinforcementKineticResistant => "Kinetic Resistant",
                Blueprint::HullReinforcementLightweight => "Lightweight",
                Blueprint::HullReinforcementThermalResistant => "Thermal Resistant",
                Blueprint::MiscLightweight => "Lightweight",
                Blueprint::MiscReinforced => "Reinforced",
                Blueprint::MiscShielded => "Shielded",
                Blueprint::PointDefenceCapacity => "Ammo capacity",
                Blueprint::PowerDistributorChargeEnhanced => "Charge Enhanced",
                Blueprint::PowerDistributorEngineFocussed => "Engine Focussed",
                Blueprint::PowerDistributorHighCapacity => "High Capacity",
                Blueprint::PowerDistributorPrioritizeEngines => "Prioritize Engines",
                Blueprint::PowerDistributorPrioritizeSystems => "Prioritize Systems",
                Blueprint::PowerDistributorPrioritizeWeapons => "Prioritize Weapons",
                Blueprint::PowerDistributorShielded => "Shielded",
                Blueprint::PowerPlantArmored => "Armored",
                Blueprint::PowerPlantLowEmissions => "Low Emissions",
                Blueprint::PowerPlantOvercharged => "Overcharged",
                Blueprint::SensorsFastScan => "Fast Scan",
                Blueprint::SensorsLightweight => "Lightweight",
                Blueprint::SensorsLongRange => "Long Range",
                Blueprint::SensorsWideAngle => "Wide Angle",
                Blueprint::SensorsExpandedRadius => "Wide Angle",
                Blueprint::ShieldBoosterBlastResistant => "Expanded Radius",
                Blueprint::ShieldBoosterHeavyDuty => "Heavy Duty",
                Blueprint::ShieldBoosterKineticResistant => "Kinetic Resistant",
                Blueprint::ShieldBoosterResistanceAugmented => "Resistance Augmented",
                Blueprint::ShieldBoosterThermalResistant => "Thermal Resistant",
                Blueprint::ShieldCellBankRapidCharge => "Rapid Charge",
                Blueprint::ShieldCellBankSpecialized => "Specialized",
                Blueprint::ShieldGeneratorEnhancedLowPower => "Enhanced, Low Power",
                Blueprint::ShieldGeneratorKineticResistant => "Kinetic Resistant",
                Blueprint::ShieldGeneratorReinforced => "Reinforced",
                Blueprint::ShieldGeneratorThermalResistant => "Thermal Resistant",
                Blueprint::ThrustersCleanTuning => "Clean Tuning",
                Blueprint::ThrustersDirtyTuning => "Dirty Tuning",
                Blueprint::ThrustersStrengthened => "Strengthened",
                Blueprint::WeaponDoubleShot => "Double Shot",
                Blueprint::WeaponEfficient => "Efficient",
                Blueprint::WeaponFocussed => "Focussed",
                Blueprint::WeaponHighCapacity => "High Capacity",
                Blueprint::WeaponLightweight => "Lightweight",
                Blueprint::WeaponLongRange => "Long Range",
                Blueprint::WeaponOvercharged => "Overcharge",
                Blueprint::WeaponRapidFire => "Rapid Fire",
                Blueprint::WeaponShortRange => "Short Range",
                Blueprint::WeaponSturdy => "Sturdy",

                #[cfg(feature = "allow-unknown")]
                Blueprint::Unknown(unknown) => return write!(f, "Unknown blueprint: {unknown}"),
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::ship::Blueprint;
    use serde_json::Value;

    #[test]
    fn all_blueprint_test_cases_are_parsed_correctly() {
        let contents = include_str!("zz_blueprint_test_cases.txt");
        let lines = contents.lines();

        for line in lines {
            dbg!(&line);
            let result = serde_json::from_value::<Blueprint>(Value::String(line.to_string()));

            assert!(result.is_ok());
        }
    }
}
