use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Weapon {
    #[serde(alias = "Wpn_M_Shotgun_Plasma_DoubleBarrel", alias = "wpn_m_shotgun_plasma_doublebarrel")]
    ManticoreIntimidator,

    #[serde(alias = "Wpn_S_Pistol_Plasma_Charged", alias = "wpn_s_pistol_plasma_charged")]
    ManticoreTormentor,

    #[serde(alias = "Wpn_M_AssaultRifle_Plasma_FAuto")]
    ManticoreOppressor,

    #[serde(alias = "Wpn_M_Sniper_Plasma_Charged", alias = "wpn_m_sniper_plasma_charged")]
    ManticoreExecutioner,
}
