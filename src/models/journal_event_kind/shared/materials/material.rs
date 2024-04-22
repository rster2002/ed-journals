use std::str::FromStr;
use thiserror::Error;
use crate::from_str_deserialize_impl;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Material {
    // Raw
    Carbon,
    Vanadium,
    Niobium,
    Yttrium,

    Phosphorus,
    Chromium,
    Molybdenum,
    Technetium,

    Sulphur,
    Manganese,
    Cadmium,
    Ruthenium,

    Iron,
    Zinc,
    Tin,
    Selenium,

    Nickel,
    Germanium,
    Tungsten,
    Tellurium,

    Rhenium,
    Arsenic,
    Mercury,
    Polonium,

    Lead,
    Zirconium,
    Boron,
    Antimony,

    // Manufactured
    ChemicalStorageUnits,
    ChemicalProcessors,
    ChemicalDistillery,
    ChemicalManipulators,
    PharmaceuticalIsolators,

    TemperedAlloys,
    HeatResistantCeramics,
    PrecipitatedAlloys,
    ThermicAlloys,
    MilitaryGradeAlloys,

    HeatConductionWiring,
    HeatDispersionPlate,
    HeatExchangers,
    HeatVanes,
    ProtoHeatRadiators,

    BasicConductors,
    ConductiveComponents,
    ConductiveCeramics,
    ConductivePolymers,
    BiotechConductors,

    MechanicalScrap,
    MechanicalEquipment,
    MechanicalComponents,
    ConfigurableComponents,
    ImprovisedComponents,

    GridResistors,
    HybridCapacitors,
    ElectrochemicalArrays,
    PolymerCapacitors,
    MilitarySupercapacitors,

    WornShieldEmitters,
    ShieldEmitters,
    ShieldingSensors,
    CompoundShielding,
    ImperialShielding,

    CompactComposites,
    FilamentComposites,
    HighDensityComposites,
    ProprietaryComposites,
    CoreDynamicsComposites,

    CrystalShards,
    FlawedFocusCrystals,
    FocusCrystals,
    RefinedFocusCrystals,
    ExquisiteFocusCrystals,

    SalvagedAlloys,
    GalvanisingAlloys,
    PhaseAlloys,
    ProtoLightAlloys,
    ProtoRadiolicAlloys,

    HardenedSurfaceFragments,
    CausticShard,
    TacticalCoreChip,
    ThargoidCarapace,
    BioMechanicalConduits,
    CorrosiveMechanisms,
    PhasingMembraneResidue,
    ThargoidEnergyCell,
    WreckageComponents,
    CausticCrystal,
    ThargoidTechnologicalComponents,
    WeaponParts,
    HeatExposureSpecimen,
    PropulsionElements,
    SensorFragment,
    ThargoidOrganicCircuitry,

    GuardianPowerCell,
    GuardianWreckageComponents,
    GuardianPowerConduit,
    GuardianSentinelWeaponParts,
    GuardianTechnologyComponent,

    // Encoded
    ExceptionScrambledEmissionData,
    IrregularEmissionData,
    UnexpectedEmissionData,
    DecodedEmissionData,
    AbnormalCompactEmissionData,

    AtypicalDisruptedWakeEchoes,
    AnomalousFSDTelemetry,
    StrangeWakeSolutions,
    EccentricHyperspaceTrajectories,
    DataminedWakeExceptions,

    DistortedShieldCycleRecordings,
    InconsistentShieldSoakAnalysis,
    UntypicalShieldScans,
    AberrantShieldPatternAnalysis,
    PeculiarShieldFrequencyData,

    UnusualEncryptedFiles,
    TaggedEncryptionCodes,
    OpenSymmetricKeys,
    AtypicalEncryptionArchives,
    AdaptiveEncryptorsCapture,

    AnomalousBulkScanData,
    UnidentifiedScanArchives,
    ClassifiedScanDatabanks,
    DivergentScanData,
    ClassifiedScanFragment,

    SpecializedLegacyFirmware,
    ModifiedConsumerFirmware,
    CrackedIndustrialFirmware,
    SecurityFirmwarePatch,
    ModifiedEmbeddedFirmware,

    ThargoidStructuralData,
    MassiveEnergySurgeAnalytics,
    ShipFlightData,
    ShipSystemsData,
    ThargoidInterdictionTelemetry,
    ThargoidMaterialCompositionData,
    ThargoidShipSignature,
    ThargoidResidueData,
    ThargoidWakeData,

    PatternGammaObeliskData,
    PatternBetaObeliskData,
    PatternAlphaObeliskData,
    PatternDeltaObeliskData,
    PatternEpsilonObeliskData,
    GuardianModuleBlueprintFragment,
    GuardianVesselBlueprintFragment,
    GuardianWeaponBlueprintFragment,

    #[cfg(not(feature = "strict"))]
    Unknown(String),
}

