use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

/// The state a faction can be in. The primary faction for a system also dictates the state of the
/// system as a whole.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum FactionState {
    Blight,
    Boom,
    Bust,
    CivilLiberty,
    CivilUnrest,
    CivilWar,
    Drought,
    Election,
    Exiled,
    Expansion,
    Famine,
    InfrastructureFailure,
    Investment,
    Lockdown,
    NaturalDisaster,
    Outbreak,
    PirateAttack,
    PublicHoliday,
    Retreat,
    Terrorism,
    War,
    None,

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    #[serde(untagged)]
    Unknown(String),
}

impl Display for FactionState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FactionState::Blight => "Blight",
                FactionState::Boom => "Boom",
                FactionState::Bust => "Bust",
                FactionState::CivilLiberty => "Civil Liberty",
                FactionState::CivilUnrest => "Civil Unrest",
                FactionState::CivilWar => "Civil War",
                FactionState::Drought => "Drought",
                FactionState::Election => "Election",
                FactionState::Exiled => "Exiled",
                FactionState::Expansion => "Expansion",
                FactionState::Famine => "Famine",
                FactionState::InfrastructureFailure => "Infrastructure Failure",
                FactionState::Investment => "Investment",
                FactionState::Lockdown => "Lockdown",
                FactionState::NaturalDisaster => "Natural Disaster",
                FactionState::Outbreak => "Outbreak",
                FactionState::PirateAttack => "Pirate Attack",
                FactionState::PublicHoliday => "Public Holiday",
                FactionState::Retreat => "Retreat",
                FactionState::Terrorism => "Terrorism",
                FactionState::War => "War",
                FactionState::None => "None",

                #[cfg(feature = "allow-unknown")]
                FactionState::Unknown(unknown) =>
                    return write!(f, "Unknown faction state: {}", unknown),
            }
        )
    }
}
