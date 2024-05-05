use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum HardpointMounting {
    #[serde(rename = "Fixed", alias = "fixed")]
    Fixed,

    #[serde(rename = "Gimbal", alias = "gimbal")]
    Gimballed,

    #[serde(rename = "Turret", alias = "turret")]
    Turreted,
}

impl FromStr for HardpointMounting {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_value(Value::String(s.to_string()))
    }
}

impl Display for HardpointMounting {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            HardpointMounting::Fixed => "Fixed",
            HardpointMounting::Gimballed => "Gimballed",
            HardpointMounting::Turreted => "Turreted",
        })
    }
}
