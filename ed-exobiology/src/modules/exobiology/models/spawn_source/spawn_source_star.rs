use ed_journals::galaxy::{StarClass, StarLuminosity};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct SpawnSourceStar {
    pub class: StarClass,
    pub luminosity: StarLuminosity,
}
