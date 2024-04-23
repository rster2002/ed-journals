use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum BodyType {
    Station,
    Star,
    Planet,

    #[cfg(not(feature = "strict"))]
    Unknown(String),
}
