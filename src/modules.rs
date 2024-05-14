/// Contains modules for working with journal log files providing readers for individual files or
/// all files at once.
pub mod logs;
pub mod journal;
pub mod status;
pub mod state;
pub mod exobiology;

/// Contains structs and enums which are shared across events. Things like commodity and material
/// names, ship types, exobiology data etc. can be found here.
pub mod shared;

/// Provides different implementations for blocking current execution, whether it be synchronous or
/// asynchronous.
mod blockers;

/// Provides some utility functions and macros that are used internally.
mod utils;
