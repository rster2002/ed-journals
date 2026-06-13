use serde::Serialize;

/// Helper struct which contains the very minimum displayable information for a system.
#[derive(Debug, Clone, Serialize)]
pub struct PartialSystemInfo {
    pub system_address: u64,
    pub star_name: String,
}
