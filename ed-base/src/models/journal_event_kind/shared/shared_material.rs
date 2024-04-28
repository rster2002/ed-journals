use crate::models::journal_event_kind::shared::materials::material::Material;
use crate::models::journal_event_kind::shared::odyssey::item::Item;
use serde::Deserialize;

/// In some cases, the game emits events that refer to materials, but can also contain Odyssey
/// items. If that is the case, this model will be used instead.
#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(untagged)]
pub enum SharedMaterial {
    ShipMaterial(Material),
    OdysseyMaterial(Item),
}
