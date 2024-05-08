use std::fmt::{Display, Formatter};

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum SystemSecurity {
    #[serde(rename = "$SYSTEM_SECURITY_high;")]
    High,

    #[serde(rename = "$SYSTEM_SECURITY_medium;")]
    Medium,

    #[serde(rename = "$SYSTEM_SECURITY_low;")]
    Low,

    #[serde(rename = "$GAlAXY_MAP_INFO_state_anarchy;")]
    Anarchy,

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}

impl Display for SystemSecurity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            SystemSecurity::High => "High",
            SystemSecurity::Medium => "Medium",
            SystemSecurity::Low => "Low",
            SystemSecurity::Anarchy => "Anarchy",

            #[cfg(not(feature = "strict"))]
            SystemSecurity::Unknown(unknown) => return write!(f, "Unknown system security: {}", unknown),
        })
    }
}
