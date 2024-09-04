use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum CodexAnomalyEntry {
    LagrangeClouds,
    ETypeAnomalies,
    KTypeAnomalies,
    LTypeAnomalies,
    PTypeAnomalies,
    QTypeAnomalies,
    TTypeAnomalies,
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
