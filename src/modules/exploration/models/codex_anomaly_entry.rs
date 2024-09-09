use std::fmt::{Display, Formatter};
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum CodexAnomalyEntry {
    LagrangeClouds,
    ETypeAnomalies,
    KTypeAnomalies,
    LTypeAnomalies,
    PTypeAnomalies,
    QTypeAnomalies,
    TTypeAnomalies,

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    Unknown(String),
}

impl CodexAnomalyEntry {
    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    pub fn is_unknown(&self) -> bool {
        matches!(self, CodexAnomalyEntry::Unknown(_))
    }
}

#[derive(Debug, Error)]
pub enum CodexAnomalyError {
    #[error("Failed to parse codex anomaly entry: '{0}'")]
    FailedToParse(String),

    #[error("Unknown codex anomaly entry: '{0}'")]
    UnknownEntry(String),
}

impl FromStr for CodexAnomalyEntry {
    type Err = CodexAnomalyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Err(CodexAnomalyError::FailedToParse(s.to_string()))
    }
}

impl Display for CodexAnomalyEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            CodexAnomalyEntry::LagrangeClouds => "Lagrange Clouds",
            CodexAnomalyEntry::ETypeAnomalies => "E Type Anomalies",
            CodexAnomalyEntry::KTypeAnomalies => "K Type Anomalies",
            CodexAnomalyEntry::LTypeAnomalies => "L Type Anomalies",
            CodexAnomalyEntry::PTypeAnomalies => "P Type Anomalies",
            CodexAnomalyEntry::QTypeAnomalies => "Q Type Anomalies",
            CodexAnomalyEntry::TTypeAnomalies => "T Type Anomalies",
        })
    }
}
