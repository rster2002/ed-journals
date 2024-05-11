use serde::Deserialize;

// TODO I'm not yet sure what this contains
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct SuitMod {}
