use serde::Deserialize;
use crate::models::journal_event_kind::shared::materials::material::Material;

/// The type of material, either Raw, Manufactured, or Encoded.
#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub enum MaterialCategory {
    #[serde(alias = "raw", alias = "$MICRORESOURCE_CATEGORY_Raw;")]
    Raw,

    #[serde(alias = "manufactured", alias = "$MICRORESOURCE_CATEGORY_Manufactured;")]
    Manufactured,

    #[serde(alias = "encoded", alias = "$MICRORESOURCE_CATEGORY_Encoded;")]
    Encoded,
    //
    // #[serde(alias = "$MICRORESOURCE_CATEGORY_Data;")]
    // Data,
    //
    // Item,

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}

// /// Sometimes a lowercase version of [MaterialCategory] is used, so this type is used in the data to
// /// preserve the casing. Prefer using [MaterialCategory] in your application by using the [From] trait.
// #[derive(Debug, Deserialize)]
// #[cfg_attr(test, derive(PartialEq))]
// pub enum MaterialTypeLowercase {
//     #[serde(rename = "raw")]
//     Raw,
//
//     #[serde(rename = "manufactured")]
//     Manufactured,
//
//     #[serde(rename = "encoded")]
//     Encoded,
//
//     #[cfg(not(feature = "strict"))]
//     #[serde(untagged)]
//     Unknown(String),
// }

