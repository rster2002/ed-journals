use std::fmt::{Display, Formatter};

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum StationType {
    AsteroidBase,
    Bernal,
    Orbis,
    Coriolis,
    Ocellus,
    Outpost,
    FleetCarrier,
    MegaShip,
    CraterOutpost,
    CraterPort,
    OnFootSettlement,

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}

impl Display for StationType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                StationType::AsteroidBase => "Asteroid Base",
                StationType::Bernal => "Bernal",
                StationType::Orbis => "Orbis",
                StationType::Coriolis => "Coriolis",
                StationType::Ocellus => "Ocellus",
                StationType::Outpost => "Outpost",
                StationType::FleetCarrier => "Fleet Carrier",
                StationType::MegaShip => "Mega Ship",
                StationType::CraterOutpost => "Crater Outpost",
                StationType::CraterPort => "Crater Port",
                StationType::OnFootSettlement => "On-Foot Settlement",

                #[cfg(not(feature = "strict"))]
                StationType::Unknown(unknown) =>
                    return write!(f, "Unknown station type: {}", unknown),
            }
        )
    }
}
