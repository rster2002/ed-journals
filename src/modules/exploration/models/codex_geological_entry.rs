use std::fmt::{Display, Formatter};
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::from_str_deserialize_impl;

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Hash)]
pub enum CodexGeologicalEntry {
    Fumarole,
    IceFumarole,
    Geyser,
    IceGeyser,
    LavaSpout,
    GasVent,
}

#[derive(Debug, Error)]
pub enum CodexGeologicalError {
    #[error("Failed to parse geological codex entry: '{0}'")]
    FailedToParse(String),

    #[error("Unknown geological codex entry: '{0}'")]
    UnknownEntry(String),
}

impl FromStr for CodexGeologicalEntry {
    type Err = CodexGeologicalError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

from_str_deserialize_impl!(CodexGeologicalEntry);

impl Display for CodexGeologicalEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            CodexGeologicalEntry::Fumarole => "Fumarole",
            CodexGeologicalEntry::IceFumarole => "Ice Fumarole",
            CodexGeologicalEntry::Geyser => "Geyser",
            CodexGeologicalEntry::IceGeyser => "Ice Geyser",
            CodexGeologicalEntry::LavaSpout => "Lava Spout",
            CodexGeologicalEntry::GasVent => "Gas Vent",
        })
    }
}
