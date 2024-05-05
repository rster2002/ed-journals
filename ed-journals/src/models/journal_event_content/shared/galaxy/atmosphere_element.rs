use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
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
