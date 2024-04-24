use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum BodyType {
    Station,
    Star,
    Planet,

    // TODO add description on when this is used
    Null,

    #[cfg(not(feature = "strict"))]
    Unknown(String),
}
