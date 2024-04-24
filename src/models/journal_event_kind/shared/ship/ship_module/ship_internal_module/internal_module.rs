use std::str::FromStr;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum InternalModule {
    #[serde(rename = "powerplant")]
    PowerPlant,

    #[serde(rename = "engine")]
    Thrusters,
}

impl FromStr for InternalModule {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_value(Value::String(s.to_string()))
    }
}
