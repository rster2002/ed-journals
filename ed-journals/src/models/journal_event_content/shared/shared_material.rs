use serde::Deserialize;

use crate::models::journal_event_content::shared::materials::material::Material;
use crate::models::journal_event_content::shared::odyssey::item::Item;

/// In some cases, the game emits events that refer to materials, but can also contain Odyssey
/// items. If that is the case, this model will be used instead.
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum SharedMaterial {
    ShipMaterial(Material),
    OdysseyMaterial(Item),
}
