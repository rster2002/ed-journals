use serde::{Deserialize, Serialize};

/// A mod applied to a suit.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
#[cfg_attr(not(feature = "allow-unknown"), non_exhaustive)]
pub enum SuitMod {
    /// Reduces the battery consumption of the suit's utility tool.
    #[serde(rename = "suit_reducedtoolbatteryconsumption")]
    ReducedToolBatteryConsumption,

    /// Increases battery capacity.
    #[serde(rename = "suit_increasedbatterycapacity")]
    ImprovedBatteryCapacity,

    /// Increases how long the player can sprint.
    #[serde(rename = "suit_increasedsprintduration")]
    IncreasedSprintDuration,

    /// Increases movement speed when carrying a weapon.
    #[serde(rename = "suit_adsmovementspeed")]
    CombatMovementSpeed,

    /// Increases how long you can use your jetpack.
    #[serde(rename = "suit_improvedjumpassist")]
    ImprovedJumpAssist,

    /// Increases the maximum oxygen capacity the suit can carry.
    #[serde(rename = "suit_increasedo2capacity")]
    IncreasedAirReserves,

    /// Adds the ability to use nightvision.
    #[serde(rename = "suit_nightvision")]
    NightVision,

    /// Increases the scanning range.
    #[serde(rename = "suit_improvedradar")]
    EnhancedTracking,

    /// Increases the carrying capacity for each item category.
    #[serde(rename = "suit_backpackcapacity")]
    ExtraBackpackCapacity,

    /// Increases melee damage.
    #[serde(rename = "suit_increasedmeleedamage")]
    AddedMeleeDamage,

    /// Decreases how much damage the player takes from various sources.
    #[serde(rename = "suit_improvedarmourrating")]
    DamageResistance,

    /// Increases how much extra ammo the player can carry.
    #[serde(rename = "suit_increasedammoreserves")]
    ExtraAmmoCapacity,

    /// Increases how fast the shield recharges.
    #[serde(rename = "suit_increasedshieldregen")]
    FasterShieldRegen,

    /// Decreases noise and in turn improves stealth.
    #[serde(rename = "suit_quieterfootsteps")]
    QuieterFootsteps,

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    #[serde(untagged)]
    Unknown(String),
}
