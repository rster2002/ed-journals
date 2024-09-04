use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum CodexGeologicalEntry {
    Fumarole,
    IceFumarole,
    Geyser,
    IceGeyser,
    LavaSpout,
    GasVent,
}

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
