use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SuitSlot {
    PrimaryWeapon1,
    PrimaryWeapon2,
    SecondaryWeapon,
}
