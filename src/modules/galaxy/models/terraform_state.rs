use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TerraformState {
    #[serde(alias = "")]
    None,
    Terraformable,
    Terraforming,
    Terraformed,
}
