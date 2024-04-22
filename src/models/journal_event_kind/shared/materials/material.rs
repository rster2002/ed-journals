use std::str::FromStr;
use serde::Deserialize;
use thiserror::Error;
use crate::from_str_deserialize_impl;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Material {
    // Raw
    #[serde(rename = "carbon")]
    Carbon,

    #[serde(rename = "vanadium")]
    Vanadium,

    #[serde(rename = "niobium")]
    Niobium,

    #[serde(rename = "yttrium")]
    Yttrium,


    #[serde(rename = "phosphorus")]
    Phosphorus,

    #[serde(rename = "chromium")]
    Chromium,

    #[serde(rename = "molybdenum")]
    Molybdenum,

    #[serde(rename = "technetium")]
    Technetium,


    #[serde(rename = "sulphur")]
    Sulphur,

    #[serde(rename = "manganese")]
    Manganese,

    #[serde(rename = "cadmium")]
    Cadmium,

    #[serde(rename = "ruthenium")]
    Ruthenium,


    #[serde(rename = "iron")]
    Iron,

    #[serde(rename = "zinc")]
    Zinc,

    #[serde(rename = "tin")]
    Tin,

    #[serde(rename = "selenium")]
    Selenium,


    #[serde(rename= "nickel")]
    Nickel,

    #[serde(rename= "germanium")]
    Germanium,

    #[serde(rename= "tungsten")]
    Tungsten,

    #[serde(rename= "tellurium")]
    Tellurium,


    #[serde(rename = "rhenium")]
    Rhenium,

    #[serde(rename = "arsenic")]
    Arsenic,

    #[serde(rename = "mercury")]
    Mercury,

    #[serde(rename = "polonium")]
    Polonium,


    #[serde(rename = "lead")]
    Lead,

    #[serde(rename = "zirconium")]
    Zirconium,

    #[serde(rename = "boron")]
    Boron,

    #[serde(rename = "antimony")]
    Antimony,

    // Manufactured
    #[serde(rename = "chemicalstorageunits")]
    ChemicalStorageUnits,

    #[serde(rename = "chemicalprocessors")]
    ChemicalProcessors,

    #[serde(rename = "chemicaldistillery")]
    ChemicalDistillery,

    #[serde(rename = "chemicalmanipulators")]
    ChemicalManipulators,

    #[serde(rename = "pharmaceuticalisolators")]
    PharmaceuticalIsolators,


    #[serde(rename = "temperedalloys")]
    TemperedAlloys,

    #[serde(rename = "heatresistantceramics")]
    HeatResistantCeramics,

    #[serde(rename = "precipitatedalloys")]
    PrecipitatedAlloys,

    #[serde(rename = "thermicalloys")]
    ThermicAlloys,

    #[serde(rename = "militarygradealloys")]
    MilitaryGradeAlloys,


    #[serde(rename = "heatconductionwiring")]
    HeatConductionWiring,

    #[serde(rename = "heatdispersionplate")]
    HeatDispersionPlate,

    #[serde(rename = "heatexchangers")]
    HeatExchangers,

    #[serde(rename = "heatvanes")]
    HeatVanes,

    #[serde(rename = "protoheatradiators")]
    ProtoHeatRadiators,


    #[serde(rename = "basicconductors")]
    BasicConductors,

    #[serde(rename = "conductivecomponents")]
    ConductiveComponents,

    #[serde(rename = "conductiveceramics")]
    ConductiveCeramics,

    #[serde(rename = "conductivepolymers")]
    ConductivePolymers,

    #[serde(rename = "biotechconductors")]
    BiotechConductors,


    #[serde(rename = "mechanicalscrap")]
    MechanicalScrap,

    #[serde(rename = "mechanicalequipment")]
    MechanicalEquipment,

    #[serde(rename = "mechanicalcomponents")]
    MechanicalComponents,

    #[serde(rename = "configurablecomponents")]
    ConfigurableComponents,

    #[serde(rename = "improvisedcomponents")]
    ImprovisedComponents,


    #[serde(rename = "gridresistors")]
    GridResistors,

    #[serde(rename = "hybridcapacitors")]
    HybridCapacitors,

    #[serde(rename = "electrochemicalarrays")]
    ElectrochemicalArrays,

    #[serde(rename = "polymercapacitors")]
    PolymerCapacitors,

    #[serde(rename = "militarysupercapacitors")]
    MilitarySupercapacitors,


    #[serde(rename = "wornshieldemitters")]
    WornShieldEmitters,

    #[serde(rename = "shieldemitters")]
    ShieldEmitters,

    #[serde(rename = "shieldingsensors")]
    ShieldingSensors,

    #[serde(rename = "compoundshielding")]
    CompoundShielding,

    #[serde(rename = "imperialshielding")]
    ImperialShielding,


    #[serde(rename = "compactcomposites")]
    CompactComposites,

    #[serde(rename = "filamentcomposites")]
    FilamentComposites,

    #[serde(rename = "highdensitycomposites")]
    HighDensityComposites,

    #[serde(rename = "fedproprietarycomposites")]
    ProprietaryComposites,

    #[serde(rename = "fedcorecomposites")]
    CoreDynamicsComposites,


    #[serde(rename = "crystalshards")]
    CrystalShards,

    #[serde(rename = "uncutfocuscrystals")]
    FlawedFocusCrystals,

    #[serde(rename = "focuscrystals")]
    FocusCrystals,

    #[serde(rename = "refinedfocuscrystals")]
    RefinedFocusCrystals,

    #[serde(rename = "exquisitefocuscrystals")]
    ExquisiteFocusCrystals,


    #[serde(rename = "salvagedalloys")]
    SalvagedAlloys,

    #[serde(rename = "galvanisingalloys")]
    GalvanisingAlloys,

    #[serde(rename = "phasealloys")]
    PhaseAlloys,

    #[serde(rename = "protolightalloys")]
    ProtoLightAlloys,

    #[serde(rename = "protoradiolicalloys")]
    ProtoRadiolicAlloys,


    #[serde(rename = "hardenedsurfacefragments")]
    HardenedSurfaceFragments,

    #[serde(rename = "tg_causticshard")]
    CausticShard,

    #[serde(rename = "tacticalcorechip")]
    TacticalCoreChip,

    #[serde(rename = "thargoidcarapace")]
    ThargoidCarapace,

    #[serde(rename = "biomechanicalconduits")]
    BioMechanicalConduits,

    #[serde(rename = "tg_causticgeneratorparts")]
    CorrosiveMechanisms,

    #[serde(rename = "phasingmembraneresidue")]
    PhasingMembraneResidue,

    #[serde(rename = "thargoidenergycell")]
    ThargoidEnergyCell,

    #[serde(rename = "tg_wreckagecomponents")]
    WreckageComponents,

    #[serde(rename = "tg_causticcrystal")]
    CausticCrystal,

    #[serde(rename = "thargoidtechnologicalcomponents")]
    ThargoidTechnologicalComponents,

    #[serde(rename = "weaponparts")]
    WeaponParts,

    #[serde(rename = "heatexposurespecimen")]
    HeatExposureSpecimen,

    #[serde(rename = "tg_propulsionelement")]
    PropulsionElements,

    #[serde(rename = "unknownenergysource")]
    SensorFragment,

    #[serde(rename = "thargoidorganiccircuitry")]
    ThargoidOrganicCircuitry,


    #[serde(rename = "guardian_powercell")]
    GuardianPowerCell,

    #[serde(rename = "guardian_sentinel_wreckagecomponents")]
    GuardianWreckageComponents,

    #[serde(rename = "guardian_powerconduit")]
    GuardianPowerConduit,

    #[serde(rename = "guardian_sentinel_weaponparts")]
    GuardianSentinelWeaponParts,

    #[serde(rename = "guardian_techcomponent")]
    GuardianTechnologyComponent,

    // Encoded
    #[serde(rename = "scrambledemissiondata")]
    ExceptionScrambledEmissionData,

    #[serde(rename = "archivedemissiondata")]
    IrregularEmissionData,

    #[serde(rename = "emissiondata")]
    UnexpectedEmissionData,

    #[serde(rename = "decodedemissiondata")]
    DecodedEmissionData,

    #[serde(rename = "compactemissionsdata")]
    AbnormalCompactEmissionData,


    #[serde(rename = "disruptedwakeechoes")]
    AtypicalDisruptedWakeEchoes,

    #[serde(rename = "fsdtelemetry")]
    AnomalousFSDTelemetry,

    #[serde(rename = "wakesolutions")]
    StrangeWakeSolutions,

    #[serde(rename = "hyperspacetrajectories")]
    EccentricHyperspaceTrajectories,

    #[serde(rename = "dataminedwake")]
    DataminedWakeExceptions,


    #[serde(rename = "shieldcyclerecordings")]
    DistortedShieldCycleRecordings,

    #[serde(rename = "shieldsoakanalysis")]
    InconsistentShieldSoakAnalysis,

    #[serde(rename = "shielddensityreports")]
    UntypicalShieldScans,

    #[serde(rename = "shieldpatternanalysis")]
    AberrantShieldPatternAnalysis,

    #[serde(rename = "shieldfrequencydata")]
    PeculiarShieldFrequencyData,


    #[serde(rename = "encryptedfiles")]
    UnusualEncryptedFiles,

    #[serde(rename = "encryptioncodes")]
    TaggedEncryptionCodes,

    #[serde(rename = "symmetrickeys")]
    OpenSymmetricKeys,

    #[serde(rename = "encryptionarchives")]
    AtypicalEncryptionArchives,

    #[serde(rename = "adaptiveencryptors")]
    AdaptiveEncryptorsCapture,


    #[serde(rename = "bulkscandata")]
    AnomalousBulkScanData,

    #[serde(rename = "scanarchives")]
    UnidentifiedScanArchives,

    #[serde(rename = "scandatabanks")]
    ClassifiedScanDatabanks,

    #[serde(rename = "divergentscandata")]
    DivergentScanData,

    #[serde(rename = "classifiedscanfragment")]
    ClassifiedScanFragment,


    #[serde(rename = "legacyfirmware")]
    SpecializedLegacyFirmware,

    #[serde(rename = "consumerfirmware")]
    ModifiedConsumerFirmware,

    #[serde(rename = "industrialfirmware")]
    CrackedIndustrialFirmware,

    #[serde(rename = "securityfirmware")]
    SecurityFirmwarePatch,

    #[serde(rename = "embeddedfirmware")]
    ModifiedEmbeddedFirmware,


    #[serde(rename = "tg_structuraldata")]
    ThargoidStructuralData,

    #[serde(rename = "tg_shutdowndata")]
    MassiveEnergySurgeAnalytics,

    #[serde(rename = "tg_shipflightdata")]
    ShipFlightData,

    #[serde(rename = "tg_shipsystemsdata")]
    ShipSystemsData,

    #[serde(rename = "tg_interdictiondata")]
    ThargoidInterdictionTelemetry,

    #[serde(rename = "tg_compositiondata")]
    ThargoidMaterialCompositionData,

    #[serde(rename = "unknownshipsignature")]
    ThargoidShipSignature,

    #[serde(rename = "thargoidresiduedata")]
    ThargoidResidueData,

    #[serde(rename = "thargoidwakedata")]
    ThargoidWakeData,

    #[serde(rename = "ancienthistoricaldata")]
    PatternGammaObeliskData,

    #[serde(rename = "ancientculturaldata")]
    PatternBetaObeliskData,

    #[serde(rename = "ancientbiologicaldata")]
    PatternAlphaObeliskData,

    #[serde(rename = "ancientlanguagedata")]
    PatternDeltaObeliskData,

    #[serde(rename = "ancienttechnologicaldata")]
    PatternEpsilonObeliskData,

    #[serde(rename = "guardian_moduleblueprint")]
    GuardianModuleBlueprintFragment,

    #[serde(rename = "guardian_vesselblueprint")]
    GuardianVesselBlueprintFragment,

    #[serde(rename = "guardian_weaponblueprint")]
    GuardianWeaponBlueprintFragment,

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}
