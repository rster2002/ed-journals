use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Item {
    // Data
    #[serde(alias = "accidentlogs")]
    AccidentLogs,

    #[serde(alias = "airqualityreports")]
    AirQualityReports,

    #[serde(alias = "atmosphericdata")]
    AtmosphericData,

    #[serde(alias = "blacklistdata")]
    BlacklistData,

    #[serde(alias = "censusdata")]
    CensusData,

    #[serde(alias = "combatantperformance")]
    CombatantPerformance,

    #[serde(alias = "evacuationprotocols")]
    EvacuationProtocols,

    #[serde(alias = "extractionyielddata")]
    ExtractionYieldData,

    #[serde(alias = "factionnews")]
    FactionNews,

    #[serde(alias = "genesequencingdata")]
    GeneSequencingData,

    #[serde(alias = "geologicaldata")]
    GeologicalData,

    #[serde(alias = "maintenancelogs")]
    MaintenanceLogs,

    #[serde(alias = "networksecurityprotocols")]
    NetworkSecurityProtocols,

    #[serde(alias = "personallogs")]
    PersonalLogs,

    #[serde(alias = "pharmaceuticalpatents")]
    PharmaceuticalPatents,

    #[serde(alias = "propaganda")]
    Propaganda,

    #[serde(alias = "purchaserecords")]
    PurchaseRecords,

    #[serde(alias = "radioactivitydata")]
    RadioactivityData,

    #[serde(alias = "reactoroutputreview")]
    ReactorOutputReview,

    #[serde(alias = "securityexpenses")]
    SecurityExpenses,

    #[serde(alias = "shareholderinformation")]
    ShareholderInformation,

    #[serde(alias = "stellaractivitylogs")]
    StellarActivityLogs,

    #[serde(alias = "topographicalsurveys")]
    TopographicalSurveys,

    #[serde(alias = "vaccinationrecords")]
    VaccinationRecords,

    #[serde(alias = "virologydata")]
    VirologyData,

    #[serde(alias = "visitorregister")]
    VisitorRegister,

    #[serde(alias = "enhancedinterrogationrecordings")]
    EnhancedInterrogationsRecords,


    // Goods
    #[serde(alias = "biologicalsample")]
    BiologicalSample,

    #[serde(alias = "buildingschematic")]
    BuildingSchematic,

    #[serde(alias = "californium")]
    Californium,

    #[serde(alias = "castfossil")]
    CastFossil,

    #[serde(alias = "compactlibrary")]
    CompactLibrary,

    #[serde(alias = "compressionliquefiedgas")]
    CompressionLiquefiedGas,

    #[serde(alias = "deepmantlesample")]
    DeepMantleSample,

    #[serde(alias = "degradedpowerregulator")]
    DegradedPowerRegulator,

    #[serde(alias = "gmeds")]
    GMeds,

    #[serde(alias = "healthmonitor")]
    HealthMonitor,

    #[serde(alias = "hush")]
    Hush,

    #[serde(alias = "inertiacanister")]
    InertiaCanister,

    #[serde(alias = "infinity")]
    Infinity,

    #[serde(alias = "insight")]
    Insight,

    #[serde(alias = "insightdatabank")]
    InsightDataBank,

    #[serde(alias = "insightentertainmentsuite")]
    InsightEntertainmentSuite,

    #[serde(alias = "ionisedgas")]
    IonisedGas,

    #[serde(alias = "microbialinhibitor")]
    MicrobialInhibitor,

    #[serde(alias = "nutritionalconcentrate")]
    NutritionalConcentrate,

    #[serde(alias = "personalcomputer")]
    PersonalComputer,

    #[serde(alias = "personaldocuments")]
    PersonalDocuments,

    #[serde(alias = "petrifiedfossil")]
    PetrifiedFossil,

    #[serde(alias = "powerregulator")]
    PowerRegulator,

    #[serde(alias = "push")]
    Push,

    #[serde(alias = "suitschematic")]
    SuitSchematic,

    #[serde(alias = "syntheticpathogen")]
    SyntheticPathogen,

    #[serde(alias = "universaltranslator")]
    UniversalTranslator,

    #[serde(alias = "vehicleschematic")]
    VehicleSchematic,

    #[serde(alias = "weaponschematic")]
    WeaponSchematic,


    // Chemicals
    #[serde(alias = "aerogel")]
    Aerogel,

    #[serde(alias = "chemicalcatalyst")]
    ChemicalCatalyst,

    #[serde(alias = "chemicalsuperbase")]
    ChemicalSuperbase,

    #[serde(alias = "epinephrine")]
    Epinephrine,

    #[serde(alias = "epoxyadhesive")]
    EpoxyAdhesive,

    #[serde(alias = "graphene")]
    Graphene,

    #[serde(alias = "oxygenicbacteria")]
    OxygenicBacteria,

    #[serde(alias = "phneutraliser")]
    PHNeutraliser,

    #[serde(alias = "rdx")]
    RDX,

    #[serde(alias = "viscoelasticpolymer")]
    ViscoelasticPolymer,


    // Circuits
    #[serde(alias = "circuitboard")]
    CircuitBoard,

    #[serde(alias = "circuitswitch")]
    CircuitSwitch,

    #[serde(alias = "electricalfuse")]
    ElectricalFuse,

    #[serde(alias = "electricalwiring")]
    ElectricalWiring,

    #[serde(alias = "elecromagnet")]
    Elecromagnet,

    #[serde(alias = "ionbattery")]
    IonBattery,

    #[serde(alias = "metalcoil")]
    MetalCoil,

    #[serde(alias = "microsupercapacitor")]
    MicroSupercapacitor,

    #[serde(alias = "microtransformer")]
    MicroTransformer,

    #[serde(alias = "microelectrode")]
    Microelectrode,

    #[serde(alias = "motor")]
    Motor,

    #[serde(alias = "opticalfibre")]
    OpticalFibre,


    // Tech
    #[serde(alias = "carbonfibreplating")]
    CarbonFibrePlating,

    #[serde(alias = "encryptedmemorychip")]
    EncryptedMemoryChip,

    #[serde(alias = "memorychip")]
    MemoryChip,

    #[serde(alias = "microthrusters")]
    MicroThrusters,

    #[serde(alias = "opticallens")]
    OpticalLens,

    #[serde(alias = "scrambler")]
    Scrambler,

    #[serde(alias = "transmitter")]
    Transmitter,

    #[serde(alias = "tungstencarbide")]
    TungstenCarbide,

    #[serde(alias = "weaponcomponent")]
    WeaponComponent,


    // Consumables
    #[serde(alias = "energycell")]
    EnergyCell,

    #[serde(alias = "amm_grenade_frag")]
    FragGranade,

    #[serde(alias = "healthpack")]
    Medkit,

    #[serde(alias = "amm_grenade_emp")]
    ShieldDisruptor,

    #[serde(alias = "amm_grenade_shield")]
    ShieldProjector,


    // Item
    #[serde(alias = "largecapacitypowerregulator")]
    LargeCapacityPowerRegulator,


    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}
