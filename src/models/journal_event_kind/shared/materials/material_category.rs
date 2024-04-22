use crate::models::journal_event_kind::shared::materials::material::Material;

#[derive(Debug)]
pub enum MaterialCategory {
    // Raw
    RawMaterials1,
    RawMaterials2,
    RawMaterials3,
    RawMaterials4,
    RawMaterials5,
    RawMaterials6,
    RawMaterials7,

    // Manufactured
    Chemical,
    Thermic,
    Heat,
    Conductive,
    MechanicalComponents,
    Capacitors,
    Shielding,
    Composite,
    Crystal,
    Alloys,

    // Encoded
    EmissionData,
    WakeScans,
    ShieldData,
    EncryptionFiles,
    DataArchives,
    EncodedFirmware,
}

impl MaterialCategory {
    pub fn materials(&self) -> Vec<Material> {
        match self {
            // Raw
            MaterialCategory::RawMaterials1 => vec![
                Material::Carbon,
                Material::Vanadium,
                Material::Niobium,
                Material::Yttrium,
            ],
            MaterialCategory::RawMaterials2 => vec![
                Material::Phosphorus,
                Material::Chromium,
                Material::Molybdenum,
                Material::Technetium,
            ],
            MaterialCategory::RawMaterials3 => vec![
                Material::Sulphur,
                Material::Manganese,
                Material::Cadmium,
                Material::Ruthenium,
            ],
            MaterialCategory::RawMaterials4 => vec![
                Material::Iron,
                Material::Zinc,
                Material::Tin,
                Material::Selenium,
            ],
            MaterialCategory::RawMaterials5 => vec![
                Material::Nickel,
                Material::Germanium,
                Material::Tungsten,
                Material::Tellurium,
            ],
            MaterialCategory::RawMaterials6 => vec![
                Material::Rhenium,
                Material::Arsenic,
                Material::Mercury,
                Material::Polonium,
            ],
            MaterialCategory::RawMaterials7 => vec![
                Material::Lead,
                Material::Zirconium,
                Material::Boron,
                Material::Antimony,
            ],

            // Manufactured
            MaterialCategory::Chemical => vec![
                Material::ChemicalStorageUnits,
                Material::ChemicalProcessors,
                Material::ChemicalDistillery,
                Material::ChemicalManipulators,
                Material::PharmaceuticalIsolators,
            ],
            MaterialCategory::Thermic => vec![
                Material::TemperedAlloys,
                Material::HeatResistantCeramics,
                Material::PrecipitatedAlloys,
                Material::ThermicAlloys,
                Material::MilitaryGradeAlloys,
            ],
            MaterialCategory::Heat => vec![
                Material::HeatConductionWiring,
                Material::HeatDispersionPlate,
                Material::HeatExchangers,
                Material::HeatVanes,
                Material::ProtoHeatRadiators,
            ],
            MaterialCategory::Conductive => vec![
                Material::BasicConductors,
                Material::ConductiveComponents,
                Material::ConductiveCeramics,
                Material::ConductivePolymers,
                Material::BiotechConductors,
            ],
            MaterialCategory::MechanicalComponents => vec![
                Material::MechanicalScrap,
                Material::MechanicalEquipment,
                Material::MechanicalComponents,
                Material::ConfigurableComponents,
                Material::ImprovisedComponents,
            ],
            MaterialCategory::Capacitors => vec![
                Material::GridResistors,
                Material::HybridCapacitors,
                Material::ElectrochemicalArrays,
                Material::PolymerCapacitors,
                Material::MilitarySupercapacitors,
            ],
            MaterialCategory::Shielding => vec![
                Material::WornShieldEmitters,
                Material::ShieldEmitters,
                Material::ShieldingSensors,
                Material::CompoundShielding,
                Material::ImperialShielding,
            ],
            MaterialCategory::Composite => vec![
                Material::CompactComposites,
                Material::FilamentComposites,
                Material::HighDensityComposites,
                Material::ProprietaryComposites,
                Material::CoreDynamicsComposites,
            ],
            MaterialCategory::Crystal => vec![
                Material::CrystalShards,
                Material::FlawedFocusCrystals,
                Material::FocusCrystals,
                Material::RefinedFocusCrystals,
                Material::ExquisiteFocusCrystals,
            ],
            MaterialCategory::Alloys => vec![
                Material::SalvagedAlloys,
                Material::GalvanisingAlloys,
                Material::PhaseAlloys,
                Material::ProtoLightAlloys,
                Material::ProtoRadiolicAlloys,
            ],

            // Encoded
            MaterialCategory::EmissionData => vec![
                Material::ExceptionScrambledEmissionData,
                Material::IrregularEmissionData,
                Material::UnexpectedEmissionData,
                Material::DecodedEmissionData,
                Material::AbnormalCompactEmissionData,
            ],
            MaterialCategory::WakeScans => vec![
                Material::AtypicalDisruptedWakeEchoes,
                Material::AnomalousFSDTelemetry,
                Material::StrangeWakeSolutions,
                Material::EccentricHyperspaceTrajectories,
                Material::DataminedWakeExceptions,
            ],
            MaterialCategory::ShieldData => vec![
                Material::DistortedShieldCycleRecordings,
                Material::InconsistentShieldSoakAnalysis,
                Material::UntypicalShieldScans,
                Material::AberrantShieldPatternAnalysis,
                Material::PeculiarShieldFrequencyData,
            ],
            MaterialCategory::EncryptionFiles => vec![
                Material::UnusualEncryptedFiles,
                Material::TaggedEncryptionCodes,
                Material::OpenSymmetricKeys,
                Material::AtypicalEncryptionArchives,
                Material::AdaptiveEncryptorsCapture,
            ],
            MaterialCategory::DataArchives => vec![
                Material::AnomalousBulkScanData,
                Material::UnidentifiedScanArchives,
                Material::ClassifiedScanDatabanks,
                Material::DivergentScanData,
                Material::ClassifiedScanFragment,
            ],
            MaterialCategory::EncodedFirmware => vec![
                Material::SpecializedLegacyFirmware,
                Material::ModifiedConsumerFirmware,
                Material::CrackedIndustrialFirmware,
                Material::SecurityFirmwarePatch,
                Material::ModifiedEmbeddedFirmware,
            ],
        }
    }
}
