use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::modules::models::materials::material_category::MaterialCategory;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
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

    #[serde(rename = "nickel")]
    Nickel,

    #[serde(rename = "germanium")]
    Germanium,

    #[serde(rename = "tungsten")]
    Tungsten,

    #[serde(rename = "tellurium")]
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

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}

impl Material {
    pub fn is_raw(&self) -> bool {
        matches!(self.into(), MaterialCategory::Raw)
    }

    pub fn is_manufactured(&self) -> bool {
        matches!(self.into(), MaterialCategory::Manufactured)
    }

    pub fn is_encoded(&self) -> bool {
        matches!(self.into(), MaterialCategory::Encoded)
    }
}

impl Display for Material {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                // Raw
                Material::Antimony => "Antimony",
                Material::Arsenic => "Arsenic",
                Material::Boron => "Boron",
                Material::Cadmium => "Cadmium",
                Material::Carbon => "Carbon",
                Material::Chromium => "Chromium",
                Material::Germanium => "Germanium",
                Material::Iron => "Iron",
                Material::Lead => "Lead",
                Material::Manganese => "Manganese",
                Material::Mercury => "Mercury",
                Material::Molybdenum => "Molybdenum",
                Material::Nickel => "Nickel",
                Material::Niobium => "Niobium",
                Material::Phosphorus => "Phosphorus",
                Material::Polonium => "Polonium",
                Material::Rhenium => "Rhenium",
                Material::Ruthenium => "Ruthenium",
                Material::Selenium => "Selenium",
                Material::Sulphur => "Sulphur",
                Material::Technetium => "Technetium",
                Material::Tellurium => "Tellurium",
                Material::Tin => "Tin",
                Material::Tungsten => "Tungsten",
                Material::Vanadium => "Vanadium",
                Material::Yttrium => "Yttrium",
                Material::Zinc => "Zinc",
                Material::Zirconium => "Zirconium",

                // Manufactured
                Material::BasicConductors => "Basic Conductors",
                Material::BioMechanicalConduits => "Bio-Mechanical Conduits",
                Material::BiotechConductors => "Biotech Conductors",
                Material::CausticCrystal => "Caustic Crystal",
                Material::CausticShard => "Caustic Shard",
                Material::ChemicalDistillery => "Chemical Distillery",
                Material::ChemicalManipulators => "Chemical Manipulators",
                Material::ChemicalProcessors => "Chemical Processors",
                Material::ChemicalStorageUnits => "Chemical Storage Units",
                Material::CompactComposites => "Compact Composites",
                Material::CompoundShielding => "Compound Shielding",
                Material::ConductiveCeramics => "Conductive Ceramics",
                Material::ConductiveComponents => "Conductive Components",
                Material::ConductivePolymers => "Conductive Polymers",
                Material::ConfigurableComponents => "Configurable Components",
                Material::CoreDynamicsComposites => "Core Dynamics Composites",
                Material::CorrosiveMechanisms => "Corrosive Mechanisms",
                Material::CrystalShards => "Crystal Shards",
                Material::ElectrochemicalArrays => "Electrochemical Arrays",
                Material::ExquisiteFocusCrystals => "Exquisite Focus Crystals",
                Material::FilamentComposites => "Filament Composites",
                Material::FlawedFocusCrystals => "Flawed Focus Crystals",
                Material::FocusCrystals => "Focus Crystals",
                Material::GalvanisingAlloys => "Galvanising Alloys",
                Material::GridResistors => "Grid Resistors",
                Material::GuardianPowerCell => "Guardian Power Cell",
                Material::GuardianPowerConduit => "Guardian Power Conduit",
                Material::GuardianSentinelWeaponParts => "Guardian Sentinel Weapon Parts",
                Material::GuardianTechnologyComponent => "Guardian Technology Component",
                Material::GuardianWreckageComponents => "Guardian Wreckage Components",
                Material::HardenedSurfaceFragments => "Hardened Surface Fragments",
                Material::HeatConductionWiring => "Heat Conduction Wiring",
                Material::HeatDispersionPlate => "Heat Dispersion Plate",
                Material::HeatExchangers => "Heat Exchangers",
                Material::HeatExposureSpecimen => "Heat Exposure Specimen",
                Material::HeatResistantCeramics => "Heat Resistant Ceramics",
                Material::HeatVanes => "Heat Vanes",
                Material::HighDensityComposites => "High Density Composites",
                Material::HybridCapacitors => "Hybrid Capacitors",
                Material::ImperialShielding => "Imperial Shielding",
                Material::ImprovisedComponents => "Improvised Components",
                Material::MechanicalComponents => "Mechanical Components",
                Material::MechanicalEquipment => "Mechanical Equipment",
                Material::MechanicalScrap => "Mechanical Scrap",
                Material::MilitaryGradeAlloys => "Military Grade Alloys",
                Material::MilitarySupercapacitors => "Military Supercapacitors",
                Material::PharmaceuticalIsolators => "Pharmaceutical Isolators",
                Material::PhaseAlloys => "Phase Alloys",
                Material::PhasingMembraneResidue => "Phasing Membrane Residue",
                Material::PolymerCapacitors => "Polymer Capacitors",
                Material::PrecipitatedAlloys => "Precipitated Alloys",
                Material::ProprietaryComposites => "Proprietary Composites",
                Material::PropulsionElements => "Propulsion Elements",
                Material::ProtoHeatRadiators => "Proto Heat Radiators",
                Material::ProtoLightAlloys => "Proto Light Alloys",
                Material::ProtoRadiolicAlloys => "Proto Radiolic Alloys",
                Material::RefinedFocusCrystals => "Refined Focus Crystals",
                Material::SalvagedAlloys => "Salvaged Alloys",
                Material::SensorFragment => "Sensor Fragment",
                Material::ShieldEmitters => "Shield Emitters",
                Material::ShieldingSensors => "Shielding Sensors",
                Material::TacticalCoreChip => "Tactical Core Chip",
                Material::TemperedAlloys => "Tempered Alloys",
                Material::ThargoidCarapace => "Thargoid Carapace",
                Material::ThargoidEnergyCell => "Thargoid Energy Cell",
                Material::ThargoidOrganicCircuitry => "Thargoid Organic Circuitry",
                Material::ThargoidTechnologicalComponents => "Thargoid Technological Components",
                Material::ThermicAlloys => "Thermic Alloys",
                Material::WeaponParts => "Weapon Parts",
                Material::WornShieldEmitters => "Worn Shield Emitters",
                Material::WreckageComponents => "Wreckage Components",

