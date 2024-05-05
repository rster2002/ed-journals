use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
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

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}
