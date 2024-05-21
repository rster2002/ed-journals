use serde::{Deserialize, Serialize};

use crate::modules::materials::Material;
use crate::modules::ship::{Blueprint, BlueprintModifier, ShipModule, ShipSlot};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct EngineerCraftEventIngredient {
    pub name: Material,
    pub count: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct EngineerCraftEventModifier {
    pub label: BlueprintModifier,
    pub value: f32,
    pub original_value: f32,
    pub less_is_good: u8,
}
