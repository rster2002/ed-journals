use serde::{Deserialize, Serialize};

use crate::modules::materials::MaterialCategory;
use crate::modules::mixed::models::mixed_material::MixedMaterial;
use crate::modules::odyssey::ItemCategory;

/// In some cases, the game emits events that refer to materials, but can also contain Odyssey
/// items. If that is the case, this model will be used instead.
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum MixedMaterialCategory {
    ShipMaterial(MaterialCategory),
    OdysseyMaterial(ItemCategory),
}

impl From<MixedMaterial> for MixedMaterialCategory {
    fn from(value: MixedMaterial) -> Self {
        match value {
            MixedMaterial::ShipMaterial(material) => {
                MixedMaterialCategory::ShipMaterial(material.into())
            }
            MixedMaterial::OdysseyMaterial(item) => {
                MixedMaterialCategory::OdysseyMaterial(item.into())
            }
        }
    }
}
