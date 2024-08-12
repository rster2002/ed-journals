use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::modules::nav_route::models::nav_route_entry::NavRouteEntry;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct NavRoute {
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "event")]
    pub event: String,
    pub route: Vec<NavRouteEntry>,
}
