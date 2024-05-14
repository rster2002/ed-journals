use serde::{Serialize, Deserialize};

// TODO figure out the contents of this
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct WeaponMod {}
