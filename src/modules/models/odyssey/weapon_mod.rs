use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum WeaponMod {
    #[serde(rename = "weapon_handling")]
    ImprovedHandling,

    #[serde(rename = "weapon_suppression_unpressurised")]
    UnpressurisedSuppression,

    #[serde(rename = "weapon_suppression_pressurised")]
    PressurizedSuppression,

    #[serde(rename = "weapon_scope")]
    ImprovedScope,

    #[serde(rename = "weapon_clipsize")]
    IncreasedClipSize,

    #[serde(rename = "weapon_reloadspeed")]
    ImprovedReloadSpeed,

    #[serde(rename = "weapon_backpackreloading")]
    BackpackReloading,
}
