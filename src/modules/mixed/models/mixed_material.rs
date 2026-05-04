use serde::{Deserialize, Serialize};

use crate::modules::materials::Material;
use crate::modules::odyssey::Item;

/// In some cases, the game emits events that refer to materials, but can also contain Odyssey
/// items. If that is the case, this model will be used instead.
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum MixedMaterial {
    ShipMaterial(Material),
    OdysseyMaterial(Item),
}
