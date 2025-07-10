use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::modules::ship::{ArmorModule, InternalType};
use crate::ship::ModuleClass;

/// The kind of internal module.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(not(feature = "allow-unknown"), non_exhaustive)]
pub enum InternalModule {
    #[serde(rename = "hyperdrive")]
    FrameShiftDrive,

    #[serde(rename = "hyperdrive_overcharge")]
    FrameShiftDriveOvercharged,

    #[serde(rename = "powerplant")]
    PowerPlant,

    #[serde(rename = "modulereinforcement")]
    ModuleReinforcement,

    #[serde(rename = "guardianmodulereinforcement")]
    GuardianModuleReinforcement,

    #[serde(rename = "guardianhullreinforcement")]
    GuardianHullReinforcement,

    #[serde(rename = "guardianshieldreinforcement")]
    GuardianShieldReinforcement,

    #[serde(rename = "metaalloyhullreinforcement")]
    MetaAlloyHullReinforcement,

    #[serde(rename = "hullreinforcement")]
    HullReinforcement,

    #[serde(rename = "dockingcomputer_standard")]
    StandardDockingComputer,

    #[serde(rename = "dockingcomputer_advanced")]
    AdvancedDockingComputer,

    #[serde(rename = "dronecontrol_collection")]
    CollectorLimpetController,

    #[serde(rename = "dronecontrol_decontamination")]
    DecontaminationLimpetController,

    #[serde(rename = "dronecontrol_repair")]
    RepairLimpetController,

    #[serde(rename = "dronecontrol_prospector")]
    ProspectorLimpetController,

    #[serde(rename = "dronecontrol_fueltransfer")]
    FuelTransferLimpetController,

    #[serde(rename = "dronecontrol_recon")]
    ReconLimpetController,

    #[serde(rename = "dronecontrol_resourcesiphon")]
    HatchBreakerLimpetController,

    #[serde(rename = "dronecontrol_unkvesselresearch")]
    ResearchLimpetController,

    #[serde(rename = "multidronecontrol_mining")]
    MiningMultiLimpetController,

    #[serde(rename = "multidronecontrol_xeno")]
    XenoMultiLimpetController,

    #[serde(rename = "multidronecontrol_rescue")]
    RescueMultiLimpetController,

    #[serde(rename = "multidronecontrol_operations")]
    OperationsMultiLimpetController,

    #[serde(rename = "multidronecontrol_universal")]
    UniversalMultiLimpetController,

    #[serde(rename = "expmodulestabiliser")]
    ExperimentalWeaponStabilizer,

    #[serde(rename = "cargorack")]
    CargoRack,

    #[serde(rename = "corrosionproofcargorack")]
    AntiCorrosionCargoRack,

    #[serde(rename = "supercruiseassist")]
    SupercruiseAssist,

    #[serde(rename = "engine")]
    Thrusters,

    #[serde(rename = "engine_fast")]
    EnhancedPerformanceThrusters,

    #[serde(rename = "fuelscoop")]
    FuelScoop,

    #[serde(rename = "lifesupport")]
    LifeSupport,

    #[serde(rename = "shieldgenerator")]
    ShieldGenerator,

    #[serde(rename = "shieldgenerator_fast")]
    BiWeaveShieldGenerator,

    #[serde(rename = "shieldgenerator_strong")]
    PrismaticShieldGenerator,

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

    #[serde(rename = "planetapproachsuite_advanced", alias = "planetapproachsuite")]
    PlanetApproachSuite,

    #[serde(rename = "refinery")]
    Refinery,

    #[serde(rename = "guardianpowerplant")]
    GuardianHybridPowerPlant,

    #[serde(rename = "guardianpowerdistributor")]
    GuardianHybridPowerDistributor,

    #[serde(rename = "stellarbodydiscoveryscanner_standard")]
    BasicDiscoveryScanner,

    #[serde(rename = "stellarbodydiscoveryscanner_intermediate")]
    IntermediateDiscoveryScanner,

    #[serde(rename = "stellarbodydiscoveryscanner_advanced")]
    AdvancedDiscoveryScanner,

    #[serde(untagged)]
    Armor(ArmorModule),

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    #[serde(untagged)]
    Unknown(String),
}

impl FromStr for InternalModule {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_value(Value::String(s.to_ascii_lowercase()))
    }
}

impl InternalModule {
    /// Returns whether the module is a core internal or an optional internal.
    pub fn internal_type(&self) -> InternalType {
        match self {
            InternalModule::FrameShiftDrive
            | InternalModule::FrameShiftDriveOvercharged
            | InternalModule::PowerPlant
            | InternalModule::GuardianHybridPowerPlant
            | InternalModule::Thrusters
            | InternalModule::PowerDistributor
            | InternalModule::GuardianHybridPowerDistributor
            | InternalModule::LifeSupport
            | InternalModule::PlanetApproachSuite
            | InternalModule::BasicDiscoveryScanner
            | InternalModule::IntermediateDiscoveryScanner
            | InternalModule::AdvancedDiscoveryScanner
            | InternalModule::Sensors
            | InternalModule::Armor(_) => InternalType::Core,

            _ => InternalType::Optional,
        }
    }

