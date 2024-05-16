use serde::{Serialize, Deserialize};

// TODO I'm not yet sure what this contains
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct SuitMod {}
