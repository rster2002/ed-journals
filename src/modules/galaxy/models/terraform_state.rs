use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TerraformState {
    #[serde(alias = "")]
    None,
    Terraformable,
    Terraforming,
    Terraformed,
}
