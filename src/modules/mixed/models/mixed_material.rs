use serde::{Serialize, Deserialize};

use crate::modules::models::materials::material::Material;
use crate::modules::models::odyssey::item::Item;

/// In some cases, the game emits events that refer to materials, but can also contain Odyssey
/// items. If that is the case, this model will be used instead.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum MixedMaterial {
    ShipMaterial(Material),
    OdysseyMaterial(Item),
}
