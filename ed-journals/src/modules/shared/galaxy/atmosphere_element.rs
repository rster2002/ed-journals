use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum AtmosphereElement {
    Water,
    Oxygen,
    CarbonDioxide,
    SulphurDioxide,
    Ammonia,
    Methane,
    Nitrogen,
    Hydrogen,
    Helium,
    Neon,
    Argon,
    Silicates,
    Iron,
}
