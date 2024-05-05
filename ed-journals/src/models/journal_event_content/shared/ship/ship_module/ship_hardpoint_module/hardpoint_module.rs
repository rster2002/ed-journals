use std::str::FromStr;

use serde::Deserialize;
use serde_json::Value;

use crate::models::journal_event_content::shared::ship::ship_module::ship_hardpoint_module::hardpoint_type::HardpointType;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum HardpointModule {
    #[serde(rename = "beamlaser")]
    BeamLaser,

    #[serde(rename = "pulselaser")]
    PulseLaser,

    #[serde(rename = "multicannon")]
    MultiCannon,

    #[serde(rename = "guardian_gausscannon")]
    GuardianGaussCannon,

    #[serde(rename = "xenoscannermk2_basic")]
    EnhancedXenoScanner,

    #[serde(rename = "xenoscanner_advanced")]
    PulseWaveXenoScanner,

    #[serde(rename = "atventdisruptorpylon")]
    NaniteTorpedoPylon,

    #[serde(rename = "heatsinklauncher")]
    HeatsinkLauncher,

    #[serde(alias = "causticsinklauncher")]
    CausticSinkLauncher,

    #[serde(rename = "plasmapointdefence")]
    PointDefenceTurret,

    #[serde(rename = "chafflauncher")]
    ChaffLauncher,

    #[serde(rename = "dumbfiremissilerack")]
    MissileRack,

    #[serde(rename = "shieldbooster")]
    ShieldBooster,

    #[serde(rename = "atmulticannon")]
    EnhancedAXMultiCannon,

    #[serde(rename = "atdumbfiremissile")]
    AXMissileRack,

    #[serde(rename = "atdumbfiremissile_v2")]
    EnhancedAXMissileRack,

    #[serde(rename = "flakmortar")]
    RemoteFlakLauncher,

    #[serde(rename = "electroniccountermeasure")]
    ElectronicCountermeasures,

    #[serde(rename = "antiunknownshutdown")]
    ShutdownFieldNeutralizer,

    #[serde(rename = "mining_abrblstr")]
    AbrasionBlaster,

    #[serde(rename = "mining_seismchrgwarhd")]
    SeismicCharge,

    #[serde(rename = "mining_subsurfdispmisle")]
    DisplacementMissile,

    #[serde(rename = "mininglaser")]
    MiningLaser,

    #[serde(rename = "antiunknownshutdown_v2")]
    ThargoidPulseNeutralizer,

    #[serde(rename = "cloudscanner")]
    WakeScanner,

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}

impl FromStr for HardpointModule {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_value(Value::String(s.to_ascii_lowercase()))
    }
}

impl HardpointModule {
    pub fn hardpoint_type(&self) -> HardpointType {
        match self {
            HardpointModule::HeatsinkLauncher
            | HardpointModule::PointDefenceTurret
            | HardpointModule::ChaffLauncher
            | HardpointModule::CausticSinkLauncher => HardpointType::Utility,

            _ => HardpointType::FullSized,
        }
    }

    pub fn is_full_sized(&self) -> bool {
        matches!(self.hardpoint_type(), HardpointType::FullSized)
    }

    pub fn is_utility(&self) -> bool {
        matches!(self.hardpoint_type(), HardpointType::Utility)
    }

    // pub fn fixed_class(&self, mounting: &HardpointMounting, size: &HardpointSize) -> Option<ModuleClass> {
    //     match (self, mounting, size) {
    //         (HardpointModule::BeamLaser, )
    //         _ => None,
    //     }
    // }
}
