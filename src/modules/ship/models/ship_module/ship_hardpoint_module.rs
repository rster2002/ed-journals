use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::{Captures, Regex};
use serde::Serialize;
use thiserror::Error;

use crate::from_str_deserialize_impl;
use crate::modules::ship::{
    HardpointModule, HardpointMounting, HardpointSize, HardpointSizeError, HardpointType,
    ModuleClass, ModuleClassError,
};

pub mod hardpoint_module;
pub mod hardpoint_mounting;
pub mod hardpoint_size;
pub mod hardpoint_type;

/// Contains all the data related to hardpoints. This model is used for both full-sized hardpoint
/// modules and utility modules. Utility modules use [HardpointSize::Tiny] and full-sized modules
/// use the other sizes.
#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct ShipHardpointModule {
    /// The kind internal module, which can both be full-sized hardpoints and utility modules.
    pub module: HardpointModule,

    /// The mounting type the module used which dictate how aiming works. Utility modules always
    /// use [HardpointMounting::Turreted].
    pub mounting: HardpointMounting,

    /// The size of the module.
    pub size: HardpointSize,

    /// The class of the module.
    pub class: ModuleClass,
}

#[derive(Debug, Error)]
pub enum ShipHardpointModuleError {
    #[error("Failed to parse hardpoint module: {0}")]
    FailedToParseHardpointModule(#[source] serde_json::Error),

    #[error("Failed to parse hardpoint mounting: {0}")]
    FailedToParseHardpointMounting(#[source] serde_json::Error),

    #[error("Failed to parse hardpoint size: {0}")]
    FailedToParseHardpointSize(#[from] HardpointSizeError),

    #[error("Failed to parse class number: {0}")]
    FailedToParseClassNumber(#[source] ParseIntError),

    #[error("Mounting type cannot be missing when the module size is not 'tiny'")]
    MissingMounting,

    #[error(transparent)]
    ModuleClassError(#[from] ModuleClassError),

    #[error("Failed to parse ship hardpoint module: '{0}'")]
    FailedToParse(String),
}

lazy_static! {
    static ref GRADED_HARDPOINT_MODULE_REGEX: Regex = Regex::new(r#"^\$?[hH]pt_(\w+?)_[sS]ize(\d+)_[cC]lass(\d+)(_[nN]ame;)?$"#).unwrap();
    static ref HARDPOINT_MODULE_REGEX: Regex = Regex::new(r#"^\$?[hH]pt_(\w+?)(_([gG]imbal|[fF]ixed|[tT]urret))?_([tT]iny|[sS]mall|[mM]edium|[lL]arge|[hH]uge)(_[a-zA-Z0-9_]+?)?(_[nN]ame;)?$"#).unwrap();
}

impl FromStr for ShipHardpointModule {
    type Err = ShipHardpointModuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(captures) = GRADED_HARDPOINT_MODULE_REGEX.captures(s) {
            return Self::from_graded_hardpoint_captures(captures);
        }

        if let Some(captures) = HARDPOINT_MODULE_REGEX.captures(s) {
            return Self::from_hardpoint_captures(captures);
        };

        Err(ShipHardpointModuleError::FailedToParse(s.to_string()))
    }
}

impl ShipHardpointModule {
    /// Returns whether the module is a full-sized hardpoint or a utility module.
    pub fn hardpoint_type(&self) -> HardpointType {
        self.module.hardpoint_type()
    }

    /// Whether the module is a full-sized hardpoint module.
    pub fn is_full_sized_module(&self) -> bool {
        self.module.is_full_sized()
    }

    /// Whether the module is a utility module.
    pub fn is_utility_module(&self) -> bool {
        self.module.is_utility()
    }

    /// Whether the module is a module that is unlocked through powerplay.
    pub fn is_powerplay_module(&self) -> bool {
        self.module.is_powerplay_module()
    }

    /// Whether the module is a module that is unlocked using guardian parts at a guardian
    /// technology broker.
    pub fn is_guardian_module(&self) -> bool {
        self.module.is_guardian_module()
    }

    fn from_graded_hardpoint_captures(
        captures: Captures,
    ) -> Result<ShipHardpointModule, ShipHardpointModuleError> {
        let module = captures
            .get(1)
            .expect("Should have been captured already")
            .as_str()
            .parse()
            .map_err(ShipHardpointModuleError::FailedToParseHardpointModule)?;

        let class = captures
            .get(3)
            .expect("Should have been captured already")
            .as_str()
            .parse::<u8>()
            .map_err(ShipHardpointModuleError::FailedToParseClassNumber)?
            .try_into()?;

        Ok(ShipHardpointModule {
            module,
            mounting: HardpointMounting::Turreted,
            size: HardpointSize::Tiny,
            class,
        })
    }

    fn from_hardpoint_captures(
        captures: Captures,
    ) -> Result<ShipHardpointModule, ShipHardpointModuleError> {
        let module = captures
            .get(1)
            .expect("Should have been captured already")
            .as_str();

        let module_suffix = captures
            .get(5)
            .map(|capture| capture.as_str())
            .unwrap_or_default();

        let mut module = format!("{}{}", module, module_suffix)
            .parse()
            .map_err(ShipHardpointModuleError::FailedToParseHardpointModule)?;

        let size = captures
            .get(4)
            .expect("Should have been captured already")
            .as_str()
            .parse()?;

        // Sometimes the mounting is missing when the size is 'tiny'
        let mounting = match captures.get(3) {
            Some(capture) => capture
                .as_str()
                .parse()
                .map_err(ShipHardpointModuleError::FailedToParseHardpointMounting),
            None => {
                if matches!(size, HardpointSize::Tiny) {
                    Ok(HardpointMounting::Turreted)
                } else {
                    Err(ShipHardpointModuleError::MissingMounting)
                }
            }
        }?;

        // I once again ask: Why Frontier?!
        if module == HardpointModule::AXMultiCannon && mounting == HardpointMounting::Gimballed {
            module = HardpointModule::EnhancedAXMultiCannon;
        }

        let class = Self::get_module_class(&module, &mounting, &size);

        Ok(ShipHardpointModule {
            module,
            mounting,
            class,
            size,
        })
    }

    #[rustfmt::skip]
    fn get_module_class(
        module: &HardpointModule,
        mounting: &HardpointMounting,
        size: &HardpointSize,
    ) -> ModuleClass {
        match (&module, &mounting, &size) {
            // Pulse lasers
            (HardpointModule::PulseLaser, HardpointMounting::Fixed, HardpointSize::Small) => ModuleClass::F,
            (HardpointModule::PulseLaser, HardpointMounting::Gimballed, HardpointSize::Small) => ModuleClass::G,
            (HardpointModule::PulseLaser, HardpointMounting::Turreted, HardpointSize::Small) => ModuleClass::G,
            (HardpointModule::PulseLaser, HardpointMounting::Fixed, HardpointSize::Medium) => ModuleClass::E,
            (HardpointModule::PulseLaser, HardpointMounting::Gimballed, HardpointSize::Medium) => ModuleClass::F,
            (HardpointModule::PulseLaser, HardpointMounting::Turreted, HardpointSize::Medium) => ModuleClass::F,
            (HardpointModule::PulseDisruptorLaser, _, _) => ModuleClass::E,
            (HardpointModule::PulseLaser, HardpointMounting::Fixed, HardpointSize::Large) => ModuleClass::D,
            (HardpointModule::PulseLaser, HardpointMounting::Gimballed, HardpointSize::Large) => ModuleClass::E,
            (HardpointModule::PulseLaser, HardpointMounting::Turreted, HardpointSize::Large) => ModuleClass::F,
            (HardpointModule::PulseLaser, _, HardpointSize::Huge) => ModuleClass::A,

            // Burst lasers
            (HardpointModule::BurstLaser, HardpointMounting::Fixed, HardpointSize::Small) => ModuleClass::F,
            (HardpointModule::BurstLaser, HardpointMounting::Gimballed, HardpointSize::Small) => ModuleClass::G,
            (HardpointModule::BurstLaser, HardpointMounting::Turreted, HardpointSize::Small) => ModuleClass::G,
            (HardpointModule::CytoscramblerBurstLaser, _, _) => ModuleClass::F,
            (HardpointModule::BurstLaser, HardpointMounting::Fixed, HardpointSize::Medium) => ModuleClass::E,
            (HardpointModule::BurstLaser, HardpointMounting::Gimballed, HardpointSize::Medium) => ModuleClass::F,
            (HardpointModule::BurstLaser, HardpointMounting::Turreted, HardpointSize::Medium) => ModuleClass::F,
            (HardpointModule::BurstLaser, HardpointMounting::Fixed, HardpointSize::Large) => ModuleClass::D,
            (HardpointModule::BurstLaser, HardpointMounting::Gimballed, HardpointSize::Large) => ModuleClass::E,
            (HardpointModule::BurstLaser, HardpointMounting::Turreted, HardpointSize::Large) => ModuleClass::E,
            (HardpointModule::BurstLaser, _, HardpointSize::Huge) => ModuleClass::E,

            // Cannons
            (HardpointModule::Cannon, HardpointMounting::Fixed, HardpointSize::Small) => ModuleClass::D,
            (HardpointModule::Cannon, HardpointMounting::Gimballed, HardpointSize::Small) => ModuleClass::E,
            (HardpointModule::Cannon, HardpointMounting::Turreted, HardpointSize::Small) => ModuleClass::F,
            (HardpointModule::Cannon, HardpointMounting::Fixed, HardpointSize::Medium) => ModuleClass::D,
            (HardpointModule::Cannon, HardpointMounting::Gimballed, HardpointSize::Medium) => ModuleClass::D,
            (HardpointModule::Cannon, HardpointMounting::Turreted, HardpointSize::Medium) => ModuleClass::E,
            (HardpointModule::Cannon, HardpointMounting::Fixed, HardpointSize::Large) => ModuleClass::C,
            (HardpointModule::Cannon, HardpointMounting::Gimballed, HardpointSize::Large) => ModuleClass::C,
            (HardpointModule::Cannon, HardpointMounting::Turreted, HardpointSize::Large) => ModuleClass::D,
            (HardpointModule::Cannon, _, HardpointSize::Huge) => ModuleClass::B,

            // Experimental
            (HardpointModule::RemoteReleaseFlakLauncher, _, HardpointSize::Medium) => ModuleClass::B,
            (HardpointModule::RemoteReleaseFlechetteLauncher, _, HardpointSize::Medium) => ModuleClass::B,
            (HardpointModule::EnzymeMissileRack, _, HardpointSize::Medium) => ModuleClass::B,

            // AX missile racks
            (HardpointModule::AXMissileRack, HardpointMounting::Fixed, HardpointSize::Medium) => ModuleClass::E,
            (HardpointModule::AXMissileRack, HardpointMounting::Turreted, HardpointSize::Medium) => ModuleClass::F,
            (HardpointModule::EnhancedAXMissileRack, HardpointMounting::Fixed, HardpointSize::Medium) => ModuleClass::D,
            (HardpointModule::EnhancedAXMissileRack, HardpointMounting::Turreted, HardpointSize::Medium) => ModuleClass::E,
            (HardpointModule::AXMissileRack, HardpointMounting::Fixed, HardpointSize::Large) => ModuleClass::C,
            (HardpointModule::AXMissileRack, HardpointMounting::Turreted, HardpointSize::Large) => ModuleClass::E,
            (HardpointModule::EnhancedAXMissileRack, HardpointMounting::Fixed, HardpointSize::Large) => ModuleClass::B,
            (HardpointModule::EnhancedAXMissileRack, HardpointMounting::Turreted, HardpointSize::Large) => ModuleClass::D,

            // AX multi-cannons
            (HardpointModule::AXMultiCannon, HardpointMounting::Fixed, HardpointSize::Medium) => ModuleClass::E,
            (HardpointModule::AXMultiCannon, HardpointMounting::Turreted, HardpointSize::Medium) => ModuleClass::F,
            (HardpointModule::EnhancedAXMultiCannon, HardpointMounting::Fixed, HardpointSize::Medium) => ModuleClass::D,
            (HardpointModule::EnhancedAXMultiCannon, HardpointMounting::Gimballed, HardpointSize::Medium) => ModuleClass::E,
            (HardpointModule::EnhancedAXMultiCannon, HardpointMounting::Turreted, HardpointSize::Medium) => ModuleClass::E,
            (HardpointModule::AXMultiCannon, HardpointMounting::Fixed, HardpointSize::Large) => ModuleClass::C,
            (HardpointModule::AXMultiCannon, HardpointMounting::Turreted, HardpointSize::Large) => ModuleClass::E,
            (HardpointModule::EnhancedAXMultiCannon, HardpointMounting::Fixed, HardpointSize::Large) => ModuleClass::B,
            (HardpointModule::EnhancedAXMultiCannon, HardpointMounting::Gimballed, HardpointSize::Large) => ModuleClass::C,
            (HardpointModule::EnhancedAXMultiCannon, HardpointMounting::Turreted, HardpointSize::Large) => ModuleClass::D,

            // Guardian gauss cannons
            (HardpointModule::GuardianGaussCannon, _, HardpointSize::Small) => ModuleClass::D,
            (HardpointModule::GuardianGaussCannon, _, HardpointSize::Medium) => ModuleClass::B,

            // Guardian plasma chargers
            (HardpointModule::GuardianPlasmaCharger, HardpointMounting::Fixed, HardpointSize::Small) => ModuleClass::D,
            (HardpointModule::GuardianPlasmaCharger, HardpointMounting::Turreted, HardpointSize::Small) => ModuleClass::F,
            (HardpointModule::GuardianPlasmaCharger, HardpointMounting::Fixed, HardpointSize::Medium) => ModuleClass::B,
            (HardpointModule::GuardianPlasmaCharger, HardpointMounting::Turreted, HardpointSize::Medium) => ModuleClass::E,
            (HardpointModule::GuardianPlasmaCharger, HardpointMounting::Fixed, HardpointSize::Large) => ModuleClass::C,
            (HardpointModule::GuardianPlasmaCharger, HardpointMounting::Turreted, HardpointSize::Large) => ModuleClass::D,

            // Guardian shard cannons
            (HardpointModule::GuardianShardCannon, HardpointMounting::Fixed, HardpointSize::Small) => ModuleClass::D,
            (HardpointModule::GuardianShardCannon, HardpointMounting::Turreted, HardpointSize::Small) => ModuleClass::F,
            (HardpointModule::GuardianShardCannon, HardpointMounting::Fixed, HardpointSize::Medium) => ModuleClass::A,
            (HardpointModule::GuardianShardCannon, HardpointMounting::Turreted, HardpointSize::Medium) => ModuleClass::A,
            (HardpointModule::GuardianShardCannon, HardpointMounting::Fixed, HardpointSize::Large) => ModuleClass::C,
            (HardpointModule::GuardianShardCannon, HardpointMounting::Turreted, HardpointSize::Large) => ModuleClass::D,

            // Nanite torpedoes
            (HardpointModule::NaniteTorpedoPylon, _, HardpointSize::Medium) => ModuleClass::I,
            (HardpointModule::NaniteTorpedoPylon, _, HardpointSize::Large) => ModuleClass::I,

            // Shock cannons
            (HardpointModule::ShockCannon, HardpointMounting::Fixed, HardpointSize::Small) => ModuleClass::D,
            (HardpointModule::ShockCannon, HardpointMounting::Gimballed, HardpointSize::Small) => ModuleClass::E,
            (HardpointModule::ShockCannon, HardpointMounting::Turreted, HardpointSize::Small) => ModuleClass::F,
            (HardpointModule::ShockCannon, HardpointMounting::Fixed, HardpointSize::Medium) => ModuleClass::D,
            (HardpointModule::ShockCannon, HardpointMounting::Gimballed, HardpointSize::Medium) => ModuleClass::D,
            (HardpointModule::ShockCannon, HardpointMounting::Turreted, HardpointSize::Medium) => ModuleClass::E,
            (HardpointModule::ShockCannon, HardpointMounting::Fixed, HardpointSize::Large) => ModuleClass::C,
            (HardpointModule::ShockCannon, HardpointMounting::Gimballed, HardpointSize::Large) => ModuleClass::C,
            (HardpointModule::ShockCannon, HardpointMounting::Turreted, HardpointSize::Large) => ModuleClass::D,

            //  Fragment cannons
            (HardpointModule::FragmentCannon, _, HardpointSize::Small) => ModuleClass::E,
            (HardpointModule::FragmentCannon, HardpointMounting::Fixed, HardpointSize::Medium) => ModuleClass::A,
            (HardpointModule::FragmentCannon, HardpointMounting::Gimballed, HardpointSize::Medium) => ModuleClass::D,
            (HardpointModule::FragmentCannon, HardpointMounting::Turreted, HardpointSize::Medium) => ModuleClass::D,
            (HardpointModule::FragmentCannon, _, HardpointSize::Large) => ModuleClass::C,
            (HardpointModule::PacifierFragCannon, _, _) => ModuleClass::C,

            // Mines
            (HardpointModule::MineLauncher, _, HardpointSize::Small) => ModuleClass::I,
            (HardpointModule::ShockMineLauncher, _, HardpointSize::Small) => ModuleClass::I,
            (HardpointModule::MineLauncher, _, HardpointSize::Medium) => ModuleClass::I,

            // Mining lasers
            (HardpointModule::MiningLaser, HardpointMounting::Fixed, HardpointSize::Small) => ModuleClass::D,
            (HardpointModule::MiningLaser, HardpointMounting::Turreted, HardpointSize::Small) => ModuleClass::D,
            (HardpointModule::AbrasionBlaster, HardpointMounting::Fixed, HardpointSize::Small) => ModuleClass::D,
            (HardpointModule::AbrasionBlaster, HardpointMounting::Turreted, HardpointSize::Small) => ModuleClass::D,
            (HardpointModule::MiningLanceBeamLaser, _, _) => ModuleClass::D,
            (HardpointModule::MiningLaser, HardpointMounting::Fixed, HardpointSize::Medium) => ModuleClass::D,
            (HardpointModule::MiningLaser, HardpointMounting::Turreted, HardpointSize::Medium) => ModuleClass::D,

            // Mining launchers
            (HardpointModule::DisplacementMissile, HardpointMounting::Fixed, HardpointSize::Small) => ModuleClass::B,
            (HardpointModule::DisplacementMissile, HardpointMounting::Turreted, HardpointSize::Small) => ModuleClass::B,
            (HardpointModule::DisplacementMissile, HardpointMounting::Fixed, HardpointSize::Medium) => ModuleClass::B,
            (HardpointModule::DisplacementMissile, HardpointMounting::Turreted, HardpointSize::Medium) => ModuleClass::B,
            (HardpointModule::SeismicCharge, HardpointMounting::Fixed, HardpointSize::Medium) => ModuleClass::B,
            (HardpointModule::SeismicCharge, HardpointMounting::Turreted, HardpointSize::Medium) => ModuleClass::B,
            (HardpointModule::SubSurfaceExtractionMissile, _, _) => ModuleClass::B,

            // Missiles
            (HardpointModule::MissileRack, _, HardpointSize::Small) => ModuleClass::B,
            (HardpointModule::SeekerMissileRack, _, HardpointSize::Small) => ModuleClass::B,
            (HardpointModule::AdvancedMissileRack, _, HardpointSize::Small) => ModuleClass::B,
            (HardpointModule::MissileRack, _, HardpointSize::Medium) => ModuleClass::B,
            (HardpointModule::SeekerMissileRack, _, HardpointSize::Medium) => ModuleClass::B,
            (HardpointModule::AdvancedMissileRack, _, HardpointSize::Medium) => ModuleClass::B,
            (HardpointModule::PackHoundMissileRack, _, _) => ModuleClass::B,
            (HardpointModule::RocketPropelledFSDDisruptor, _, _) => ModuleClass::B,
            (HardpointModule::MissileRack, _, HardpointSize::Large) => ModuleClass::A,
            (HardpointModule::SeekerMissileRack, _, HardpointSize::Large) => ModuleClass::A,

            // Multi-cannons
            (HardpointModule::MultiCannon, HardpointMounting::Fixed, HardpointSize::Small) => ModuleClass::F,
            (HardpointModule::MultiCannon, HardpointMounting::Gimballed, HardpointSize::Small) => ModuleClass::G,
            (HardpointModule::MultiCannon, HardpointMounting::Turreted, HardpointSize::Small) => ModuleClass::G,
            (HardpointModule::AdvancedMultiCannon, _, HardpointSize::Small) => ModuleClass::F,
            (HardpointModule::EnforcerCannon, _, _) => ModuleClass::F,
            (HardpointModule::MultiCannon, HardpointMounting::Fixed, HardpointSize::Medium) => ModuleClass::E,
            (HardpointModule::MultiCannon, HardpointMounting::Gimballed, HardpointSize::Medium) => ModuleClass::F,
            (HardpointModule::MultiCannon, HardpointMounting::Turreted, HardpointSize::Medium) => ModuleClass::F,
            (HardpointModule::AdvancedMultiCannon, _, HardpointSize::Medium) => ModuleClass::E,
            (HardpointModule::MultiCannon, HardpointMounting::Fixed, HardpointSize::Large) => ModuleClass::C,
            (HardpointModule::MultiCannon, HardpointMounting::Gimballed, HardpointSize::Large) => ModuleClass::C,
            (HardpointModule::MultiCannon, HardpointMounting::Turreted, HardpointSize::Large) => ModuleClass::E,
            (HardpointModule::MultiCannon, _, _) => ModuleClass::A,

            // Plasma accelerators
            (HardpointModule::PlasmaAccelerator, _, HardpointSize::Medium) => ModuleClass::C,
            (HardpointModule::PlasmaAccelerator, _, HardpointSize::Large) => ModuleClass::B,
            (HardpointModule::AdvancedPlasmaAccelerator, _, _) => ModuleClass::B,
            (HardpointModule::PlasmaAccelerator, _, HardpointSize::Huge) => ModuleClass::A,

            // Beam lasers
            (HardpointModule::BeamLaser, HardpointMounting::Fixed | HardpointMounting::Gimballed, HardpointSize::Small) => ModuleClass::E,
            (HardpointModule::BeamLaser, HardpointMounting::Turreted, HardpointSize::Small) => ModuleClass::F,
            (HardpointModule::RetributorBeamLaser, _, _) => ModuleClass::E,
            (HardpointModule::BeamLaser, HardpointMounting::Fixed, HardpointSize::Medium) => ModuleClass::D,
            (HardpointModule::BeamLaser, HardpointMounting::Gimballed, HardpointSize::Medium) => ModuleClass::D,
            (HardpointModule::BeamLaser, HardpointMounting::Turreted, HardpointSize::Medium) => ModuleClass::E,
            (HardpointModule::BeamLaser, HardpointMounting::Fixed, HardpointSize::Large) => ModuleClass::C,
            (HardpointModule::BeamLaser, HardpointMounting::Gimballed, HardpointSize::Large) => ModuleClass::C,
            (HardpointModule::BeamLaser, HardpointMounting::Turreted, HardpointSize::Large) => ModuleClass::D,
            (HardpointModule::BeamLaser, _, HardpointSize::Huge) => ModuleClass::A,

            // Rail guns
            (HardpointModule::RailGun, _, HardpointSize::Small) => ModuleClass::D,
            (HardpointModule::RailGun, _, HardpointSize::Medium) => ModuleClass::B,
            (HardpointModule::ImperialHammerRailGun, _, _) => ModuleClass::B,

            // Torpedoes
            (HardpointModule::TorpedoPylon, _, HardpointSize::Small) => ModuleClass::I,
            (HardpointModule::TorpedoPylon, _, HardpointSize::Medium) => ModuleClass::I,
            (HardpointModule::TorpedoPylon, _, HardpointSize::Large) => ModuleClass::I,

            // Chaff launchers
            (HardpointModule::ChaffLauncher, _, _) => ModuleClass::I,

            // ECMS
            (HardpointModule::ElectronicCountermeasures, _, _) => ModuleClass::F,

            // Experimental utilities
            (HardpointModule::EnhancedXenoScanner, _, _) => ModuleClass::C,
            (HardpointModule::PulseWaveXenoScanner, _, _) => ModuleClass::C,
            (HardpointModule::ShutdownFieldNeutralizer, _, _) => ModuleClass::F,
            (HardpointModule::ThargoidPulseNeutralizer, _, _) => ModuleClass::E,
            (HardpointModule::XenoScanner, _, _) => ModuleClass::E,

            // Heat sink launchers
            (HardpointModule::CausticSinkLauncher, _, _) => ModuleClass::I,
            (HardpointModule::HeatsinkLauncher, _, _) => ModuleClass::I,

            // Point defence
            (HardpointModule::PointDefenceTurret, _, _) => ModuleClass::I,

            _ => ModuleClass::E,
        }
    }
}

from_str_deserialize_impl!(ShipHardpointModule);

impl Display for ShipHardpointModule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let HardpointSize::Tiny = self.size {
            return write!(f, "{}{} {}", self.size.size_nr(), self.class, self.module);
        }

        write!(
            f,
            "{}{}/{} {}",
            self.size.size_nr(),
            self.class,
            self.mounting,
            self.module
        )
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::modules::ship::{
        HardpointModule, HardpointMounting, HardpointSize, ModuleClass, ShipHardpointModule,
    };

    #[test]
    fn ship_hardpoint_module_test_cases_all_pass() {
        let test_cases = [
            (
                "$hpt_beamlaser_gimbal_medium_name;",
                ShipHardpointModule {
                    module: HardpointModule::BeamLaser,
                    mounting: HardpointMounting::Gimballed,
                    size: HardpointSize::Medium,
                    class: ModuleClass::D,
                },
            ),
            (
                "$hpt_heatsinklauncher_turret_tiny_name;",
                ShipHardpointModule {
                    module: HardpointModule::HeatsinkLauncher,
                    mounting: HardpointMounting::Turreted,
                    size: HardpointSize::Tiny,
                    class: ModuleClass::I,
                },
            ),
            (
                "Hpt_CausticSinkLauncher_Turret_Tiny",
                ShipHardpointModule {
                    module: HardpointModule::CausticSinkLauncher,
                    mounting: HardpointMounting::Turreted,
                    size: HardpointSize::Tiny,
                    class: ModuleClass::I,
                },
            ),
            (
                "$hpt_chafflauncher_tiny_name;",
                ShipHardpointModule {
                    module: HardpointModule::ChaffLauncher,
                    mounting: HardpointMounting::Turreted,
                    size: HardpointSize::Tiny,
                    class: ModuleClass::I,
                },
            ),
            (
                "$hpt_shieldbooster_size0_class5_name;",
                ShipHardpointModule {
                    module: HardpointModule::ShieldBooster,
                    mounting: HardpointMounting::Turreted,
                    size: HardpointSize::Tiny,
                    class: ModuleClass::A,
                },
            ),
            (
                "Hpt_Guardian_GaussCannon_Fixed_Medium",
                ShipHardpointModule {
                    module: HardpointModule::GuardianGaussCannon,
                    mounting: HardpointMounting::Fixed,
                    size: HardpointSize::Medium,
                    class: ModuleClass::B,
                },
            ),
            (
                "Hpt_CargoScanner_Size0_Class1",
                ShipHardpointModule {
                    module: HardpointModule::ManifestScanner,
                    mounting: HardpointMounting::Turreted,
                    size: HardpointSize::Tiny,
                    class: ModuleClass::E,
                },
            ),
            (
                "Hpt_PlasmaAccelerator_Fixed_Large_Advanced",
                ShipHardpointModule {
                    module: HardpointModule::AdvancedPlasmaAccelerator,
                    mounting: HardpointMounting::Fixed,
                    size: HardpointSize::Large,
                    class: ModuleClass::B,
                },
            ),
        ];

        for (case, expected) in test_cases {
            let result = ShipHardpointModule::from_str(case);

            dbg!(&result);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), expected);
        }
    }

    #[test]
    fn incorrect_combination_of_missing_mounting_type_and_size_is_rejected() {
        let result = ShipHardpointModule::from_str("$hpt_chafflauncher_mediun_name;");
        assert!(result.is_err());
    }
}
