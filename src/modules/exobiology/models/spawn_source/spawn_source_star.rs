use crate::galaxy::{StarClass, StarLuminosity};
use serde::Serialize;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize)]
pub struct SpawnSourceStar {
    pub class: StarClass,
    pub luminosity: StarLuminosity,
}
