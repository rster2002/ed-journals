use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CodexAnomalyEntry {
    LagrangeClouds,
    ETypeAnomalies,
    KTypeAnomalies,
    LTypeAnomalies,
    PTypeAnomalies,
    QTypeAnomalies,
    TTypeAnomalies,
}
