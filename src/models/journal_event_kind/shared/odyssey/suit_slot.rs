use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum SuitSlot {
    PrimaryWeapon1,
    PrimaryWeapon2,
    SecondaryWeapon,
}
