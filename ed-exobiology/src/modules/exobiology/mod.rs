mod models;
pub mod r#static;
mod traits;

pub use models::spawn_condition::SpawnCondition;
pub use models::spawn_source::SpawnSource;
pub use models::spawn_source::spawn_source_star::SpawnSourceStar;
pub use models::spawn_source::target_planet::TargetPlanet;
pub use models::spawn_source::target_system::TargetSystem;

pub use r#static::species_spawn_conditions::SPECIES_SPAWN_CONDITIONS;
pub use traits::species_spawn_conditions::SpeciesSpawnConditions;
