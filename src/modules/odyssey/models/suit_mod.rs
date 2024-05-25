use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum SuitMod {
    #[serde(rename = "suit_reducedtoolbatteryconsumption")]
    ReducedToolBatteryConsumption,

    #[serde(rename = "suit_increasedbatterycapacity")]
    ImprovedBatteryCapacity,

    #[serde(rename = "suit_increasedsprintduration")]
    IncreasedSprintDuration,

    CombatMovementSpeed,
    ImprovedJumpAssist,

    #[serde(rename = "suit_increasedo2capacity")]
    IncreasedAirReserves,
    NightVision,

    #[serde(rename = "suit_improvedradar")]
    EnhancedTracking,

    #[serde(rename = "suit_backpackcapacity")]
    ExtraBackpackCapacity,
    AddedMeleeDamage,
    DamageResistance,

    #[serde(rename = "suit_increasedammoreserves")]
    ExtraAmmoCapacity,

    #[serde(rename = "suit_increasedshieldregen")]
    FasterShieldRegen,

    #[serde(rename = "suit_quieterfootsteps")]
    QuieterFootsteps,
}
