use crate::backpack::Backpack;
use crate::logs::content::LogEvent;
use crate::market::Market;
use crate::modules::cargo::Cargo;
use crate::modules::outfitting::Outfitting;
use crate::modules_info::ModulesInfo;
use crate::nav_route::NavRoute;
use crate::ship_locker::ShipLocker;
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
    ModulesInfo(ModulesInfo),
    Backpack(Backpack),
    Cargo(Cargo),
    ShipLocker(ShipLocker),
}
