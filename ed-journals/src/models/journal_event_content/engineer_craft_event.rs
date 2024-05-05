use crate::models::journal_event_content::shared::materials::material::Material;
use crate::models::journal_event_content::shared::ship::blueprint::Blueprint;
use crate::models::journal_event_content::shared::ship::blueprint_modifier::BlueprintModifier;
use crate::models::journal_event_content::shared::ship::ship_module::ShipModule;
use crate::models::journal_event_content::shared::ship::ship_slot::ShipSlot;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct EngineerCraftEvent {
    pub slot: ShipSlot,
    pub module: ShipModule,
    pub ingredients: Vec<EngineerCraftEventIngredient>,
    pub engineer: String,

    #[serde(rename = "EngineerID")]
    pub engineer_id: u64,

    #[serde(rename = "BlueprintID")]
    pub blueprint_id: u64,
    pub blueprint_name: Blueprint,
    pub level: u8,
    pub quality: f32,
    pub modifiers: Vec<EngineerCraftEventModifier>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct EngineerCraftEventIngredient {
    pub name: Material,
    pub count: u8,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct EngineerCraftEventModifier {
    pub label: BlueprintModifier,
    pub value: f32,
    pub original_value: f32,
    pub less_is_good: u8,
}
