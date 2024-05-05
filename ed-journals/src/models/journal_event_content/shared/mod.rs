/// For models related to populated systems and the different factions and powers in the galaxy.
pub mod civilization;

/// Contains models for models for commander ranks.
pub mod commander;

/// Includes exploration models like exobiology species information and variants and codex entries.
pub mod exploration;

/// Provides models for bodies like stars and planets.
pub mod galaxy;

/// Models for 'ship' materials. These are the materials that are used for ship engineering. For
/// materials for Odyssey suit and weapon engineering, check out the [odyssey] module.
pub mod materials;

/// Contains models for Odyssey specific entries like suit type and weapons.
pub mod odyssey;

/// Special model for handling scenarios where the input might either be a 'ship' commodity or an
/// Odyssey commodity.
pub mod shared_commodity;

/// Special model for handling scenarios where the input might either be 'ship' material or an
/// Odyssey material.
pub mod shared_material;

/// Special model for handling scenarios where the input might either be a 'ship' material category
/// or an Odyssey category.
pub mod shared_material_category;

/// Provides models for everything related to ships, like the ship hull 'type' and structs for
/// representing ship slots and modules.
pub mod ship;

/// Includes models for station services and missions.
pub mod station;

/// Models for thargoid ships.
pub mod thargoid;

/// Primarily provides the [trading::commodity::Commodity] enum.
pub mod trading;
