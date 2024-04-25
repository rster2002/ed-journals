use std::str::FromStr;
use serde::Deserialize;
use serde_json::Value;
use crate::models::journal_event_kind::shared::ship::ship_module::ship_internal_module::internal_type::InternalType;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum InternalModule {
    #[serde(rename = "hyperdrive")]
    FrameShiftDrive,

    #[serde(rename = "powerplant")]
    PowerPlant,

    #[serde(rename = "guardianmodulereinforcement")]
    GuardianModuleReinforcement,

    #[serde(rename = "hullreinforcement")]
    HullReinforcement,

    #[serde(rename = "dockingcomputer_advanced")]
    AdvancedDockingComputer,

    #[serde(rename = "dronecontrol_collection")]
    CollectorLimpetController,

    #[serde(rename = "cargorack")]
    CargoRack,

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

    #[serde(rename = "guardianfsdbooster")]
    GuardianFSDBooster,

    #[serde(rename = "multidronecontrol_rescue")]
    RescueLimpetController,

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
            | InternalModule::LifeSupport => InternalType::Core,

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
