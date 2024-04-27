use std::str::FromStr;
use serde::Deserialize;
use thiserror::Error;
use crate::from_str_deserialize_impl;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Material {
    // Raw
    #[serde(alias = "carbon")]
    Carbon,

    #[serde(alias = "vanadium")]
    Vanadium,

    #[serde(alias = "niobium")]
    Niobium,

    #[serde(alias = "yttrium")]
    Yttrium,


    #[serde(alias = "phosphorus")]
    Phosphorus,

    #[serde(alias = "chromium")]
    Chromium,

    #[serde(alias = "molybdenum")]
    Molybdenum,

    #[serde(alias = "technetium")]
    Technetium,


    #[serde(alias = "sulphur")]
    Sulphur,

    #[serde(alias = "manganese")]
    Manganese,

    #[serde(alias = "cadmium")]
    Cadmium,

    #[serde(alias = "ruthenium")]
    Ruthenium,


    #[serde(alias = "iron")]
    Iron,

    #[serde(alias = "zinc")]
    Zinc,

    #[serde(alias = "tin")]
    Tin,

    #[serde(alias = "selenium")]
    Selenium,


    #[serde(rename= "nickel")]
    Nickel,

    #[serde(rename= "germanium")]
    Germanium,

    #[serde(rename= "tungsten")]
    Tungsten,

    #[serde(rename= "tellurium")]
    Tellurium,


    #[serde(alias = "rhenium")]
    Rhenium,

    #[serde(alias = "arsenic")]
    Arsenic,

    #[serde(alias = "mercury")]
    Mercury,

    #[serde(alias = "polonium")]
    Polonium,


    #[serde(alias = "lead")]
    Lead,

    #[serde(alias = "zirconium")]
    Zirconium,

    #[serde(alias = "boron")]
    Boron,

    #[serde(alias = "antimony")]
    Antimony,

    // Manufactured
    #[serde(alias = "chemicalstorageunits")]
    ChemicalStorageUnits,

    #[serde(alias = "chemicalprocessors")]
    ChemicalProcessors,

    #[serde(alias = "chemicaldistillery")]
    ChemicalDistillery,

    #[serde(alias = "chemicalmanipulators")]
    ChemicalManipulators,

    #[serde(alias = "pharmaceuticalisolators")]
    PharmaceuticalIsolators,


    #[serde(alias = "temperedalloys")]
    TemperedAlloys,

    #[serde(alias = "heatresistantceramics")]
    HeatResistantCeramics,

    #[serde(alias = "precipitatedalloys")]
    PrecipitatedAlloys,

    #[serde(alias = "thermicalloys")]
    ThermicAlloys,

    #[serde(alias = "militarygradealloys")]
    MilitaryGradeAlloys,


    #[serde(alias = "heatconductionwiring")]
    HeatConductionWiring,

    #[serde(alias = "heatdispersionplate")]
    HeatDispersionPlate,

    #[serde(alias = "heatexchangers")]
    HeatExchangers,

    #[serde(alias = "heatvanes")]
    HeatVanes,

    #[serde(alias = "protoheatradiators")]
    ProtoHeatRadiators,


    #[serde(alias = "basicconductors")]
    BasicConductors,

    #[serde(alias = "conductivecomponents")]
    ConductiveComponents,

    #[serde(alias = "conductiveceramics")]
    ConductiveCeramics,

    #[serde(alias = "conductivepolymers")]
    ConductivePolymers,

    #[serde(alias = "biotechconductors")]
    BiotechConductors,


    #[serde(alias = "mechanicalscrap")]
    MechanicalScrap,

    #[serde(alias = "mechanicalequipment")]
    MechanicalEquipment,

    #[serde(alias = "mechanicalcomponents")]
    MechanicalComponents,

    #[serde(alias = "configurablecomponents")]
    ConfigurableComponents,

    #[serde(alias = "improvisedcomponents")]
    ImprovisedComponents,


    #[serde(alias = "gridresistors")]
    GridResistors,

    #[serde(alias = "hybridcapacitors")]
    HybridCapacitors,

    #[serde(alias = "electrochemicalarrays")]
    ElectrochemicalArrays,

    #[serde(alias = "polymercapacitors")]
    PolymerCapacitors,

    #[serde(alias = "militarysupercapacitors")]
    MilitarySupercapacitors,


    #[serde(alias = "wornshieldemitters")]
    WornShieldEmitters,

    #[serde(alias = "shieldemitters")]
    ShieldEmitters,

    #[serde(alias = "shieldingsensors")]
    ShieldingSensors,

    #[serde(alias = "compoundshielding")]
    CompoundShielding,

    #[serde(alias = "imperialshielding")]
    ImperialShielding,


    #[serde(alias = "compactcomposites")]
    CompactComposites,

    #[serde(alias = "filamentcomposites")]
    FilamentComposites,

    #[serde(alias = "highdensitycomposites")]
    HighDensityComposites,

    #[serde(alias = "fedproprietarycomposites")]
    ProprietaryComposites,

    #[serde(alias = "fedcorecomposites")]
    CoreDynamicsComposites,


    #[serde(alias = "crystalshards")]
    CrystalShards,

    #[serde(alias = "uncutfocuscrystals")]
    FlawedFocusCrystals,

    #[serde(alias = "focuscrystals")]
    FocusCrystals,

    #[serde(alias = "refinedfocuscrystals")]
    RefinedFocusCrystals,

    #[serde(alias = "exquisitefocuscrystals")]
    ExquisiteFocusCrystals,


    #[serde(alias = "salvagedalloys")]
    SalvagedAlloys,

    #[serde(alias = "galvanisingalloys")]
    GalvanisingAlloys,

    #[serde(alias = "phasealloys")]
    PhaseAlloys,

    #[serde(alias = "protolightalloys")]
    ProtoLightAlloys,

    #[serde(alias = "protoradiolicalloys")]
    ProtoRadiolicAlloys,


    #[serde(alias = "hardenedsurfacefragments")]
    HardenedSurfaceFragments,

    #[serde(alias = "tg_causticshard")]
    CausticShard,

    #[serde(alias = "tacticalcorechip")]
    TacticalCoreChip,

    #[serde(alias = "thargoidcarapace")]
    ThargoidCarapace,

    #[serde(alias = "biomechanicalconduits")]
    BioMechanicalConduits,

    #[serde(alias = "tg_causticgeneratorparts")]
    CorrosiveMechanisms,

    #[serde(alias = "phasingmembraneresidue")]
    PhasingMembraneResidue,

    #[serde(alias = "thargoidenergycell")]
    ThargoidEnergyCell,

    #[serde(alias = "tg_wreckagecomponents")]
    WreckageComponents,

    #[serde(alias = "tg_causticcrystal")]
    CausticCrystal,

    #[serde(alias = "thargoidtechnologicalcomponents")]
    ThargoidTechnologicalComponents,

    #[serde(alias = "weaponparts")]
    WeaponParts,

    #[serde(alias = "heatexposurespecimen")]
    HeatExposureSpecimen,

    #[serde(alias = "tg_propulsionelement")]
    PropulsionElements,

    #[serde(alias = "unknownenergysource")]
    SensorFragment,

    #[serde(alias = "thargoidorganiccircuitry")]
    ThargoidOrganicCircuitry,


    #[serde(alias = "guardian_powercell")]
    GuardianPowerCell,

    #[serde(alias = "guardian_sentinel_wreckagecomponents")]
    GuardianWreckageComponents,

    #[serde(alias = "guardian_powerconduit")]
    GuardianPowerConduit,

    #[serde(alias = "guardian_sentinel_weaponparts")]
    GuardianSentinelWeaponParts,

    #[serde(alias = "guardian_techcomponent")]
    GuardianTechnologyComponent,

    // Encoded
    #[serde(alias = "scrambledemissiondata")]
    ExceptionScrambledEmissionData,

    #[serde(alias = "archivedemissiondata")]
    IrregularEmissionData,

    #[serde(alias = "emissiondata")]
    UnexpectedEmissionData,

    #[serde(alias = "decodedemissiondata")]
    DecodedEmissionData,

    #[serde(alias = "compactemissionsdata")]
    AbnormalCompactEmissionData,


    #[serde(alias = "disruptedwakeechoes")]
    AtypicalDisruptedWakeEchoes,

    #[serde(alias = "fsdtelemetry")]
    AnomalousFSDTelemetry,

    #[serde(alias = "wakesolutions")]
    StrangeWakeSolutions,

    #[serde(alias = "hyperspacetrajectories")]
    EccentricHyperspaceTrajectories,

    #[serde(alias = "dataminedwake")]
    DataminedWakeExceptions,


    #[serde(alias = "shieldcyclerecordings")]
    DistortedShieldCycleRecordings,

    #[serde(alias = "shieldsoakanalysis")]
    InconsistentShieldSoakAnalysis,

    #[serde(alias = "shielddensityreports")]
    UntypicalShieldScans,

    #[serde(alias = "shieldpatternanalysis")]
    AberrantShieldPatternAnalysis,

    #[serde(alias = "shieldfrequencydata")]
    PeculiarShieldFrequencyData,


    #[serde(alias = "encryptedfiles")]
    UnusualEncryptedFiles,

    #[serde(alias = "encryptioncodes")]
    TaggedEncryptionCodes,

    #[serde(alias = "symmetrickeys")]
    OpenSymmetricKeys,

    #[serde(alias = "encryptionarchives")]
    AtypicalEncryptionArchives,

    #[serde(alias = "adaptiveencryptors")]
    AdaptiveEncryptorsCapture,


    #[serde(alias = "bulkscandata")]
    AnomalousBulkScanData,

    #[serde(alias = "scanarchives")]
    UnidentifiedScanArchives,

    #[serde(alias = "scandatabanks")]
    ClassifiedScanDatabanks,

    #[serde(alias = "EncodedScanData", alias = "encodedscandata")]
    DivergentScanData,

    #[serde(alias = "classifiedscanfragment")]
    ClassifiedScanFragment,


    #[serde(alias = "legacyfirmware")]
    SpecializedLegacyFirmware,

    #[serde(alias = "consumerfirmware")]
    ModifiedConsumerFirmware,

    #[serde(alias = "industrialfirmware")]
    CrackedIndustrialFirmware,

    #[serde(alias = "securityfirmware")]
    SecurityFirmwarePatch,

    #[serde(alias = "EmbeddedFirmware", alias = "embeddedfirmware")]
    ModifiedEmbeddedFirmware,


    #[serde(alias = "tg_structuraldata")]
    ThargoidStructuralData,

    #[serde(alias = "tg_shutdowndata")]
    MassiveEnergySurgeAnalytics,

    #[serde(alias = "tg_shipflightdata")]
    ShipFlightData,

    #[serde(alias = "tg_shipsystemsdata")]
    ShipSystemsData,

    #[serde(alias = "tg_interdictiondata")]
    ThargoidInterdictionTelemetry,

    #[serde(alias = "tg_compositiondata")]
    ThargoidMaterialCompositionData,

    #[serde(alias = "unknownshipsignature")]
    ThargoidShipSignature,

    #[serde(alias = "thargoidresiduedata")]
    ThargoidResidueData,

    #[serde(alias = "thargoidwakedata")]
    ThargoidWakeData,

    #[serde(alias = "ancienthistoricaldata")]
    PatternGammaObeliskData,

    #[serde(alias = "ancientculturaldata")]
    PatternBetaObeliskData,

    #[serde(alias = "ancientbiologicaldata")]
    PatternAlphaObeliskData,

    #[serde(alias = "ancientlanguagedata")]
    PatternDeltaObeliskData,

    #[serde(alias = "ancienttechnologicaldata")]
    PatternEpsilonObeliskData,

    #[serde(alias = "guardian_moduleblueprint")]
    GuardianModuleBlueprintFragment,

    #[serde(alias = "guardian_vesselblueprint")]
    GuardianVesselBlueprintFragment,

    #[serde(alias = "guardian_weaponblueprint")]
    GuardianWeaponBlueprintFragment,
    //
    // // Odyssey item
    // GMeds,
    //
    // // Odyssey data
    // ExtractionYieldData,
    // StellarActivityLogs,

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}
