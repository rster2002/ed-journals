use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TerraformState {
    #[serde(alias = "")]
    None,
    Terraformable,
    Terraforming,
    Terraformed,
}
