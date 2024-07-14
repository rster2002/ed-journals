use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum LegalStatus {
    Clean,
    IllegalCargo,
    Speeding,
    Wanted,
    Hostile,
    PassengerWanted,
    Warrant,
}