                // Encoded
                Material::AberrantShieldPatternAnalysis => "Aberrant Shield Pattern Analysis",
                Material::AbnormalCompactEmissionData => "Abnormal Compact Emissions Data",
                Material::AdaptiveEncryptorsCapture => "Adaptive Encryptors Capture",
                Material::AnomalousBulkScanData => "Anomalous Bulk Scan Data",
                Material::AnomalousFSDTelemetry => "Anomalous FSD Telemetry",
                Material::AtypicalDisruptedWakeEchoes => "Atypical Disrupted Wake Echoes",
                Material::AtypicalEncryptionArchives => "Atypical Encryption Archives",
                Material::ClassifiedScanDatabanks => "Classified Scan Databanks",
                Material::ClassifiedScanFragment => "Classified Scan Fragment",
                Material::CrackedIndustrialFirmware => "Cracked Industrial Firmware",
                Material::DataminedWakeExceptions => "Datamined Wake Exceptions",
                Material::DecodedEmissionData => "Decoded Emission Data",
                Material::DistortedShieldCycleRecordings => "Distorted Shield Cycle Recordings",
                Material::DivergentScanData => "Divergent Scan Data",
                Material::EccentricHyperspaceTrajectories => "Eccentric Hyperspace Trajectories",
                Material::ExceptionScrambledEmissionData => "Exceptional Scrambled Emission Data",
                Material::GuardianModuleBlueprintFragment => "Guardian Module Blueprint Fragment",
                Material::GuardianVesselBlueprintFragment => "Guardian Vessel Blueprint Fragment",
                Material::GuardianWeaponBlueprintFragment => "Guardian Weapon Blueprint Fragment",
                Material::InconsistentShieldSoakAnalysis => "Inconsistent Shield Soak Analysis",
                Material::IrregularEmissionData => "Irregular Emission Data",
                Material::MassiveEnergySurgeAnalytics => "Massive Energy Surge Analytics",
                Material::ModifiedConsumerFirmware => "Modified Consumer Firmware",
                Material::ModifiedEmbeddedFirmware => "Modified Embedded Firmware",
                Material::OpenSymmetricKeys => "Open Symmetric Keys",
                Material::PatternAlphaObeliskData => "Pattern Alpha Obelisk Data",
                Material::PatternBetaObeliskData => "Pattern Beta Obelisk Data",
                Material::PatternDeltaObeliskData => "Pattern Delta Obelisk Data",
                Material::PatternEpsilonObeliskData => "Pattern Epsilon Obelisk Data",
                Material::PatternGammaObeliskData => "Pattern Gamma Obelisk Data",
                Material::PeculiarShieldFrequencyData => "Peculiar Shield Frequency Data",
                Material::SecurityFirmwarePatch => "Security Firmware Patch",
                Material::ShipFlightData => "Ship Flight Data",
                Material::ShipSystemsData => "Ship Systems Data",
                Material::SpecializedLegacyFirmware => "Specialised Legacy Firmware",
                Material::StrangeWakeSolutions => "Strange Wake Solutions",
                Material::TaggedEncryptionCodes => "Tagged Encryption Codes",
                Material::ThargoidInterdictionTelemetry => "Thargoid Interdiction Telemetry",
                Material::ThargoidMaterialCompositionData => "Thargoid Material Composition Data",
                Material::ThargoidResidueData => "Thargoid Residue Data",
                Material::ThargoidShipSignature => "Thargoid Ship Signature",
                Material::ThargoidStructuralData => "Thargoid Structural Data",
                Material::ThargoidWakeData => "Thargoid Wake Data",
                Material::UnexpectedEmissionData => "Unexpected Emission Data",
                Material::UnidentifiedScanArchives => "Unidentified Scan Archives",
                Material::UntypicalShieldScans => "Untypical Shield Scans",
                Material::UnusualEncryptedFiles => "Unusual Encrypted Files",

                #[cfg(not(feature = "strict"))]
                Material::Unknown(unknown) => return write!(f, "Unknown material: {}", unknown),
            }
        )
    }
}
