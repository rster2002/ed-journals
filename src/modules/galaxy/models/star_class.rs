use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde::Serialize;
use thiserror::Error;

use crate::from_str_deserialize_impl;

#[derive(Debug, Serialize, Clone, PartialEq, Hash, Eq)]
pub enum StarClass {
    // Main sequence
    O,
    B,
    A,
    F,
    G,
    K,
    M,
    L,
    T,
    Y,

    // Proto stars
    TTS,
    Ae,
    Be,
    AeBe,

    // Wolf-Rayet
    W,
    WN,
    WNC,
    WC,
    WO,

    // Carbon stars
    CS,
    C,
    CN,
    CJ,
    CH,
    CHd,

    // White dwarfs
    D,
    DA,
    DAB,
    DAO,
    DAZ,
    DAV,
    DB,
    DBZ,
    DBV,
    DO,
    DOV,
    DQ,
    DC,
    DCV,
    DX,

    N, // Neutron star
    H, // Black hole
    X, // Exotic
    SupermassiveBlackHole,

    ABlueWhiteSuperGiant, // A_BlueWhiteSuperGiant
    BBlueWhiteSuperGiant, // B_BlueWhiteSuperGiant
    FWhiteSuperGiant,     // F_WhiteSuperGiant
    GWhiteSuperGiant,     // G_WhiteSuperGiant
    MRedSuperGiant,       // M_RedSuperGiant
    MRedGiant,            // M_RedGiant
    KOrangeGiant,         // K_OrangeGiant
    RoguePlanet,
    Nebula,
    StellarRemnantNebula,

    // No category
    MS,
    S,
}

#[derive(Debug, Error)]
pub enum StarClassError {
    #[error("Unknown star class: '{0}'")]
    UnknownStarClass(String),
}

impl FromStr for StarClass {
    type Err = StarClassError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "O" | "o" => StarClass::O,
            "B" | "b" => StarClass::B,
            "A" | "a" => StarClass::A,
            "F" | "f" => StarClass::F,
            "G" | "g" => StarClass::G,
            "K" | "k" => StarClass::K,
            "M" | "m" => StarClass::M,
            "L" | "l" => StarClass::L,
            "T" | "t" => StarClass::T,
            "Y" | "y" => StarClass::Y,

            "TTS" | "tts" => StarClass::TTS,
            "Ae" | "ae" => StarClass::Ae,
            "Be" | "be" => StarClass::Be,
            "AeBe" | "aebe" => StarClass::AeBe,

            "W" | "w" => StarClass::W,
            "WN" | "wn" => StarClass::WN,
            "WNC" | "wnc" => StarClass::WNC,
            "WC" | "wc" => StarClass::WC,
            "WO" | "wo" => StarClass::WO,

            "CS" | "cs" => StarClass::CS,
            "C" | "c" => StarClass::C,
            "CN" | "cn" => StarClass::CN,
            "CJ" | "cj" => StarClass::CJ,
            "CH" | "ch" => StarClass::CH,
            "CHd" | "chd" => StarClass::CHd,

            "D" | "d" => StarClass::D,
            "DA" | "da" => StarClass::DA,
            "DAB" | "dab" => StarClass::DAB,
            "DAO" | "dao" => StarClass::DAO,
            "DAZ" | "daz" => StarClass::DAZ,
            "DAV" | "dav" => StarClass::DAV,
            "DB" | "db" => StarClass::DB,
            "DBZ" | "dbz" => StarClass::DBZ,
            "DBV" | "dbv" => StarClass::DBV,
            "DO" | "do" => StarClass::DO,
            "DOV" | "dov" => StarClass::DOV,
            "DQ" | "dq" => StarClass::DQ,
            "DC" | "dc" => StarClass::DC,
            "DCV" | "dcv" => StarClass::DCV,
            "DX" | "dx" => StarClass::DX,

            "N" | "n" => StarClass::N,
            "H" | "h" => StarClass::H,
            "X" | "x" => StarClass::X,
            "SupermassiveBlackHole" | "supermassiveblackhole" => StarClass::SupermassiveBlackHole,