    /// Returns true if the module is a core internal like engines or powerplant.
    pub fn is_core(&self) -> bool {
        matches!(self.internal_type(), InternalType::Core)
    }

    /// Returns true if the module is an optional internal like AFMUs and shield generators.
    pub fn is_optional(&self) -> bool {
        matches!(self.internal_type(), InternalType::Optional)
    }

    /// Whether the module is a module that is unlocked through powerplay.
    pub fn is_powerplay_module(&self) -> bool {
        matches!(self, InternalModule::PrismaticShieldGenerator)
    }

    /// Whether the module is a module that is unlocked using guardian parts at a guardian
    /// technology broker.
    pub fn is_guardian_module(&self) -> bool {
        matches!(
            self,
            InternalModule::GuardianHybridPowerDistributor
                | InternalModule::GuardianHybridPowerPlant
                | InternalModule::GuardianModuleReinforcement
                | InternalModule::GuardianFSDBooster
                | InternalModule::GuardianHullReinforcement
                | InternalModule::GuardianShieldReinforcement
        )
    }

    pub(crate) fn special_grades(
        &self,
        size: u8,
        grade: Option<&ModuleClass>,
    ) -> Option<ModuleClass> {
        Some(match (self, size, grade) {
            (InternalModule::IntermediateDiscoveryScanner, _, _) => ModuleClass::D,
            (InternalModule::AdvancedDiscoveryScanner, _, _) => ModuleClass::C,
            (InternalModule::PlanetaryVehicleHangar, _, Some(ModuleClass::E)) => ModuleClass::H,
            (InternalModule::PlanetaryVehicleHangar, _, Some(ModuleClass::D)) => ModuleClass::G,
            (InternalModule::PlanetApproachSuite, _, _) => ModuleClass::I,
            (InternalModule::AntiCorrosionCargoRack, _, Some(ModuleClass::D)) => ModuleClass::F,
            (InternalModule::FighterHangar, _, _) => ModuleClass::D,
            (InternalModule::GuardianFSDBooster, _, _) => ModuleClass::H,
            (InternalModule::GuardianHybridPowerDistributor, _, _) => ModuleClass::A,
            (InternalModule::GuardianHybridPowerPlant, _, _) => ModuleClass::A,
            (InternalModule::ExperimentalWeaponStabilizer, _, _) => ModuleClass::F,
            (_, _, _) => return None,
        })
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
                InternalModule::FrameShiftDriveOvercharged => "Frame Shift Drive (SCO)",
                InternalModule::GuardianHullReinforcement => "Guardian Hull Reinforcement Package",
                InternalModule::MetaAlloyHullReinforcement =>
                    "Meta Alloy Hull Reinforcement Package",
                InternalModule::StandardDockingComputer => "Standard Docking Computer",
                InternalModule::DecontaminationLimpetController =>
                    "Decontamination Limpet Controller",
                InternalModule::FuelTransferLimpetController => "Fuel Transfer Limpet Controller",
                InternalModule::ReconLimpetController => "Recon Limpet Controller",
                InternalModule::HatchBreakerLimpetController => "Hatch Breaker Limpet Controller",
                InternalModule::OperationsMultiLimpetController =>
                    "Operations Multi Limpet Controller",
                InternalModule::ExperimentalWeaponStabilizer => "Experimental Weapon Stabilizer",
                InternalModule::PrismaticShieldGenerator => "Prismatic Shield Generator",
                InternalModule::GuardianHybridPowerPlant => "Guardian Hybrid Power Plant",
                InternalModule::GuardianHybridPowerDistributor =>
                    "Guardian Hybrid Power Distributor",
                InternalModule::ResearchLimpetController => "Research Limpet Controller",
                InternalModule::UniversalMultiLimpetController =>
                    "Universal Multi Limpet Controller",
                InternalModule::EnhancedPerformanceThrusters => "Enhanced Performance Thrusters",
                InternalModule::BasicDiscoveryScanner => "Basic Discovery Scanner",
                InternalModule::IntermediateDiscoveryScanner => "Intermediate Discovery Scanner",
                InternalModule::AdvancedDiscoveryScanner => "Advanced Discovery Scanner",

                #[cfg(feature = "allow-unknown")]
                InternalModule::Unknown(unknown) =>
                    return write!(f, "Unknown module: {unknown}"),
            }
        )
    }
}
