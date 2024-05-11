use serde::Deserialize;

use crate::modules::shared::shared_material::SharedMaterial;
use crate::modules::shared::materials::material_category::MaterialCategory;
use crate::modules::shared::odyssey::item_type::ItemType;

/// In some cases, the game emits events that refer to materials, but can also contain Odyssey
/// items. If that is the case, this model will be used instead.
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum SharedMaterialCategory {
    ShipMaterial(MaterialCategory),
    OdysseyMaterial(ItemType),
}

impl From<SharedMaterial> for SharedMaterialCategory {
    fn from(value: SharedMaterial) -> Self {
        match value {
            SharedMaterial::ShipMaterial(material) => {
                SharedMaterialCategory::ShipMaterial(material.into())
            }
            SharedMaterial::OdysseyMaterial(item) => {
                SharedMaterialCategory::OdysseyMaterial(item.into())
            }
        }
    }
}