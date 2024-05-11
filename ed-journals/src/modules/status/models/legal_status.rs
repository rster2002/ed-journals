use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, PartialEq)]
pub enum LegalStatus {
    Clean,
    IllegalCargo,
    Speeding,
    Wanted,
    Hostile,
    PassengerWanted,
    Warrant,
}
