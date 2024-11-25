use serde::{Deserialize, Serialize};

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
}
