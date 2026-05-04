use serde::{Deserialize, Serialize};

/// This event is fired when something changes in the `ModulesInfo.json` file and does not
/// contain any data itself.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct ModuleInfoEvent {}
