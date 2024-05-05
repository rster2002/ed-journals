use crate::models::journal_event_content::shared::materials::material_category::MaterialCategory;
use crate::models::journal_event_content::shared::odyssey::item_type::ItemType;
use serde::Deserialize;
use crate::journal_event_content::shared::shared_material::SharedMaterial;

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
            SharedMaterial::ShipMaterial(material) => SharedMaterialCategory::ShipMaterial(material.into()),
            SharedMaterial::OdysseyMaterial(item) => SharedMaterialCategory::OdysseyMaterial(item.into()),
        }
    }
}
