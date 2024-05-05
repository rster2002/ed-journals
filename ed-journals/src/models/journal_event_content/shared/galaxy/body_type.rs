use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum BodyType {
    AsteroidCluster,
    PlanetaryRing,
    Station,
    Star,
    StellarRing,
    Planet,

    // TODO add description on when this is used
    Null,

    #[cfg(not(feature = "strict"))]
    Unknown(String),
}
