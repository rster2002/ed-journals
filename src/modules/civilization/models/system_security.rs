use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SystemSecurity {
    #[serde(rename = "$SYSTEM_SECURITY_high;")]
    High,

    #[serde(rename = "$SYSTEM_SECURITY_medium;")]
    Medium,

    #[serde(rename = "$SYSTEM_SECURITY_low;")]
    Low,

    #[serde(rename = "$GAlAXY_MAP_INFO_state_anarchy;")]
    Anarchy,

    #[serde(rename = "$GALAXY_MAP_INFO_state_lawless;")]
    Lawless,

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}

impl Display for SystemSecurity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SystemSecurity::High => "High",
                SystemSecurity::Medium => "Medium",
                SystemSecurity::Low => "Low",
                SystemSecurity::Anarchy => "Anarchy",
                SystemSecurity::Lawless => "Lawless",

                #[cfg(not(feature = "strict"))]
                SystemSecurity::Unknown(unknown) =>
                    return write!(f, "Unknown system security: {}", unknown),
            }
        )
    }
}
