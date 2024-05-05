use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum SuitSlot {
    PrimaryWeapon1,
    PrimaryWeapon2,
    SecondaryWeapon,
}
