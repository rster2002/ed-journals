/// Contains modules for working with journal log files providing readers for individual files or
/// all files at once.
pub mod logs;

/// Allows for tracking journal directory as a whole, firing events for when logs change but also
/// for when `.json` files are updated.
pub mod journal;

/// Utilities for working with the `Status.json` file. The `Status.json` file contains information
/// about the current status of the player and their ship. Things like if the player is docked
/// or landed or the location of the player on a planet.
pub mod status;

/// Used to construct a state by feeding it events from the journal logs which can then be used
/// to create relations between events and create a history.
pub mod state;
pub mod exobiology;

/// Contains functions for exploration related things, like calculating estimated exploration value
/// for stars and planets and for analysing scans for unusual data.
pub mod exploration;

/// Utilities for working with the `Outfitting.json` file. Is updated when opening the outfitting
/// menu on stations and includes all the modules that are available for purchase at that market.
/// Is reset every time the player opens the outfitting menu.
pub mod outfitting;

/// Utilities for working with the `Shipyard.json` file. Is updated when opening the shipyard menu
/// on stations and lists all the ships that are available to buy at that location. Resets every
/// time the player opens the shipyard menu.
pub mod shipyard;

/// Utilities for working with the `Market.json` file. Includes a list of commodities that are being
/// sold/requested at the current location. Resets every time the player opens the commodity market.
pub mod market;

/// Utilities for working with the `NavRoute.json` file. Provides the current plotted nav route for
/// the player. Resets if the player selects a new route destination or when clearing the route.
pub mod nav_route;

/// Utilities for working with the `ModulesInfo.json` file. This file provides information about
/// the current state of the modules of the player's ship.
pub mod modules_info;

/// Utilities for working with the `Backpack.json` file. Contains all the items that the player
/// currently has in their backpack. This is only filled if the player has the Odyssey expansion.
/// Updates when there are changes to the player's backpack.
pub mod backpack;

/// Utilities for working with the `Cargo.json` file. Includes a list of all the commodities that
/// the player currently has in their inventory. This is updated for both the player's ship and
/// an SRV, so at one time it might show the inventory of the player's ship and the next it shows
/// the inventory of the SRV, but never both at once. Is updated when there are changes to the
/// cargo hold or when switching between ship and SRV and vise versa.
pub mod cargo;

/// Utilities for working with the `ShipLocker.json` file. This file includes all the Odyssey items
/// the player has stored on their ship. Is updated when there are changes.
pub mod ship_locker;

/// Provides some utility functions and macros that are used internally.
mod utils;

/// Module for shared internal code.
mod shared;

/// Provides models for bodies like stars and planets.
pub mod galaxy;
/// For models related to populated systems and the different factions and powers in the galaxy.
pub mod civilization;
/// Contains models for models for commander ranks.
pub mod commander;

pub mod materials;
pub mod trading;
pub mod odyssey;
pub mod ship;
pub mod station;
pub mod thargoid;
pub mod small;
pub mod mixed;
