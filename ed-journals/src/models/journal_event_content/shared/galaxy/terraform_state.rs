use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum TerraformState {
    #[serde(alias = "")]
    None,
    Terraformable,
    Terraforming,
    Terraformed,
}
