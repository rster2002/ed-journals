use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde::{Serialize, Deserialize};
use serde_json::Value;

use crate::modules::shared::ship::ship_module::ship_internal_module::armor_module::ArmorModule;
use crate::modules::shared::ship::ship_module::ship_internal_module::internal_type::InternalType;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum InternalModule {
    #[serde(rename = "hyperdrive")]
    FrameShiftDrive,

    #[serde(rename = "powerplant")]
    PowerPlant,

    #[serde(rename = "modulereinforcement")]
    ModuleReinforcement,

    #[serde(rename = "guardianmodulereinforcement")]
    GuardianModuleReinforcement,

    #[serde(rename = "guardianshieldreinforcement")]
    GuardianShieldReinforcement,

    #[serde(rename = "hullreinforcement")]
    HullReinforcement,

    #[serde(rename = "dockingcomputer_advanced")]
    AdvancedDockingComputer,

    #[serde(rename = "dronecontrol_collection")]
    CollectorLimpetController,

    #[serde(rename = "dronecontrol_repair")]
    RepairLimpetController,

    #[serde(rename = "dronecontrol_prospector")]
    ProspectorLimpetController,

    #[serde(rename = "multidronecontrol_mining")]
    MiningMultiLimpetController,

    #[serde(rename = "multidronecontrol_xeno")]
    XenoMultiLimpetController,

    #[serde(rename = "multidronecontrol_rescue")]
    RescueMultiLimpetController,

    #[serde(rename = "cargorack")]
    CargoRack,

    #[serde(rename = "corrosionproofcargorack")]
    AntiCorrosionCargoRack,

    #[serde(rename = "supercruiseassist")]
    SupercruiseAssist,

    #[serde(rename = "engine")]
    Thrusters,

    #[serde(rename = "fuelscoop")]
    FuelScoop,

    #[serde(rename = "lifesupport")]
    LifeSupport,

    #[serde(rename = "shieldgenerator")]
    ShieldGenerator,

    #[serde(rename = "shieldgenerator_fast")]
    BiWeaveShieldGenerator,

    #[serde(rename = "shieldcellbank")]
    ShieldCellBank,

    #[serde(rename = "guardianfsdbooster")]
    GuardianFSDBooster,

    #[serde(rename = "detailedsurfacescanner")]
    DetailedSurfaceScanner,

    #[serde(rename = "buggybay")]
    PlanetaryVehicleHangar,

    #[serde(rename = "powerdistributor")]
    PowerDistributor,

    #[serde(rename = "sensors")]
    Sensors,

    #[serde(rename = "repairer")]
    AFMU,

    #[serde(rename = "fighterbay")]
    FighterHangar,

    #[serde(rename = "passengercabin")]
    PassengerCabin,

    #[serde(rename = "fueltank")]
    FuelTank,

    #[serde(rename = "fsdinterdictor")]
    FSDInterdictor,

    #[serde(rename = "planetapproachsuite_advanced")]
    PlanetApproachSuite,

    #[serde(rename = "refinery")]
    Refinery,

    #[serde(untagged)]
    Armor(ArmorModule),
}

impl FromStr for InternalModule {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_value(Value::String(s.to_ascii_lowercase()))
    }
}

impl InternalModule {
    pub fn internal_type(&self) -> InternalType {
        match self {
            InternalModule::FrameShiftDrive
            | InternalModule::PowerPlant
            | InternalModule::Thrusters
            | InternalModule::PowerDistributor
            | InternalModule::LifeSupport
            | InternalModule::PlanetApproachSuite
            | InternalModule::Sensors => InternalType::Core,

            _ => InternalType::Optional,
        }
    }

    pub fn is_core(&self) -> bool {
        matches!(self.internal_type(), InternalType::Core)
    }

    pub fn is_optional(&self) -> bool {
        matches!(self.internal_type(), InternalType::Optional)
    }
}

impl Display for InternalModule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                InternalModule::AFMU => "Auto Field-Maintenance Unit",
                InternalModule::AdvancedDockingComputer => "Advanced Docking Computer",
                InternalModule::AntiCorrosionCargoRack => "Anti-Corrosion Cargo Rack",
                InternalModule::BiWeaveShieldGenerator => "Bi-Weave Shield Generator",
                InternalModule::CargoRack => "Cargo Rack",
                InternalModule::CollectorLimpetController => "Collector Limpet Controller",
                InternalModule::DetailedSurfaceScanner => "Detailed Surface Scanner",
                InternalModule::FSDInterdictor => "FSD Interdictor",
                InternalModule::FighterHangar => "Fighter Hangar",
                InternalModule::FrameShiftDrive => "Frame Shift Drive",
                InternalModule::FuelScoop => "Fuel Scoop",
                InternalModule::FuelTank => "Fuel Tank",
                InternalModule::GuardianFSDBooster => "Guardian FSD Booster",
                InternalModule::GuardianModuleReinforcement => "Guardian Module Reinforcement",
                InternalModule::GuardianShieldReinforcement => "Guardian Shield Reinforcement",
                InternalModule::HullReinforcement => "Hull Reinforcement",
                InternalModule::LifeSupport => "Life Support",
                InternalModule::MiningMultiLimpetController => "Mining Limpet Controller",
                InternalModule::ModuleReinforcement => "Module Reinforcement",
                InternalModule::PassengerCabin => "Passenger Cabin",
                InternalModule::PlanetApproachSuite => "Planet Approach Suite",
                InternalModule::PlanetaryVehicleHangar => "Planetary Vehicle Hangar",
                InternalModule::PowerDistributor => "Power Distributor",
                InternalModule::PowerPlant => "Power Plant",
                InternalModule::ProspectorLimpetController => "Prospector Limpet Controller",
                InternalModule::Refinery => "Refinery",
                InternalModule::RepairLimpetController => "Repair Limpet Controller",
                InternalModule::RescueMultiLimpetController => "Rescue Limpet Controller",
                InternalModule::Sensors => "Sensors",
                InternalModule::ShieldCellBank => "Shield Cell Bank",
                InternalModule::ShieldGenerator => "Shield Generator",
                InternalModule::SupercruiseAssist => "Supercruise Assist",
                InternalModule::Thrusters => "Thrusters",
                InternalModule::XenoMultiLimpetController => "Xeno Limpet Controller",
                InternalModule::Armor(armor) => return write!(f, "{}", armor.grade),
            }
        )
    }
}
