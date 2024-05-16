use serde::Deserialize;
use crate::models::trading::commodity::Commodity;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CargoEntry {
    pub name: Commodity,
    pub count: u16,
    pub stolen: u16,
}
