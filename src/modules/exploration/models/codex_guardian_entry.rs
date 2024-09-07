use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum CodexGuardianEntry {
    AncientCasket,
}

impl Display for CodexGuardianEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            CodexGuardianEntry::AncientCasket => "Ancient Casket",
        })
    }
}