#[derive(Debug, Error)]
pub enum MaterialParseError {
    #[error("Unknown material: '{0}'")]
    UnknownMaterial(String),
}

impl FromStr for Material {
    type Err = MaterialParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // Raw
            "carbon" => Ok(Material::Carbon),
            "vanadium" => Ok(Material::Vanadium),
            "niobium" => Ok(Material::Niobium),
            "yttrium" => Ok(Material::Yttrium),

            "phosphorus" => Ok(Material::Phosphorus),
            "chromium" => Ok(Material::Chromium),
            "molybdenum" => Ok(Material::Molybdenum),
            "technetium" => Ok(Material::Technetium),

            "sulphur" => Ok(Material::Sulphur),
            "manganese" => Ok(Material::Manganese),
            "cadmium" => Ok(Material::Cadmium),
            "ruthenium" => Ok(Material::Ruthenium),

            "iron" => Ok(Material::Iron),
            "zinc" => Ok(Material::Zinc),
            "tin" => Ok(Material::Tin),
            "selenium" => Ok(Material::Selenium),

            "nickel" => Ok(Material::Nickel),
            "germanium" => Ok(Material::Germanium),
            "tungsten" => Ok(Material::Tungsten),
            "tellurium" => Ok(Material::Tellurium),

            "rhenium" => Ok(Material::Rhenium),
            "arsenic" => Ok(Material::Arsenic),
            "mercury" => Ok(Material::Mercury),
            "polonium" => Ok(Material::Polonium),

            "lead" => Ok(Material::Lead),
            "zirconium" => Ok(Material::Zirconium),
            "boron" => Ok(Material::Boron),
            "antimony" => Ok(Material::Antimony),

            // Manufactured
            "chemicalstorageunits" => Ok(Material::ChemicalStorageUnits),
            "chemicalprocessors" => Ok(Material::ChemicalProcessors),
            "chemicaldistillery" => Ok(Material::ChemicalDistillery),
            "chemicalmanipulators" => Ok(Material::ChemicalManipulators),
            "pharmaceuticalisolators" => Ok(Material::PharmaceuticalIsolators),

            "temperedalloys" => Ok(Material::TemperedAlloys),
            "heatresistantceramics" => Ok(Material::HeatResistantCeramics),
            "precipitatedalloys" => Ok(Material::PrecipitatedAlloys),
            "thermicalloys" => Ok(Material::ThermicAlloys),
            "militarygradealloys" => Ok(Material::MilitaryGradeAlloys),

            "heatconductionwiring" => Ok(Material::HeatConductionWiring),
            "heatdispersionplate" => Ok(Material::HeatDispersionPlate),
            "heatexchangers" => Ok(Material::HeatExchangers),
            "heatvanes" => Ok(Material::HeatVanes),
            "protoheatradiators" => Ok(Material::ProtoHeatRadiators),

            "basicconductors" => Ok(Material::BasicConductors),
            "conductivecomponents" => Ok(Material::ConductiveComponents),
            "conductiveceramics" => Ok(Material::ConductiveCeramics),
            "conductivepolymers" => Ok(Material::ConductivePolymers),
            "biotechconductors" => Ok(Material::BiotechConductors),

            "mechanicalscrap" => Ok(Material::MechanicalScrap),
            "mechanicalequipment" => Ok(Material::MechanicalEquipment),
            "mechanicalcomponents" => Ok(Material::MechanicalComponents),
            "configurablecomponents" => Ok(Material::ConfigurableComponents),
            "improvisedcomponents" => Ok(Material::ImprovisedComponents),

