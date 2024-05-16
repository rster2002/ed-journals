use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ShipType {
    #[serde(rename = "adder")]
    Adder,

    #[serde(rename = "TypeX", alias = "typex")]
    AllianceChieftain,

    #[serde(rename = "typex_2")]
    AllianceCrusader,

    #[serde(rename = "typex_3")]
    AllianceChallenger,

    #[serde(alias = "anaconda")]
    Anaconda,

    #[serde(rename = "Asp", alias = "asp")]
    AspExplorer,

    #[serde(rename = "asp_scout")]
    AspScout,

    #[serde(rename = "belugaliner")]
    BelugaLiner,

    #[serde(alias = "cobramkiii")]
    CobraMkIII,

    #[serde(rename = "cobramkiv")]
    CobraMkIV,

    #[serde(rename = "diamondbackxl")]
    DiamondBackExplorer,

    #[serde(rename = "diamondback")]
    DiamondBackScout,

    #[serde(rename = "dolphin")]
    Dolphin,

    #[serde(rename = "eagle")]
    Eagle,

    #[serde(rename = "federation_dropship_mkii")]
    FederalAssaultShip,

    #[serde(rename = "federation_corvette")]
    FederalCorvette,

    #[serde(rename = "federation_dropship")]
    FederalDropship,

    #[serde(rename = "federation_gunship")]
    FederalGunship,

    #[serde(rename = "FerDeLance", alias = "ferdelance")]
    FerDeLance,

    #[serde(rename = "hauler")]
    Hauler,

    #[serde(rename = "empire_trader")]
    ImperialClipper,

    #[serde(rename = "empire_courier")]
    ImperialCourier,

    #[serde(rename = "Cutter", alias = "cutter")]
    ImperialCutter,

    #[serde(rename = "empire_eagle")]
    ImperialEagle,

    #[serde(rename = "independant_trader")]
    Keelback,

    #[serde(rename = "Krait_MkII", alias = "krait_mkii")]
    KraitMkII,

    #[serde(rename = "Krait_Light", alias = "krait_light")]
    KraitPhantom,

    #[serde(rename = "mamba")]
    Mamba,

    #[serde(rename = "orca")]
    Orca,

    #[serde(alias = "python")]
    Python,

    #[serde(alias = "sidewinder")]
    SideWinder,

    #[serde(rename = "Type7", alias = "type6")]
    Type6Transporter,

    #[serde(rename = "Type7", alias = "type7")]
    Type7Transporter,

    #[serde(rename = "Type9", alias = "type9")]
    Type9Heavy,

    #[serde(rename = "Type9_Military", alias = "type9_military")]
    Type10Defender,

    #[serde(rename = "viper")]
    ViperMkIII,

    #[serde(rename = "viper_mkiv")]
    ViperMkIV,

    #[serde(rename = "vulture")]
    Vulture,

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}

impl FromStr for ShipType {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_value(Value::String(s.to_string()))
    }
}

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
