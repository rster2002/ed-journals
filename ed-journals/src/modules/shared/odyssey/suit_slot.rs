use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SuitSlot {
    PrimaryWeapon1,
    PrimaryWeapon2,
    SecondaryWeapon,
}