            "gridresistors" => Ok(Material::GridResistors),
            "hybridcapacitors" => Ok(Material::HybridCapacitors),
            "electrochemicalarrays" => Ok(Material::ElectrochemicalArrays),
            "polymercapacitors" => Ok(Material::PolymerCapacitors),
            "militarysupercapacitors" => Ok(Material::MilitarySupercapacitors),

            "wornshieldemitters" => Ok(Material::WornShieldEmitters),
            "shieldemitters" => Ok(Material::ShieldEmitters),
            "shieldingsensors" => Ok(Material::ShieldingSensors),
            "compoundshielding" => Ok(Material::CompoundShielding),
            "imperialshielding" => Ok(Material::ImperialShielding),

            "compactcomposites" => Ok(Material::CompactComposites),
            "filamentcomposites" => Ok(Material::FilamentComposites),
            "highdensitycomposites" => Ok(Material::HighDensityComposites),
            "proprietarycomposites" => Ok(Material::ProprietaryComposites),
            "fedproprietarycomposites" => Ok(Material::ProprietaryComposites),
            "coredynamicscomposites" => Ok(Material::CoreDynamicsComposites),
            "fedcorecomposites" => Ok(Material::CoreDynamicsComposites),

            "crystalshards" => Ok(Material::CrystalShards),
            "flawedfocuscrystals" => Ok(Material::FlawedFocusCrystals),
            "uncutfocuscrystals" => Ok(Material::FlawedFocusCrystals),
            "focuscrystals" => Ok(Material::FocusCrystals),
            "refinedfocuscrystals" => Ok(Material::RefinedFocusCrystals),
            "exquisitefocuscrystals" => Ok(Material::ExquisiteFocusCrystals),

            "salvagedalloys" => Ok(Material::SalvagedAlloys),
            "galvanisingalloys" => Ok(Material::GalvanisingAlloys),
            "phasealloys" => Ok(Material::PhaseAlloys),
            "protolightalloys" => Ok(Material::ProtoLightAlloys),
            "protoradiolicalloys" => Ok(Material::ProtoRadiolicAlloys),

            "hardenedsurfacefragments" => Ok(Material::HardenedSurfaceFragments),
            "causticshard" => Ok(Material::CausticShard),
            "tg_causticshard" => Ok(Material::CausticShard),
            "tacticalcorechip" => Ok(Material::TacticalCoreChip),
            "thargoidcarapace" => Ok(Material::ThargoidCarapace),
            "biomechanicalconduits" => Ok(Material::BioMechanicalConduits),
            "corrosivemechanisms" => Ok(Material::CorrosiveMechanisms),
            "tg_causticgeneratorparts" => Ok(Material::CorrosiveMechanisms),
            "phasingmembraneresidue" => Ok(Material::PhasingMembraneResidue),
            "thargoidenergycell" => Ok(Material::ThargoidEnergyCell),
            "wreckagecomponents" => Ok(Material::WreckageComponents),
            "tg_wreckagecomponents" => Ok(Material::WreckageComponents),
            "causticcrystal" => Ok(Material::CausticCrystal),
            "tg_causticcrystal" => Ok(Material::CausticCrystal),
            "thargoidtechnologicalcomponents" => Ok(Material::ThargoidTechnologicalComponents),
            "weaponparts" => Ok(Material::WeaponParts),
            "heatexposurespecimen" => Ok(Material::HeatExposureSpecimen),
            "propulsionelements" => Ok(Material::PropulsionElements),
            "tg_propulsionelement" => Ok(Material::PropulsionElements),
            "sensorfragment" => Ok(Material::SensorFragment),
            "unknownenergysource" => Ok(Material::SensorFragment),
            "thargoidorganiccircuitry" => Ok(Material::ThargoidOrganicCircuitry),

            "guardianpowercell" => Ok(Material::GuardianPowerCell),
            "guardian_powercell" => Ok(Material::GuardianPowerCell),
            "guardianwreckagecomponents" => Ok(Material::GuardianWreckageComponents),
            "guardian_sentinel_wreckagecomponents" => Ok(Material::GuardianWreckageComponents),
            "guardianpowerconduit" => Ok(Material::GuardianPowerConduit),
            "guardian_powerconduit" => Ok(Material::GuardianPowerConduit),
            "guardiansentinelweaponparts" => Ok(Material::GuardianSentinelWeaponParts),
            "guardian_sentinel_weaponparts" => Ok(Material::GuardianSentinelWeaponParts),
            "guardiantechnologycomponent" => Ok(Material::GuardianTechnologyComponent),
            "guardian_techcomponent" => Ok(Material::GuardianTechnologyComponent),

