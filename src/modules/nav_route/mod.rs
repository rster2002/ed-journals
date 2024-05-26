pub use models::nav_route::NavRoute;
pub use models::nav_route_entry::NavRouteEntry;

pub mod blocking;
mod models;

#[cfg(feature = "asynchronous")]
pub mod asynchronous;
