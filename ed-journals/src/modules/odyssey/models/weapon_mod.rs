use serde::{Deserialize, Serialize};

/// A mod applied to an Odyssey weapon.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum WeaponMod {
    #[serde(rename = "weapon_handling")]
    FasterHandling,

    #[serde(rename = "weapon_accuracy")]
    HigherAccuracy,

    #[serde(rename = "weapon_suppression_pressurised")]
    PressurizedNoiceSuppressor,

    #[serde(rename = "weapon_suppression_unpressurised")]
    UnpressurizedNoiceSuppressor,

    #[serde(rename = "weapon_range")]
    GreaterRange,

    #[serde(rename = "weapon_clipsize")]
    MagazineSize,

    #[serde(rename = "weapon_reloadspeed")]
    ReloadSpeed,

    #[serde(rename = "weapon_backpackreloading")]
    StowedReloading,

    #[serde(rename = "weapon_scope")]
    Scope,

    #[serde(rename = "weapon_stability")]
    Stability,
}
