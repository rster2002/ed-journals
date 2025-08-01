use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum BodyType {
    AsteroidCluster,
    PlanetaryRing,
    Station,
    Star,
    StellarRing,
    Planet,

    // TODO add description on when this is used
    Null,

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    Unknown(String),
}
