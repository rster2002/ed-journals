use crate::civilization::Engineer;
use serde::{Deserialize, Serialize};

use crate::modules::materials::Material;
use crate::modules::ship::{Blueprint, BlueprintModifier, ShipModule, ShipSlot};

/// Fired when the player applies an engineering modification to one of their ship's modules.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct EngineerCraftEvent {
    /// The slot of the module being modified.
    pub slot: ShipSlot,

    /// The type of module being modified.
    pub module: ShipModule,

    /// List of paid materials for the modification. Note that these are the materials for one
    /// increase in quality and not for fully upgrading the whole level.
    pub ingredients: Vec<EngineerCraftEventIngredient>,

    /// The name of engineer that applied the modification. In some cases where the engineer is
    /// [Engineer::System] this is not set.
    pub engineer: Option<String>,

    /// The engineer where the player applied the modifications.
    #[serde(rename = "EngineerID")]
    pub engineer_id: Engineer,

    /// The id of the modification the player applied.
    #[serde(rename = "BlueprintID")]
    pub blueprint_id: u64,

    /// The name of the modification the player applied.
    pub blueprint_name: Blueprint,

    /// The level or 'grade' of the applied modification.
    pub level: u8,

    /// The quality or 'progress' of the applied modification.
    pub quality: f32,

    /// The modifiers that are affected by the modification.
    pub modifiers: Vec<EngineerCraftEventModifier>,
}

/// A material requirement for a modification.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct EngineerCraftEventIngredient {
    /// The material required.
    pub name: Material,

    /// The number of the given material required.
    pub count: u16,
}

/// An applied modifier to the module.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct EngineerCraftEventModifier {
    /// The name of the modified applied.
    pub label: BlueprintModifier,

    /// The kind of modifier that was applied.
    #[serde(flatten)]
    pub kind: EngineerCraftEventModifierKind,
}

/// The kind of applied modifier/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum EngineerCraftEventModifierKind {
    /// A change in a numeric value.
    ValueChange(EngineerCraftEventValueChange),

    /// Replacement of a certain string.
    StringChange(EngineerCraftEventStringChange),
}

/// A change in a numeric value.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct EngineerCraftEventValueChange {
    /// The new value of the target modifier.
    pub value: f32,

    /// The original value of target modifier.
    pub original_value: f32,

    /// Whether decreasing the value yields positive results.
    pub less_is_good: u8,
}

/// Replacement of a certain string.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct EngineerCraftEventStringChange {
    /// New value of the string for the given modifier.
    pub value_str: String,

    /// New localized value of the string for the given modifier.
    #[serde(rename = "ValueStr_Localised")]
    pub value_str_localized: String,
}
