use std::fmt::{Display, Formatter};

use crate::models::journal_event_content::shared::materials::material::Material;

#[derive(Debug)]
pub enum MaterialGroup {
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

impl MaterialGroup {
    pub fn materials(&self) -> Vec<Material> {
        match self {
            // Raw
            MaterialGroup::RawMaterials1 => vec![
                Material::Carbon,
                Material::Vanadium,
                Material::Niobium,
                Material::Yttrium,
            ],
            MaterialGroup::RawMaterials2 => vec![
                Material::Phosphorus,
                Material::Chromium,
                Material::Molybdenum,
                Material::Technetium,
            ],
            MaterialGroup::RawMaterials3 => vec![
                Material::Sulphur,
                Material::Manganese,
                Material::Cadmium,
                Material::Ruthenium,
            ],
            MaterialGroup::RawMaterials4 => vec![
                Material::Iron,
                Material::Zinc,
                Material::Tin,
                Material::Selenium,
            ],
            MaterialGroup::RawMaterials5 => vec![
                Material::Nickel,
                Material::Germanium,
                Material::Tungsten,
                Material::Tellurium,
            ],
            MaterialGroup::RawMaterials6 => vec![
                Material::Rhenium,
                Material::Arsenic,
                Material::Mercury,
                Material::Polonium,
            ],
            MaterialGroup::RawMaterials7 => vec![
                Material::Lead,
                Material::Zirconium,
                Material::Boron,
                Material::Antimony,
            ],

            // Manufactured
            MaterialGroup::Chemical => vec![
                Material::ChemicalStorageUnits,
                Material::ChemicalProcessors,
                Material::ChemicalDistillery,
                Material::ChemicalManipulators,
                Material::PharmaceuticalIsolators,
            ],
            MaterialGroup::Thermic => vec![
                Material::TemperedAlloys,
                Material::HeatResistantCeramics,
                Material::PrecipitatedAlloys,
                Material::ThermicAlloys,
                Material::MilitaryGradeAlloys,
            ],
            MaterialGroup::Heat => vec![
                Material::HeatConductionWiring,
                Material::HeatDispersionPlate,
                Material::HeatExchangers,
                Material::HeatVanes,
                Material::ProtoHeatRadiators,
            ],
            MaterialGroup::Conductive => vec![
                Material::BasicConductors,
                Material::ConductiveComponents,
                Material::ConductiveCeramics,
                Material::ConductivePolymers,
                Material::BiotechConductors,
            ],
            MaterialGroup::MechanicalComponents => vec![
                Material::MechanicalScrap,
                Material::MechanicalEquipment,
                Material::MechanicalComponents,
                Material::ConfigurableComponents,
                Material::ImprovisedComponents,
            ],
            MaterialGroup::Capacitors => vec![
                Material::GridResistors,
                Material::HybridCapacitors,
                Material::ElectrochemicalArrays,
                Material::PolymerCapacitors,
                Material::MilitarySupercapacitors,
            ],
            MaterialGroup::Shielding => vec![
                Material::WornShieldEmitters,
                Material::ShieldEmitters,
                Material::ShieldingSensors,
                Material::CompoundShielding,
                Material::ImperialShielding,
            ],
            MaterialGroup::Composite => vec![
                Material::CompactComposites,
                Material::FilamentComposites,
                Material::HighDensityComposites,
                Material::ProprietaryComposites,
                Material::CoreDynamicsComposites,
            ],
            MaterialGroup::Crystal => vec![
                Material::CrystalShards,
                Material::FlawedFocusCrystals,
                Material::FocusCrystals,
                Material::RefinedFocusCrystals,
                Material::ExquisiteFocusCrystals,
            ],
            MaterialGroup::Alloys => vec![
                Material::SalvagedAlloys,
                Material::GalvanisingAlloys,
                Material::PhaseAlloys,
                Material::ProtoLightAlloys,
                Material::ProtoRadiolicAlloys,
            ],

            // Encoded
            MaterialGroup::EmissionData => vec![
                Material::ExceptionScrambledEmissionData,
                Material::IrregularEmissionData,
                Material::UnexpectedEmissionData,
                Material::DecodedEmissionData,
                Material::AbnormalCompactEmissionData,
            ],
            MaterialGroup::WakeScans => vec![
                Material::AtypicalDisruptedWakeEchoes,
                Material::AnomalousFSDTelemetry,
                Material::StrangeWakeSolutions,
                Material::EccentricHyperspaceTrajectories,
                Material::DataminedWakeExceptions,
            ],
            MaterialGroup::ShieldData => vec![
                Material::DistortedShieldCycleRecordings,
                Material::InconsistentShieldSoakAnalysis,
                Material::UntypicalShieldScans,
                Material::AberrantShieldPatternAnalysis,
                Material::PeculiarShieldFrequencyData,
            ],
            MaterialGroup::EncryptionFiles => vec![
                Material::UnusualEncryptedFiles,
                Material::TaggedEncryptionCodes,
                Material::OpenSymmetricKeys,
                Material::AtypicalEncryptionArchives,
                Material::AdaptiveEncryptorsCapture,
            ],
            MaterialGroup::DataArchives => vec![
                Material::AnomalousBulkScanData,
                Material::UnidentifiedScanArchives,
                Material::ClassifiedScanDatabanks,
                Material::DivergentScanData,
                Material::ClassifiedScanFragment,
            ],
            MaterialGroup::EncodedFirmware => vec![
                Material::SpecializedLegacyFirmware,
                Material::ModifiedConsumerFirmware,
                Material::CrackedIndustrialFirmware,
                Material::SecurityFirmwarePatch,
                Material::ModifiedEmbeddedFirmware,
            ],
        }
    }
}

impl Display for MaterialGroup {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            // Raw
            MaterialGroup::RawMaterials1 => "Raw Materials 1",
            MaterialGroup::RawMaterials2 => "Raw Materials 2",
            MaterialGroup::RawMaterials3 => "Raw Materials 3",
            MaterialGroup::RawMaterials4 => "Raw Materials 4",
            MaterialGroup::RawMaterials5 => "Raw Materials 5",
            MaterialGroup::RawMaterials6 => "Raw Materials 6",
            MaterialGroup::RawMaterials7 => "Raw Materials 7",

            // Manufactured
            MaterialGroup::Chemical => "Chemical",
            MaterialGroup::Thermic => "Thermic",
            MaterialGroup::Heat => "Heat",
            MaterialGroup::Conductive => "Conductive",
            MaterialGroup::MechanicalComponents => "Mechanical Components",
            MaterialGroup::Capacitors => "Capacitors",
            MaterialGroup::Shielding => "Shielding",
            MaterialGroup::Composite => "Composite",
            MaterialGroup::Crystal => "Crystal",
            MaterialGroup::Alloys => "Alloys",

            // Encoded
            MaterialGroup::EmissionData => "Emission Data",
            MaterialGroup::WakeScans => "Wake Scans",
            MaterialGroup::ShieldData => "Shield Data",
            MaterialGroup::EncryptionFiles => "Encryption Files",
            MaterialGroup::DataArchives => "Data Archives",
            MaterialGroup::EncodedFirmware => "Encoded Firmware",
        })
    }
}
