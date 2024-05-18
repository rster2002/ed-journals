use serde::{Serialize, Deserialize};

use crate::modules::mixed::models::mixed_material::MixedMaterial;
use crate::modules::models::materials::material_category::MaterialCategory;
use crate::modules::models::odyssey::item_type::ItemType;

/// In some cases, the game emits events that refer to materials, but can also contain Odyssey
/// items. If that is the case, this model will be used instead.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum MixedMaterialCategory {
    ShipMaterial(MaterialCategory),
    OdysseyMaterial(ItemType),
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