impl From<Material> for MaterialCategory {
    fn from(value: Material) -> Self {
        match value {
            Material::Carbon
            | Material::Vanadium
            | Material::Niobium
            | Material::Yttrium
            | Material::Phosphorus
            | Material::Chromium
            | Material::Molybdenum
            | Material::Technetium
            | Material::Sulphur
            | Material::Manganese
            | Material::Cadmium
            | Material::Ruthenium
            | Material::Iron
            | Material::Zinc
            | Material::Tin
            | Material::Selenium
            | Material::Nickel
            | Material::Germanium
            | Material::Tungsten
            | Material::Tellurium
            | Material::Rhenium
            | Material::Arsenic
            | Material::Mercury
            | Material::Polonium
            | Material::Lead
            | Material::Zirconium
            | Material::Boron
            | Material::Antimony => MaterialCategory::Raw,

            Material::ChemicalStorageUnits
            | Material::ChemicalProcessors
            | Material::ChemicalDistillery
            | Material::ChemicalManipulators
            | Material::PharmaceuticalIsolators
            | Material::TemperedAlloys
            | Material::HeatResistantCeramics
            | Material::PrecipitatedAlloys
            | Material::ThermicAlloys
            | Material::MilitaryGradeAlloys
            | Material::HeatConductionWiring
            | Material::HeatDispersionPlate
            | Material::HeatExchangers
            | Material::HeatVanes
            | Material::ProtoHeatRadiators
            | Material::BasicConductors
            | Material::ConductiveComponents
            | Material::ConductiveCeramics
            | Material::ConductivePolymers
            | Material::BiotechConductors
            | Material::MechanicalScrap
            | Material::MechanicalEquipment
            | Material::MechanicalComponents
            | Material::ConfigurableComponents
            | Material::ImprovisedComponents
            | Material::GridResistors
            | Material::HybridCapacitors
            | Material::ElectrochemicalArrays
            | Material::PolymerCapacitors
            | Material::MilitarySupercapacitors
            | Material::WornShieldEmitters
            | Material::ShieldEmitters
            | Material::ShieldingSensors
            | Material::CompoundShielding
            | Material::ImperialShielding
            | Material::CompactComposites
            | Material::FilamentComposites
            | Material::HighDensityComposites
            | Material::ProprietaryComposites
            | Material::CoreDynamicsComposites
            | Material::CrystalShards
            | Material::FlawedFocusCrystals
            | Material::FocusCrystals
            | Material::RefinedFocusCrystals
            | Material::ExquisiteFocusCrystals
            | Material::SalvagedAlloys
            | Material::GalvanisingAlloys
            | Material::PhaseAlloys
            | Material::ProtoLightAlloys
            | Material::ProtoRadiolicAlloys
            | Material::HardenedSurfaceFragments
            | Material::CausticShard
            | Material::TacticalCoreChip
            | Material::ThargoidCarapace
            | Material::BioMechanicalConduits
            | Material::CorrosiveMechanisms
            | Material::PhasingMembraneResidue
            | Material::ThargoidEnergyCell
            | Material::WreckageComponents
            | Material::CausticCrystal
            | Material::ThargoidTechnologicalComponents
            | Material::WeaponParts
            | Material::HeatExposureSpecimen
            | Material::PropulsionElements
            | Material::SensorFragment
            | Material::ThargoidOrganicCircuitry
            | Material::GuardianPowerCell
            | Material::GuardianWreckageComponents
            | Material::GuardianPowerConduit
            | Material::GuardianSentinelWeaponParts
            | Material::GuardianTechnologyComponent => MaterialCategory::Manufactured,

            Material::ExceptionScrambledEmissionData
            | Material::IrregularEmissionData
            | Material::UnexpectedEmissionData
            | Material::DecodedEmissionData
            | Material::AbnormalCompactEmissionData
            | Material::AtypicalDisruptedWakeEchoes
            | Material::AnomalousFSDTelemetry
            | Material::StrangeWakeSolutions
            | Material::EccentricHyperspaceTrajectories
            | Material::DataminedWakeExceptions
            | Material::DistortedShieldCycleRecordings
            | Material::InconsistentShieldSoakAnalysis
            | Material::UntypicalShieldScans
            | Material::AberrantShieldPatternAnalysis
            | Material::PeculiarShieldFrequencyData
            | Material::UnusualEncryptedFiles
            | Material::TaggedEncryptionCodes
            | Material::OpenSymmetricKeys
            | Material::AtypicalEncryptionArchives
            | Material::AdaptiveEncryptorsCapture
            | Material::AnomalousBulkScanData
            | Material::UnidentifiedScanArchives
            | Material::ClassifiedScanDatabanks
            | Material::DivergentScanData
            | Material::ClassifiedScanFragment
            | Material::SpecializedLegacyFirmware
            | Material::ModifiedConsumerFirmware
            | Material::CrackedIndustrialFirmware
            | Material::SecurityFirmwarePatch
            | Material::ModifiedEmbeddedFirmware
            | Material::ThargoidStructuralData
            | Material::MassiveEnergySurgeAnalytics
            | Material::ShipFlightData
            | Material::ShipSystemsData
            | Material::ThargoidInterdictionTelemetry
            | Material::ThargoidMaterialCompositionData
            | Material::ThargoidShipSignature
            | Material::ThargoidResidueData
            | Material::ThargoidWakeData
            | Material::PatternGammaObeliskData
            | Material::PatternBetaObeliskData
            | Material::PatternAlphaObeliskData
            | Material::PatternDeltaObeliskData
            | Material::PatternEpsilonObeliskData
            | Material::GuardianModuleBlueprintFragment
            | Material::GuardianVesselBlueprintFragment
            | Material::GuardianWeaponBlueprintFragment => MaterialCategory::Encoded,
            //
            // Material::ExtractionYieldData
            // | Material::StellarActivityLogs => MaterialCategory::Data,
            //
            // Material::GMeds => MaterialCategory::Item,

            #[cfg(not(feature = "strict"))]
            Material::Unknown(value) => MaterialCategory::Unknown(format!("Unknown material: '{}'", value)),
        }
    }
}

// impl From<MaterialTypeLowercase> for MaterialCategory {
//     fn from(value: MaterialTypeLowercase) -> Self {
//         match value {
//             MaterialTypeLowercase::Raw => MaterialCategory::Raw,
//             MaterialTypeLowercase::Manufactured => MaterialCategory::Manufactured,
//             MaterialTypeLowercase::Encoded => MaterialCategory::Encoded,
//
//             #[cfg(not(feature = "strict"))]
//             MaterialTypeLowercase::Unknown(value) => MaterialCategory::Unknown(value),
//         }
//     }
// }
//
// impl From<Material> for MaterialTypeLowercase {
//     fn from(value: Material) -> Self {
//         MaterialCategory::from(value).into()
//     }
// }
//
// impl From<MaterialCategory> for MaterialTypeLowercase {
//     fn from(value: MaterialCategory) -> Self {
//         match value {
//             MaterialCategory::Raw => MaterialTypeLowercase::Raw,
//             MaterialCategory::Manufactured => MaterialTypeLowercase::Manufactured,
//             MaterialCategory::Encoded => MaterialTypeLowercase::Encoded,
//
//             #[cfg(not(feature = "strict"))]
//             MaterialCategory::Unknown(value) => MaterialTypeLowercase::Unknown(value),
//         }
//     }
// }
