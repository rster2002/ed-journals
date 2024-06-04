pub use models::genus::Genus;
pub use models::spawn_condition::SpawnCondition;
pub use models::spawn_source::spawn_source_star::SpawnSourceStar;
pub use models::spawn_source::target_planet::TargetPlanet;
pub use models::spawn_source::target_system::TargetSystem;
pub use models::spawn_source::SpawnSource;
pub use models::species::Species;
pub use models::variant::Variant;
pub use models::variant::VariantError;
pub use models::variant_color::VariantColor;
pub use models::variant_color::VariantColorError;
pub use models::variant_source::VariantSource;
pub use models::variant_source::VariantSourceError;

mod models;
mod r#static;
