use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CodexGeologicalEntry {
    Fumarole,
    IceFumarole,
    Geyser,
    IceGeyser,
    LavaSpout,
    GasVent,
}