            // Encoded
            "exceptionscrambledemissiondata" => Ok(Material::ExceptionScrambledEmissionData),
            "scrambledemissiondata" => Ok(Material::ExceptionScrambledEmissionData),
            "irregularemissiondata" => Ok(Material::IrregularEmissionData),
            "archivedemissiondata" => Ok(Material::IrregularEmissionData),
            "unexpectedemissiondata" => Ok(Material::UnexpectedEmissionData),
            "emissiondata" => Ok(Material::UnexpectedEmissionData),
            "decodedemissiondata" => Ok(Material::DecodedEmissionData),
            "abnormalcompactemissiondata" => Ok(Material::AbnormalCompactEmissionData),
            "compactemissionsdata" => Ok(Material::AbnormalCompactEmissionData),

            "atypicaldisruptedwakeechoes" => Ok(Material::AtypicalDisruptedWakeEchoes),
            "disruptedwakeechoes" => Ok(Material::AtypicalDisruptedWakeEchoes),
            "anomalousfsdtelemetry" => Ok(Material::AnomalousFSDTelemetry),
            "fsdtelemetry" => Ok(Material::AnomalousFSDTelemetry),
            "strangewakesolutions" => Ok(Material::StrangeWakeSolutions),
            "wakesolutions" => Ok(Material::StrangeWakeSolutions),
            "eccentrichyperspacetrajectories" => Ok(Material::EccentricHyperspaceTrajectories),
            "hyperspacetrajectories" => Ok(Material::EccentricHyperspaceTrajectories),
            "dataminedwakeexceptions" => Ok(Material::DataminedWakeExceptions),
            "dataminedwake" => Ok(Material::DataminedWakeExceptions),

            "distortedshieldcyclerecordings" => Ok(Material::DistortedShieldCycleRecordings),
            "shieldcyclerecordings" => Ok(Material::DistortedShieldCycleRecordings),
            "inconsistentshieldsoakanalysis" => Ok(Material::InconsistentShieldSoakAnalysis),
            "shieldsoakanalysis" => Ok(Material::InconsistentShieldSoakAnalysis),
            "untypicalshieldscans" => Ok(Material::UntypicalShieldScans),
            "shielddensityreports" => Ok(Material::UntypicalShieldScans),
            "aberrantshieldpatternanalysis" => Ok(Material::AberrantShieldPatternAnalysis),
            "shieldpatternanalysis" => Ok(Material::AberrantShieldPatternAnalysis),
            "peculiarshieldfrequencydata" => Ok(Material::PeculiarShieldFrequencyData),
            "shieldfrequencydata" => Ok(Material::PeculiarShieldFrequencyData),

            "unusualencryptedfiles" => Ok(Material::UnusualEncryptedFiles),
            "encryptedfiles" => Ok(Material::UnusualEncryptedFiles),
            "taggedencryptioncodes" => Ok(Material::TaggedEncryptionCodes),
            "encryptioncodes" => Ok(Material::TaggedEncryptionCodes),
            "opensymmetrickeys" => Ok(Material::OpenSymmetricKeys),
            "symmetrickeys" => Ok(Material::OpenSymmetricKeys),
            "atypicalencryptionarchives" => Ok(Material::AtypicalEncryptionArchives),
            "encryptionarchives" => Ok(Material::AtypicalEncryptionArchives),
            "adaptiveencryptorscapture" => Ok(Material::AdaptiveEncryptorsCapture),
            "adaptiveencryptors" => Ok(Material::AdaptiveEncryptorsCapture),

            "anomalousbulkscandata" => Ok(Material::AnomalousBulkScanData),
            "bulkscandata" => Ok(Material::AnomalousBulkScanData),
            "unidentifiedscanarchives" => Ok(Material::UnidentifiedScanArchives),
            "scanarchives" => Ok(Material::UnidentifiedScanArchives),
            "classifiedscandatabanks" => Ok(Material::ClassifiedScanDatabanks),
            "scandatabanks" => Ok(Material::ClassifiedScanDatabanks),
            "divergentscandata" => Ok(Material::DivergentScanData),
            "classifiedscanfragment" => Ok(Material::ClassifiedScanFragment),

