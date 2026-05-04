use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
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
