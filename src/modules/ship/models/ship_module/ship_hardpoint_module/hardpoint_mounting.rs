use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum HardpointMounting {
    #[serde(rename = "Fixed", alias = "fixed")]
    Fixed,

    #[serde(rename = "Gimbal", alias = "gimbal", alias = "Gimballed")]
    Gimballed,

    #[serde(rename = "Turret", alias = "turret", alias = "Turreted")]
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
        write!(
            f,
            "{}",
            match self {
                HardpointMounting::Fixed => "Fixed",
                HardpointMounting::Gimballed => "Gimballed",
                HardpointMounting::Turreted => "Turreted",
            }
        )
    }
}