            "specializedlegacyfirmware" => Ok(Material::SpecializedLegacyFirmware),
            "legacyfirmware" => Ok(Material::SpecializedLegacyFirmware),
            "modifiedconsumerfirmware" => Ok(Material::ModifiedConsumerFirmware),
            "consumerfirmware" => Ok(Material::ModifiedConsumerFirmware),
            "crackedindustrialfirmware" => Ok(Material::CrackedIndustrialFirmware),
            "industrialfirmware" => Ok(Material::CrackedIndustrialFirmware),
            "securityfirmwarepatch" => Ok(Material::SecurityFirmwarePatch),
            "securityfirmware" => Ok(Material::SecurityFirmwarePatch),
            "modifiedembeddedfirmware" => Ok(Material::ModifiedEmbeddedFirmware),
            "embeddedfirmware" => Ok(Material::ModifiedEmbeddedFirmware),

            "thargoidstructuraldata" => Ok(Material::ThargoidStructuralData),
            "tg_structuraldata" => Ok(Material::ThargoidStructuralData),
            "massiveenergysurgeanalytics" => Ok(Material::MassiveEnergySurgeAnalytics),
            "tg_shutdowndata" => Ok(Material::MassiveEnergySurgeAnalytics),
            "shipflightdata" => Ok(Material::ShipFlightData),
            "tg_shipflightdata" => Ok(Material::ShipFlightData),
            "shipsystemsdata" => Ok(Material::ShipSystemsData),
            "tg_shipsystemsdata" => Ok(Material::ShipSystemsData),
            "thargoidinterdictiontelemetry" => Ok(Material::ThargoidInterdictionTelemetry),
            "tg_interdictiondata" => Ok(Material::ThargoidInterdictionTelemetry),
            "thargoidmaterialcompositiondata" => Ok(Material::ThargoidMaterialCompositionData),
            "tg_compositiondata" => Ok(Material::ThargoidMaterialCompositionData),
            "thargoidshipsignature" => Ok(Material::ThargoidShipSignature),
            "unknownshipsignature" => Ok(Material::ThargoidShipSignature),
            "thargoidresiduedata" => Ok(Material::ThargoidResidueData),
            "thargoidwakedata" => Ok(Material::ThargoidWakeData),

            "patterngammaobeliskdata" => Ok(Material::PatternGammaObeliskData),
            "ancienthistoricaldata" => Ok(Material::PatternGammaObeliskData),
            "patternbetaobeliskdata" => Ok(Material::PatternBetaObeliskData),
            "ancientculturaldata" => Ok(Material::PatternBetaObeliskData),
            "patternalphaobeliskdata" => Ok(Material::PatternAlphaObeliskData),
            "ancientbiologicaldata" => Ok(Material::PatternAlphaObeliskData),
            "patterndeltaobeliskdata" => Ok(Material::PatternDeltaObeliskData),
            "ancientlanguagedata" => Ok(Material::PatternDeltaObeliskData),
            "patternepsilonobeliskdata" => Ok(Material::PatternEpsilonObeliskData),
            "ancienttechnologicaldata" => Ok(Material::PatternEpsilonObeliskData),
            "guardianmoduleblueprintfragment" => Ok(Material::GuardianModuleBlueprintFragment),
            "guardian_moduleblueprint" => Ok(Material::GuardianModuleBlueprintFragment),
            "guardianvesselblueprintfragment" => Ok(Material::GuardianVesselBlueprintFragment),
            "guardian_vesselblueprint" => Ok(Material::GuardianVesselBlueprintFragment),
            "guardianweaponblueprintfragment" => Ok(Material::GuardianWeaponBlueprintFragment),
            "guardian_weaponblueprint" => Ok(Material::GuardianWeaponBlueprintFragment),

            #[cfg(not(feature = "strict"))]
            _ => Ok(Material::Unknown(s.to_string())),

            #[cfg(feature = "strict")]
            _ => Err(MaterialParseError::UnknownMaterial(s.to_string())),
        }
    }
}

from_str_deserialize_impl!(Material);
