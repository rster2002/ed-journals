use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(not(feature = "allow-unknown"), non_exhaustive)]
pub enum Weapon {
    #[serde(
        alias = "Wpn_M_AssaultRifle_Kinetic_FAuto",
        alias = "wpn_m_assaultrifle_kinetic_fauto"
    )]
    KarmaAR50,

    #[serde(
        alias = "Wpn_M_SubMachineGun_Kinetic_FAuto",
        alias = "wpn_m_submachinegun_kinetic_fauto"
    )]
    KarmaC44,

    #[serde(
        alias = "Wpn_M_Launcher_Rocket_SAuto",
        alias = "wpn_m_launcher_rocket_sauto"
    )]
    KarmaL6,

    #[serde(
        alias = "Wpn_S_Pistol_Kinetic_SAuto",
        alias = "wpn_s_pistol_kinetic_sauto"
    )]
    KarmaP15,

    #[serde(
        alias = "Wpn_M_Sniper_Plasma_Charged",
        alias = "wpn_m_sniper_plasma_charged"
    )]
    ManticoreExecutioner,

    #[serde(
        alias = "Wpn_M_Shotgun_Plasma_DoubleBarrel",
        alias = "wpn_m_shotgun_plasma_doublebarrel"
    )]
    ManticoreIntimidator,

    #[serde(
        alias = "Wpn_M_AssaultRifle_Plasma_FAuto",
        alias = "wpn_m_assaultrifle_plasma_fauto"
    )]
    ManticoreOppressor,

    #[serde(
        alias = "Wpn_S_Pistol_Plasma_Charged",
        alias = "wpn_s_pistol_plasma_charged"
    )]
    ManticoreTormentor,

    #[serde(
        alias = "Wpn_M_AssaultRifle_Laser_FAuto",
        alias = "wpn_m_assaultrifle_laser_fauto"
    )]
    TKAphelion,

    #[serde(
        alias = "Wpn_M_SubMachineGun_Laser_FAuto",
        alias = "wpn_m_submachinegun_laser_fauto"
    )]
    TKEclipse,

    #[serde(alias = "Wpn_S_Pistol_Laser_SAuto", alias = "wpn_s_pistol_laser_sauto")]
    TKZenith,

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    #[serde(untagged)]
    Unknown(String),
}

impl Display for Weapon {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Weapon::KarmaAR50 => "Karma AR-50",
                Weapon::KarmaC44 => "Karma C-44",
                Weapon::KarmaL6 => "Karma L-6",
                Weapon::KarmaP15 => "Karma P-15",

                Weapon::ManticoreExecutioner => "Manticore Executioner",
                Weapon::ManticoreIntimidator => "Manticore Intimidator",
                Weapon::ManticoreOppressor => "Manticore Oppressor",
                Weapon::ManticoreTormentor => "Manticore Tormentor",

                Weapon::TKAphelion => "TK Aphelion",
                Weapon::TKEclipse => "TK Eclipse",
                Weapon::TKZenith => "TK Zenith ",
            }
        )
    }
}
