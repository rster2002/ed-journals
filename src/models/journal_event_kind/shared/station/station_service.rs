use crate::from_str_deserialize_impl;
use std::str::FromStr;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum StationService {
    BlackMarket,
    Missions,
    MissionsGenerated,
    Facilitator,
    Powerplay,
    Outfitting,
    Livery,

    /// Tuning is an old service that does not hold any meaning anymore. Check this
    /// [Frontiers community forum post](https://forums.frontier.co.uk/threads/tuning-in-station-facilities-what-is-it.362951/)
    Tuning,
    SearchAndRescue,
    Dock,
    AutoDock,
    Commodities,
    Contacts,
    Exploration,
    CrewLounge,
    Rearm,
    Refuel,
    Repair,
    Engineer,
    FlightController,
    StationOperations,
    StationMenu,
    CarrierVendor,
    CarrierFuel,
    CarrierManagement,
    SocialSpace,
    Bartender,
    ApexInterstellar,
    VistaGenomics,
    PioneerSupplies,
    Shipyard,
    RedemptionOffice,
    FrontlineSolutions,
    MaterialTrader,

    // TODO not sure what this is
    Shop,

    #[cfg(not(feature = "strict"))]
    Unknown(String),
}

impl FromStr for StationService {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blackmarket" => Ok(StationService::BlackMarket),
            "missions" => Ok(StationService::Missions),
            "missionsgenerated" => Ok(StationService::MissionsGenerated),
            "facilitator" => Ok(StationService::Facilitator),
            "powerplay" => Ok(StationService::Powerplay),
            "outfitting" => Ok(StationService::Outfitting),
            "livery" => Ok(StationService::Livery),
            "tuning" => Ok(StationService::Tuning),
            "searchrescue" => Ok(StationService::SearchAndRescue),
            "dock" => Ok(StationService::Dock),
            "autodock" => Ok(StationService::AutoDock),
            "commodities" => Ok(StationService::Commodities),
            "contacts" => Ok(StationService::Contacts),
            "exploration" => Ok(StationService::Exploration),
            "crewlounge" => Ok(StationService::CrewLounge),
            "rearm" => Ok(StationService::Rearm),
            "refuel" => Ok(StationService::Refuel),
            "repair" => Ok(StationService::Repair),
            "engineer" => Ok(StationService::Engineer),
            "flightcontroller" => Ok(StationService::FlightController),
            "stationoperations" => Ok(StationService::StationOperations),
            "stationMenu" => Ok(StationService::StationMenu),
            "carriervendor" => Ok(StationService::CarrierVendor),
            "carrierfuel" => Ok(StationService::CarrierFuel),
            "carriermanagement" => Ok(StationService::CarrierManagement),
            "socialspace" => Ok(StationService::SocialSpace),
            "bartender" => Ok(StationService::Bartender),
            "apexinterstellar" => Ok(StationService::ApexInterstellar),
            "vistagenomics" => Ok(StationService::VistaGenomics),
            "pioneersupplies" => Ok(StationService::PioneerSupplies),
            "shipyard" => Ok(StationService::Shipyard),
            "voucherredemption" => Ok(StationService::RedemptionOffice),
            "frontlinesolutions" => Ok(StationService::FrontlineSolutions),
            "materialtrader" => Ok(StationService::MaterialTrader),
            "shop" => Ok(StationService::Shop),

            #[cfg(not(feature = "strict"))]
            _ => Ok(StationService::Unknown(s.to_string())),

            #[cfg(feature = "strict")]
            _ => Err(s.to_string()),
        }
    }
}

from_str_deserialize_impl!(StationService);
