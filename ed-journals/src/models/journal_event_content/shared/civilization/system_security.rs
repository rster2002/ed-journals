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
