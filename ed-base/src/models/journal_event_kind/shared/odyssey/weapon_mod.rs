use serde::Deserialize;

// TODO figure out the contents of this
#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct WeaponMod {}
