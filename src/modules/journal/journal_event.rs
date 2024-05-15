use crate::logs::content::LogEvent;
use crate::market::Market;
use crate::modules::outfitting::Outfitting;
use crate::nav_route::NavRoute;
use crate::shipyard::Shipyard;
use crate::status::Status;

#[derive(Debug, PartialEq)]
pub enum JournalEvent {
    LogEvent(LogEvent),
    StatusEvent(Status),
    OutfittingEvent(Outfitting),
    ShipyardEvent(Shipyard),
    MarketEvent(Market),
    NavRoute(NavRoute),
}
