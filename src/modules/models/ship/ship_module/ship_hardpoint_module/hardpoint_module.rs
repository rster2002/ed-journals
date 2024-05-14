use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde::{Serialize, Deserialize};
use serde_json::Value;

use crate::modules::models::ship::ship_module::ship_hardpoint_module::hardpoint_type::HardpointType;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum HardpointModule {
    #[serde(rename = "beamlaser")]
    BeamLaser,

    #[serde(rename = "beamlaser_heat")]
    RetributorBeamLaser,

    #[serde(rename = "pulselaser")]
    PulseLaser,

    #[serde(rename = "pulselaser_disruptor")]
    PulseDisruptorLaser,

    #[serde(rename = "pulselaserburst")]
    BurstLaser,

    #[serde(rename = "pulselaserburst_scatter")]
    CytoscramblerBurstLaser,

    #[serde(rename = "slugshot")]
    FragmentCannon,

    #[serde(rename = "slugshot_range")]
    PacifierFragCannon,

    #[serde(rename = "cannon")]
    Cannon,

    #[serde(rename = "multicannon")]
    MultiCannon,

    #[serde(rename = "multicannon_advanced")]
    AdvancedMultiCannon,

    #[serde(rename = "multicannon_strong")]
    EnforcerCannon,

    #[serde(rename = "dumbfiremissilerack_lasso")]
    RocketPropelledFSDDisruptor,

    #[serde(rename = "guardian_gausscannon")]
    GuardianGaussCannon,

    #[serde(rename = "guardian_plasmalauncher")]
    GuardianPlasmaCharger,

    #[serde(rename = "guardian_shardcannon")]
    GuardianShardCannon,

    #[serde(rename = "xenoscannermk2_basic")]
    EnhancedXenoScanner,

    #[serde(rename = "crimescanner")]
    KillWarrantScanner,

    #[serde(rename = "cargoscanner")]
    ManifestScanner,

    #[serde(rename = "xenoscanner_advanced")]
    PulseWaveXenoScanner,

    #[serde(rename = "atventdisruptorpylon")]
    NaniteTorpedoPylon,

    #[serde(rename = "plasmashockcannon")]
    ShockCannon,

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

    #[serde(rename = "basicmissilerack")]
    SeekerMissileRack,

    #[serde(rename = "drunkmissilerack")]
    PackHoundMissileRack,

    #[serde(rename = "minelauncher_impulse")]
    ShockMineLauncher,

    #[serde(rename = "minelauncher")]
    MineLauncher,

    #[serde(rename = "advancedtorppylon")]
    TorpedoPylon,

    #[serde(rename = "plasmaaccelerator")]
    PlasmaAccelerator,

    #[serde(rename = "plasmaaccelerator")]
    AdvancedPlasmaAccelerator,

    #[serde(rename = "shieldbooster")]
    ShieldBooster,

    #[serde(rename = "atmulticannon")]
    AXMultiCannon,

    #[serde(rename = "atmulticannon_v2")]
    EnhancedAXMultiCannon,

    #[serde(rename = "atdumbfiremissile")]
    AXMissileRack,

    #[serde(rename = "atdumbfiremissile_v2")]
    EnhancedAXMissileRack,

    #[serde(rename = "flakmortar")]
    RemoteReleaseFlakLauncher,

    #[serde(rename = "flechettelauncher")]
    RemoteReleaseFlechetteLauncher,

    #[serde(rename = "causticmissile")]
    EnzymeMissileRack,

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

    #[serde(rename = "mininglaser_advanced")]
    MiningLanceBeamLaser,

    #[serde(rename = "human_extraction")]
    SubSurfaceExtractionMissile,

    #[serde(rename = "antiunknownshutdown_v2")]
    ThargoidPulseNeutralizer,

    #[serde(rename = "cloudscanner")]
    WakeScanner,

    #[serde(rename = "mrascanner")]
    PulseWaveAnalyzer,

    #[serde(rename = "railgun")]
    RailGun,

    #[serde(rename = "railgun_burst")]
    ImperialHammerRailGun,

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
            | HardpointModule::CausticSinkLauncher
            | HardpointModule::ThargoidPulseNeutralizer
            | HardpointModule::ShutdownFieldNeutralizer
            | HardpointModule::ElectronicCountermeasures
            | HardpointModule::WakeScanner => HardpointType::Utility,

            _ => HardpointType::FullSized,
        }
    }

    pub fn is_full_sized(&self) -> bool {
        matches!(self.hardpoint_type(), HardpointType::FullSized)
    }

    pub fn is_utility(&self) -> bool {
        matches!(self.hardpoint_type(), HardpointType::Utility)
    }
}

impl Display for HardpointModule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                HardpointModule::BeamLaser => "Beam Laser",
                HardpointModule::PulseLaser => "Pulse Laser",
                HardpointModule::MultiCannon => "Multi-Cannon",
                HardpointModule::GuardianGaussCannon => "Guardian Gauss Cannon",
                HardpointModule::EnhancedXenoScanner => "Enhanced Xeno Scanner",
                HardpointModule::PulseWaveXenoScanner => "Pulse Wave Xeno Scanner",
                HardpointModule::NaniteTorpedoPylon => "Nanite Torpedo Pylon",
                HardpointModule::HeatsinkLauncher => "Heatsink Launcher",
                HardpointModule::CausticSinkLauncher => "Caustic Sink Launcher",
                HardpointModule::PointDefenceTurret => "Point Defence Turret",
                HardpointModule::ChaffLauncher => "Chaff Launcher",
                HardpointModule::MissileRack => "Missile Rack",
                HardpointModule::ShieldBooster => "Shield Booster",
                HardpointModule::AXMultiCannon => "Enhanced AX Multi-Cannon",
                HardpointModule::AXMissileRack => "AX Missile Rack",
                HardpointModule::EnhancedAXMissileRack => "Enhanced AX Missile Rack",
                HardpointModule::RemoteReleaseFlakLauncher => "Remote Flak Launcher",
                HardpointModule::ElectronicCountermeasures => "Electronic Countermeasures",
                HardpointModule::ShutdownFieldNeutralizer => "Shutdown Field Neutralizer",
                HardpointModule::AbrasionBlaster => "Abrasion Blaster",
                HardpointModule::SeismicCharge => "Seismic Charge",
                HardpointModule::DisplacementMissile => "Displacement Missile",
                HardpointModule::MiningLaser => "Mining Laser",
                HardpointModule::ThargoidPulseNeutralizer => "Thargoid Pulse Neutralizer",
                HardpointModule::WakeScanner => "Wake Scanner",
                HardpointModule::PulseWaveAnalyzer => "Pulse Wave Analyzer",
                HardpointModule::RailGun => "Rail Gun",

                #[cfg(not(feature = "strict"))]
                HardpointModule::Unknown(unknown) => return write!(f, "Unknown: {}", unknown),
            }
        )
    }
}
