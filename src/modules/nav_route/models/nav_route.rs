use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::modules::nav_route::models::nav_route_entry::NavRouteEntry;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct NavRoute {
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "event")]
    pub event: String,
    pub route: Vec<NavRouteEntry>,
}
