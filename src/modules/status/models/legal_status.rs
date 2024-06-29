use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum LegalStatus {
    Clean,
    IllegalCargo,
    Speeding,
    Wanted,
    Hostile,
    PassengerWanted,
    Warrant,
}
