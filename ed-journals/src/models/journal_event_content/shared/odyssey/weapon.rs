use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum Weapon {
    #[serde(
        alias = "Wpn_M_Shotgun_Plasma_DoubleBarrel",
        alias = "wpn_m_shotgun_plasma_doublebarrel"
    )]
    ManticoreIntimidator,

    #[serde(
        alias = "Wpn_S_Pistol_Plasma_Charged",
        alias = "wpn_s_pistol_plasma_charged"
    )]
    ManticoreTormentor,

    #[serde(
        alias = "Wpn_M_AssaultRifle_Plasma_FAuto",
        alias = "wpn_m_assaultrifle_plasma_fauto"
    )]
    ManticoreOppressor,

    #[serde(
        alias = "Wpn_M_Sniper_Plasma_Charged",
        alias = "wpn_m_sniper_plasma_charged"
    )]
    ManticoreExecutioner,

    #[serde(
        alias = "Wpn_M_SubMachineGun_Laser_FAuto",
        alias = "wpn_m_submachinegun_laser_fauto"
    )]
    TKEclipse,

    #[serde(
        alias = "Wpn_M_SubMachineGun_Kinetic_FAuto",
        alias = "wpn_m_submachinegun_kinetic_fauto"
    )]
    KarmaC44,
}
