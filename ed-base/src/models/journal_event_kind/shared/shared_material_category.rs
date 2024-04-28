use crate::models::journal_event_kind::shared::materials::material_category::MaterialCategory;
use crate::models::journal_event_kind::shared::odyssey::item_type::ItemType;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(untagged)]
pub enum SharedMaterialCategory {
    ShipMaterial(MaterialCategory),
    OdysseyMaterial(ItemType),
}
