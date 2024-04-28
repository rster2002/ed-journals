use crate::models::journal_event_kind::shared::materials::material::Material;

#[derive(Debug)]
pub enum MaterialGrade {
    Grade1,
    Grade2,
    Grade3,
    Grade4,
    Grade5,

    /// Used for odyssey materials
    None,

    #[cfg(not(feature = "strict"))]
    Unknown,
}

impl From<Material> for MaterialGrade {
    fn from(value: Material) -> Self {
        match value {
            // Raw
            Material::Carbon => MaterialGrade::Grade1,
            Material::Vanadium => MaterialGrade::Grade2,
            Material::Niobium => MaterialGrade::Grade3,
            Material::Yttrium => MaterialGrade::Grade4,

            Material::Phosphorus => MaterialGrade::Grade1,
            Material::Chromium => MaterialGrade::Grade2,
            Material::Molybdenum => MaterialGrade::Grade3,
            Material::Technetium => MaterialGrade::Grade4,

            Material::Sulphur => MaterialGrade::Grade1,
            Material::Manganese => MaterialGrade::Grade2,
            Material::Cadmium => MaterialGrade::Grade3,
            Material::Ruthenium => MaterialGrade::Grade4,

            Material::Iron => MaterialGrade::Grade1,
            Material::Zinc => MaterialGrade::Grade2,
            Material::Tin => MaterialGrade::Grade3,
            Material::Selenium => MaterialGrade::Grade4,

            Material::Nickel => MaterialGrade::Grade1,
            Material::Germanium => MaterialGrade::Grade2,
            Material::Tungsten => MaterialGrade::Grade3,
            Material::Tellurium => MaterialGrade::Grade4,

            Material::Rhenium => MaterialGrade::Grade1,
            Material::Arsenic => MaterialGrade::Grade2,
            Material::Mercury => MaterialGrade::Grade3,
            Material::Polonium => MaterialGrade::Grade4,

            Material::Lead => MaterialGrade::Grade1,
            Material::Zirconium => MaterialGrade::Grade2,
            Material::Boron => MaterialGrade::Grade3,
            Material::Antimony => MaterialGrade::Grade4,

            // Manufactured
            Material::ChemicalStorageUnits => MaterialGrade::Grade1,
            Material::ChemicalProcessors => MaterialGrade::Grade2,
            Material::ChemicalDistillery => MaterialGrade::Grade3,
            Material::ChemicalManipulators => MaterialGrade::Grade4,
            Material::PharmaceuticalIsolators => MaterialGrade::Grade5,

            Material::TemperedAlloys => MaterialGrade::Grade1,
            Material::HeatResistantCeramics => MaterialGrade::Grade2,
            Material::PrecipitatedAlloys => MaterialGrade::Grade3,
            Material::ThermicAlloys => MaterialGrade::Grade4,
            Material::MilitaryGradeAlloys => MaterialGrade::Grade5,

            Material::HeatConductionWiring => MaterialGrade::Grade1,
            Material::HeatDispersionPlate => MaterialGrade::Grade2,
            Material::HeatExchangers => MaterialGrade::Grade3,
            Material::HeatVanes => MaterialGrade::Grade4,
            Material::ProtoHeatRadiators => MaterialGrade::Grade5,

            Material::BasicConductors => MaterialGrade::Grade1,
            Material::ConductiveComponents => MaterialGrade::Grade2,
            Material::ConductiveCeramics => MaterialGrade::Grade3,
            Material::ConductivePolymers => MaterialGrade::Grade4,
            Material::BiotechConductors => MaterialGrade::Grade5,

            Material::MechanicalScrap => MaterialGrade::Grade1,
            Material::MechanicalEquipment => MaterialGrade::Grade2,
            Material::MechanicalComponents => MaterialGrade::Grade3,
            Material::ConfigurableComponents => MaterialGrade::Grade4,
            Material::ImprovisedComponents => MaterialGrade::Grade5,

            Material::GridResistors => MaterialGrade::Grade1,
            Material::HybridCapacitors => MaterialGrade::Grade2,
            Material::ElectrochemicalArrays => MaterialGrade::Grade3,
            Material::PolymerCapacitors => MaterialGrade::Grade4,
            Material::MilitarySupercapacitors => MaterialGrade::Grade5,

            Material::WornShieldEmitters => MaterialGrade::Grade1,
            Material::ShieldEmitters => MaterialGrade::Grade2,
            Material::ShieldingSensors => MaterialGrade::Grade3,
            Material::CompoundShielding => MaterialGrade::Grade4,
            Material::ImperialShielding => MaterialGrade::Grade5,

            Material::CompactComposites => MaterialGrade::Grade1,
            Material::FilamentComposites => MaterialGrade::Grade2,
            Material::HighDensityComposites => MaterialGrade::Grade3,
            Material::ProprietaryComposites => MaterialGrade::Grade4,
            Material::CoreDynamicsComposites => MaterialGrade::Grade5,

            Material::CrystalShards => MaterialGrade::Grade1,
            Material::FlawedFocusCrystals => MaterialGrade::Grade2,
            Material::FocusCrystals => MaterialGrade::Grade3,
            Material::RefinedFocusCrystals => MaterialGrade::Grade4,
            Material::ExquisiteFocusCrystals => MaterialGrade::Grade5,

            Material::SalvagedAlloys => MaterialGrade::Grade1,
            Material::GalvanisingAlloys => MaterialGrade::Grade2,
            Material::PhaseAlloys => MaterialGrade::Grade3,
            Material::ProtoLightAlloys => MaterialGrade::Grade4,
            Material::ProtoRadiolicAlloys => MaterialGrade::Grade5,

            Material::HardenedSurfaceFragments => MaterialGrade::Grade1,
            Material::CausticShard => MaterialGrade::Grade2,
            Material::TacticalCoreChip => MaterialGrade::Grade2,
            Material::ThargoidCarapace => MaterialGrade::Grade2,
            Material::BioMechanicalConduits => MaterialGrade::Grade3,
            Material::CorrosiveMechanisms => MaterialGrade::Grade3,
            Material::PhasingMembraneResidue => MaterialGrade::Grade3,
            Material::ThargoidEnergyCell => MaterialGrade::Grade3,
            Material::WreckageComponents => MaterialGrade::Grade3,
            Material::CausticCrystal => MaterialGrade::Grade4,
            Material::ThargoidTechnologicalComponents => MaterialGrade::Grade4,
            Material::WeaponParts => MaterialGrade::Grade4,
            Material::HeatExposureSpecimen => MaterialGrade::Grade5,
            Material::PropulsionElements => MaterialGrade::Grade5,
            Material::SensorFragment => MaterialGrade::Grade5,
            Material::ThargoidOrganicCircuitry => MaterialGrade::Grade5,

            Material::GuardianPowerCell => MaterialGrade::Grade1,
            Material::GuardianWreckageComponents => MaterialGrade::Grade1,
            Material::GuardianPowerConduit => MaterialGrade::Grade2,
            Material::GuardianSentinelWeaponParts => MaterialGrade::Grade3,
            Material::GuardianTechnologyComponent => MaterialGrade::Grade3,

            // Encoded
            Material::ExceptionScrambledEmissionData => MaterialGrade::Grade1,
            Material::IrregularEmissionData => MaterialGrade::Grade2,
            Material::UnexpectedEmissionData => MaterialGrade::Grade3,
            Material::DecodedEmissionData => MaterialGrade::Grade4,
            Material::AbnormalCompactEmissionData => MaterialGrade::Grade5,

            Material::AtypicalDisruptedWakeEchoes => MaterialGrade::Grade1,
            Material::AnomalousFSDTelemetry => MaterialGrade::Grade2,
            Material::StrangeWakeSolutions => MaterialGrade::Grade3,
            Material::EccentricHyperspaceTrajectories => MaterialGrade::Grade4,
            Material::DataminedWakeExceptions => MaterialGrade::Grade5,

            Material::DistortedShieldCycleRecordings => MaterialGrade::Grade1,
            Material::InconsistentShieldSoakAnalysis => MaterialGrade::Grade2,
            Material::UntypicalShieldScans => MaterialGrade::Grade3,
            Material::AberrantShieldPatternAnalysis => MaterialGrade::Grade4,
            Material::PeculiarShieldFrequencyData => MaterialGrade::Grade5,

            Material::UnusualEncryptedFiles => MaterialGrade::Grade1,
            Material::TaggedEncryptionCodes => MaterialGrade::Grade2,
            Material::OpenSymmetricKeys => MaterialGrade::Grade3,
            Material::AtypicalEncryptionArchives => MaterialGrade::Grade4,
            Material::AdaptiveEncryptorsCapture => MaterialGrade::Grade5,

            Material::AnomalousBulkScanData => MaterialGrade::Grade1,
            Material::UnidentifiedScanArchives => MaterialGrade::Grade2,
            Material::ClassifiedScanDatabanks => MaterialGrade::Grade3,
            Material::DivergentScanData => MaterialGrade::Grade4,
            Material::ClassifiedScanFragment => MaterialGrade::Grade5,

            Material::SpecializedLegacyFirmware => MaterialGrade::Grade1,
            Material::ModifiedConsumerFirmware => MaterialGrade::Grade2,
            Material::CrackedIndustrialFirmware => MaterialGrade::Grade3,
            Material::SecurityFirmwarePatch => MaterialGrade::Grade4,
            Material::ModifiedEmbeddedFirmware => MaterialGrade::Grade5,

            Material::ThargoidStructuralData => MaterialGrade::Grade2,
            Material::MassiveEnergySurgeAnalytics => MaterialGrade::Grade3,
            Material::ShipFlightData => MaterialGrade::Grade3,
            Material::ShipSystemsData => MaterialGrade::Grade3,
            Material::ThargoidInterdictionTelemetry => MaterialGrade::Grade3,
            Material::ThargoidMaterialCompositionData => MaterialGrade::Grade3,
            Material::ThargoidShipSignature => MaterialGrade::Grade3,
            Material::ThargoidResidueData => MaterialGrade::Grade4,
            Material::ThargoidWakeData => MaterialGrade::Grade4,

            Material::PatternGammaObeliskData => MaterialGrade::Grade1,
            Material::PatternBetaObeliskData => MaterialGrade::Grade2,
            Material::PatternAlphaObeliskData => MaterialGrade::Grade3,
            Material::PatternDeltaObeliskData => MaterialGrade::Grade4,
            Material::PatternEpsilonObeliskData => MaterialGrade::Grade4,
            Material::GuardianModuleBlueprintFragment => MaterialGrade::Grade5,
            Material::GuardianVesselBlueprintFragment => MaterialGrade::Grade5,
            Material::GuardianWeaponBlueprintFragment => MaterialGrade::Grade5,

            // Material::ExtractionYieldData
            // | Material::StellarActivityLogs => MaterialGrade::None,
            #[cfg(not(feature = "strict"))]
            Material::Unknown(_) => MaterialGrade::Unknown,
        }
    }
}
