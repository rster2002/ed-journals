use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum StationService {
    #[serde(rename = "ondockmission")]
    OnDockMission,

    #[serde(rename = "blackmarket")]
    BlackMarket,

    #[serde(rename = "missions")]
    Missions,

    #[serde(rename = "missionsgenerated")]
    MissionsGenerated,

    #[serde(rename = "facilitator")]
    Facilitator,

    #[serde(rename = "powerplay")]
    Powerplay,

    #[serde(rename = "outfitting")]
    Outfitting,

    #[serde(rename = "livery")]
    Livery,

    /// Tuning is an old service that does not hold any meaning anymore. Check this
    /// [Frontiers community forum post](https://forums.frontier.co.uk/threads/tuning-in-station-facilities-what-is-it.362951/)
    #[serde(rename = "tuning")]
    Tuning,

    #[serde(rename = "searchrescue")]
    SearchAndRescue,

    #[serde(rename = "dock")]
    Dock,

    #[serde(rename = "autodock")]
    AutoDock,

    #[serde(rename = "commodities")]
    Commodities,

    #[serde(rename = "contacts")]
    Contacts,

    #[serde(rename = "exploration")]
    Exploration,

    #[serde(rename = "crewlounge")]
    CrewLounge,

    #[serde(rename = "rearm")]
    Rearm,

    #[serde(rename = "refuel")]
    Refuel,

    #[serde(rename = "repair")]
    Repair,

    #[serde(rename = "engineer")]
    Engineer,

    #[serde(rename = "flightcontroller")]
    FlightController,

    #[serde(rename = "stationoperations")]
    StationOperations,

    #[serde(rename = "stationMenu")]
    StationMenu,

    #[serde(rename = "carriervendor")]
    CarrierVendor,

    #[serde(rename = "carrierfuel")]
    CarrierFuel,

    #[serde(rename = "carriermanagement")]
    CarrierManagement,

    #[serde(rename = "socialspace")]
    SocialSpace,

    #[serde(rename = "bartender")]
    Bartender,

    #[serde(rename = "apexinterstellar")]
    ApexInterstellar,

    #[serde(rename = "vistagenomics")]
    VistaGenomics,

    #[serde(rename = "pioneersupplies")]
    PioneerSupplies,

    #[serde(rename = "shipyard")]
    Shipyard,

    #[serde(rename = "voucherredemption")]
    RedemptionOffice,

    #[serde(rename = "frontlinesolutions")]
    FrontlineSolutions,

    #[serde(rename = "materialtrader")]
    MaterialTrader,

    #[serde(rename = "techBroker")]
    TechnologyBroker,

    // TODO not sure what this is
    #[serde(rename = "shop")]
    Shop,

    // TODO I assume this is where FC can buy module packs
    #[serde(rename = "modulepacks")]
    ModulePacks,

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    #[serde(untagged)]
    Unknown(String),
}

impl Display for StationService {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                StationService::OnDockMission => "On-Dock Mission",
                StationService::BlackMarket => "Black Market",
                StationService::Missions => "Missions",
                StationService::MissionsGenerated => "Missions Generated",
                StationService::Facilitator => "Facilitator",
                StationService::Powerplay => "Powerplay",
                StationService::Outfitting => "Outfitting",
                StationService::Livery => "Livery",
                StationService::Tuning => "Tuning",
                StationService::SearchAndRescue => "Search & Rescue",
                StationService::Dock => "Dock",
                StationService::AutoDock => "Auto Dock",
                StationService::Commodities => "Commodities",
                StationService::Contacts => "Contacts",
                StationService::Exploration => "Exploration",
                StationService::CrewLounge => "Crew Lounge",
                StationService::Rearm => "Rearm",
                StationService::Refuel => "Refuel",
                StationService::Repair => "Repair",
                StationService::Engineer => "Engineer",
                StationService::FlightController => "Flight Controller",
                StationService::StationOperations => "Station Operations",
                StationService::StationMenu => "Station Menu",
                StationService::CarrierVendor => "Carrier Vendor",
                StationService::CarrierFuel => "Carrier Fuel",
                StationService::CarrierManagement => "Carrier Management",
                StationService::SocialSpace => "Social Space",
                StationService::Bartender => "Bartender",
                StationService::ApexInterstellar => "Apex Interstellar",
                StationService::VistaGenomics => "Vista Genomics",
                StationService::PioneerSupplies => "Pioneer Supplies",
                StationService::Shipyard => "Shipyard",
                StationService::RedemptionOffice => "Redemption Office",
                StationService::FrontlineSolutions => "Frontline Solutions",
                StationService::MaterialTrader => "Material Trader",
                StationService::TechnologyBroker => "Technology Broker",
                StationService::Shop => "Shop",
                StationService::ModulePacks => "Module Packs",

                #[cfg(feature = "allow-unknown")]
                StationService::Unknown(unknown) =>
                    return write!(f, "Unknown station service: {}", unknown),
            }
        )
    }
}