            "ABlueWhiteSuperGiant" | "abluewhitesupergiant" | "A_BlueWhiteSuperGiant" => {
                StarClass::ABlueWhiteSuperGiant
            }
            "FWhiteSuperGiant" | "fwhitesupergiant" | "F_WhiteSuperGiant" => {
                StarClass::FWhiteSuperGiant
            }
            "MRedSuperGiant" | "mredsupergiant" | "M_RedSuperGiant" => StarClass::MRedSuperGiant,
            "MRedGiant" | "mredgiant" | "M_RedGiant" => StarClass::KOrangeGiant,
            "KOrangeGiant" | "korangegiant" | "K_OrangeGiant" => StarClass::KOrangeGiant,
            "RoguePlanet" | "rogueplanet" => StarClass::RoguePlanet,
            "Nebula" | "nebula" => StarClass::Nebula,
            "StellarRemnantNebula" | "stellarremnantnebula" => StarClass::StellarRemnantNebula,

            "MS" | "ms" => StarClass::MS,
            "S" | "s" => StarClass::S,

            _ => return Err(StarClassError::UnknownStarClass(s.to_string())),
        })
    }
}

from_str_deserialize_impl!(StarClass);

impl StarClass {
    /// Returns the base exploration value of the star class. The value is returned as a f32 instead
    /// of an u64 like how most credit values are represented as the base value
    /// [StarClass::SupermassiveBlackHole] contains decimal places.
    pub fn base_value(&self) -> f32 {
        match self {
            StarClass::D => 14_057.0,
            StarClass::DA => 14_057.0,
            StarClass::DAB => 14_057.0,
            StarClass::DAO => 14_057.0,
            StarClass::DAZ => 14_057.0,
            StarClass::DAV => 14_057.0,
            StarClass::DB => 14_057.0,
            StarClass::DBZ => 14_057.0,
            StarClass::DBV => 14_057.0,
            StarClass::DO => 14_057.0,
            StarClass::DOV => 14_057.0,
            StarClass::DQ => 14_057.0,
            StarClass::DC => 14_057.0,
            StarClass::DCV => 14_057.0,
            StarClass::DX => 14_057.0,
            StarClass::N => 22_628.0,
            StarClass::H => 22_628.0,
            StarClass::SupermassiveBlackHole => 33.5678,
            _ => 1_200.0,
        }
    }
}

impl Display for StarClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            StarClass::O => "O",
            StarClass::B => "B",
            StarClass::A => "A",
            StarClass::F => "F",
            StarClass::G => "G",
            StarClass::K => "K",
            StarClass::M => "M",
            StarClass::L => "L",
            StarClass::T => "T",
            StarClass::Y => "Y",
            StarClass::TTS => "TTS",
            StarClass::Ae => "Ae",
            StarClass::Be => "Be",
            StarClass::AeBe => "AeBe",
            StarClass::W => "W",
            StarClass::WN => "WN",
            StarClass::WNC => "WNC",
            StarClass::WC => "WC",
            StarClass::WO => "WO",
            StarClass::CS => "CS",
            StarClass::C => "C",
            StarClass::CN => "CN",
            StarClass::CJ => "CJ",
            StarClass::CH => "CH",
            StarClass::CHd => "CHd",
            StarClass::D => "D",
            StarClass::DA => "DA",
            StarClass::DAB => "DAB",
            StarClass::DAO => "DAO",
            StarClass::DAZ => "DAZ",
            StarClass::DAV => "DAV",
            StarClass::DB => "DB",
            StarClass::DBZ => "DBZ",
            StarClass::DBV => "DBV",
            StarClass::DO => "DO",
            StarClass::DOV => "DOV",
            StarClass::DQ => "DQ",
            StarClass::DC => "DC",
            StarClass::DCV => "DCV",
            StarClass::DX => "DX",
            StarClass::N => "N",
            StarClass::H => "H",
            StarClass::X => "X",
            StarClass::SupermassiveBlackHole => "SupermassiveBlackHole",
            StarClass::ABlueWhiteSuperGiant => "ABlueWhiteSuperGiant",
            StarClass::BBlueWhiteSuperGiant => "BBlueWhiteSuperGiant",
            StarClass::FWhiteSuperGiant => "FWhiteSuperGiant",
            StarClass::GWhiteSuperGiant => "GWhiteSuperGiant",
            StarClass::MRedSuperGiant => "MRedSuperGiant",
            StarClass::MRedGiant => "MRedGiant",
            StarClass::KOrangeGiant => "KOrangeGiant",
            StarClass::RoguePlanet => "RoguePlanet",
            StarClass::Nebula => "Nebula",
            StarClass::StellarRemnantNebula => "StellarRemnantNebula",
            StarClass::MS => "MS",
            StarClass::S => "S",
        })
    }
}
