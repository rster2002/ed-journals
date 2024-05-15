/// Contains modules for working with journal log files providing readers for individual files or
/// all files at once.
pub mod logs;
pub mod journal;
pub mod status;
pub mod state;
pub mod exobiology;
pub mod exploration;
pub mod outfitting;
pub mod shipyard;
pub mod market;

/// Contains structs and enums which are used in multiple places. Things like commodity and material
/// names, ship types, exobiology data etc. can be found here.
pub mod models;

/// Provides some utility functions and macros that are used internally.
mod utils;

/// Module for shared internal code.
mod shared;
