use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde::{Serialize, Deserialize};
use thiserror::Error;
use crate::from_str_deserialize_impl;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum ShipType {
    Adder,
    AllianceChieftain,
    AllianceCrusader,
    AllianceChallenger,
    Anaconda,
    AspExplorer,
    AspScout,
    BelugaLiner,
    CobraMkIII,
    CobraMkIV,
    DiamondBackExplorer,
    DiamondBackScout,
    Dolphin,
    Eagle,
    FederalAssaultShip,
    FederalCorvette,
    FederalDropship,
    FederalGunship,
    FerDeLance,
    Hauler,
    ImperialClipper,
    ImperialCourier,
    ImperialCutter,
    ImperialEagle,
    Keelback,
    KraitMkII,
    KraitPhantom,
    Mamba,
    Orca,
    Python,
    PythonMkII,
    SideWinder,
    Type6Transporter,
    Type7Transporter,
    Type9Heavy,
    Type10Defender,
    ViperMkIII,
    ViperMkIV,
    Vulture,

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}

#[derive(Debug, Error)]
pub enum ShipTypeError {
    #[error("Unknown ship type: '{0}'")]
    UnknownShipType(String),
}

impl FromStr for ShipType {
    type Err = ShipTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let string: &str = &s.to_ascii_lowercase();

        Ok(match string {
            "adder" => ShipType::Adder,
            "typex" => ShipType::AllianceChieftain,
            "typex_2" => ShipType::AllianceCrusader,
            "typex_3" => ShipType::AllianceChallenger,
            "anaconda" => ShipType::Anaconda,
            "asp" => ShipType::AspExplorer,
            "asp_scout" => ShipType::AspScout,
            "belugaliner" => ShipType::BelugaLiner,
            "cobramkiii" => ShipType::CobraMkIII,
            "cobramkiv" => ShipType::CobraMkIV,
            "diamondbackxl" => ShipType::DiamondBackExplorer,
            "diamondback" => ShipType::DiamondBackScout,
            "dolphin" => ShipType::Dolphin,
            "eagle" => ShipType::Eagle,
            "federation_dropship_mkii" => ShipType::FederalAssaultShip,
            "federation_corvette" => ShipType::FederalCorvette,
            "federation_dropship" => ShipType::FederalDropship,
            "federation_gunship" => ShipType::FederalGunship,
            "ferdelance" => ShipType::FerDeLance,
            "hauler" => ShipType::Hauler,
            "empire_trader" => ShipType::ImperialClipper,
            "empire_courier" => ShipType::ImperialCourier,
            "cutter" => ShipType::ImperialCutter,
            "empire_eagle" => ShipType::ImperialEagle,
            "independant_trader" => ShipType::Keelback,
            "krait_mkii" => ShipType::KraitMkII,
            "krait_light" => ShipType::KraitPhantom,
            "mamba" => ShipType::Mamba,
            "orca" => ShipType::Orca,
            "python" => ShipType::Python,
            "sidewinder" => ShipType::SideWinder,
            "type6" => ShipType::Type6Transporter,
            "type7" => ShipType::Type7Transporter,
            "type9" => ShipType::Type9Heavy,
            "type9_military" => ShipType::Type10Defender,
            "viper" => ShipType::ViperMkIII,
            "viper_mkiv" => ShipType::ViperMkIV,
            "vulture" => ShipType::Vulture,

            #[cfg(not(feature = "strict"))]
            _ => ShipType::Unknown(s.to_string()),

            #[cfg(feature = "strict")]
            _ => return Err(ShipTypeError::UnknownShipType(s.to_string())),
        })
    }
}

from_str_deserialize_impl!(ShipType);

impl Display for ShipType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ShipType::Adder => "Adder",
                ShipType::AllianceChallenger => "Alliance Challenger",
                ShipType::AllianceChieftain => "Alliance Chieftain",
                ShipType::AllianceCrusader => "Alliance Crusader",
                ShipType::Anaconda => "Anaconda",
                ShipType::AspExplorer => "Asp Explorer",
                ShipType::AspScout => "Asp Scout",
                ShipType::BelugaLiner => "Beluga Liner",
                ShipType::CobraMkIII => "Cobra Mk III",
                ShipType::CobraMkIV => "Cobra Mk IV",
                ShipType::DiamondBackExplorer => "Diamondback Explorer",
                ShipType::DiamondBackScout => "Diamondback Scout",
                ShipType::Dolphin => "Dolphin",
                ShipType::Eagle => "Eagle",
                ShipType::FederalAssaultShip => "Federal Assault Ship",
                ShipType::FederalCorvette => "Federal Corvette",
                ShipType::FederalDropship => "Federal Dropship",
                ShipType::FederalGunship => "Federal Gunship",
                ShipType::FerDeLance => "Fer-de-Lance",
                ShipType::Hauler => "Hauler",
                ShipType::ImperialClipper => "Imperial Clipper",
                ShipType::ImperialCourier => "Imperial Courier",
                ShipType::ImperialCutter => "Imperial Cutter",
                ShipType::ImperialEagle => "Imperial Eagle",
                ShipType::Keelback => "Keelback",
                ShipType::KraitMkII => "Krait Mk II",
                ShipType::KraitPhantom => "Krait Phantom",
                ShipType::Mamba => "Mamba",
                ShipType::Orca => "Orca",
                ShipType::Python => "Python",
                ShipType::PythonMkII => "Python Mk II",
                ShipType::SideWinder => "Sidewinder",
                ShipType::Type6Transporter => "Type-10 Defender",
                ShipType::Type7Transporter => "Type-6 Transporter",
                ShipType::Type9Heavy => "Type-7 Transporter",
                ShipType::Type10Defender => "Type-9 Heavy",
                ShipType::ViperMkIII => "Viper Mk III",
                ShipType::ViperMkIV => "Viper Mk IV",
                ShipType::Vulture => "Vulture",

                #[cfg(not(feature = "strict"))]
                ShipType::Unknown(unknown) => unknown,
            }
        )
    }
}
