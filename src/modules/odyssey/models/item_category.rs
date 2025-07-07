use crate::modules::odyssey::Item;
use serde::{Deserialize, Serialize};

/// Type of category for a given Odyssey item.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ItemCategory {
    #[serde(alias = "Data", alias = "$MICRORESOURCE_CATEGORY_Data;")]
    Data,

    #[serde(alias = "Component", alias = "$MICRORESOURCE_CATEGORY_Component;")]
    Component,

    #[serde(alias = "Item", alias = "$MICRORESOURCE_CATEGORY_Item;")]
    Item,

    #[serde(alias = "Consumable", alias = "$MICRORESOURCE_CATEGORY_Consumable;")]
    Consumable,

    #[serde(alias = "Mission", alias = "$MICRORESOURCE_CATEGORY_Mission;")]
    Mission,

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    #[serde(untagged)]
    Unknown(String),
}

impl From<Item> for ItemCategory {
    fn from(value: Item) -> Self {
        match value {
            Item::AccidentLogs
            | Item::AirQualityReports
            | Item::AtmosphericData
            | Item::AudioLogs
            | Item::AxCombatLogs
            | Item::BallisticsData
            | Item::BiologicalWeaponData
            | Item::BiometricData
            | Item::BlacklistData
            | Item::BloodTestResults
            | Item::CampaignPlans
            | Item::CatMedia
            | Item::CensusData
            | Item::ChemicalExperimentData
            | Item::ChemicalFormulae
            | Item::ChemicalInventory
            | Item::ChemicalPatents
            | Item::ChemicalWeaponData
            | Item::ClassicEntertainment
            | Item::ClinicalTrialRecords
            | Item::CocktailRecipes
            | Item::CombatTrainingMaterial
            | Item::CombatantPerformance
            | Item::ConflictHistory
            | Item::CriminalRecords
            | Item::CropYieldAnalysis
            | Item::CulinaryRecipes
            | Item::DigitalDesigns
            | Item::DutyRota
            | Item::EmployeeDirectory
            | Item::EmployeeExpenses
            | Item::EmployeeGeneticData
            | Item::EmploymentHistory
            | Item::EnhancedInterrogationRecordings
            | Item::EspionageMaterial
            | Item::EvacuationProtocols
            | Item::ExplorationJournals
            | Item::ExtractionYieldData
            | Item::FactionAssociates
            | Item::FactionDonatorList
            | Item::FactionNews
            | Item::FinancialProjections
            | Item::FleetRegistry
            | Item::GeneSequencingData
            | Item::GeneticResearch
            | Item::GeologicalData
            | Item::HydroponicData
            | Item::IncidentLogs
            | Item::InfluenceProjections
            | Item::InternalCorrespondence
            | Item::InterrogationRecordings
            | Item::InterviewRecordings
            | Item::JobApplications
            | Item::Kompromat
            | Item::LiteraryFiction
            | Item::MaintenanceLogs
            | Item::ManufacturingInstructions
            | Item::MedicalRecords
            | Item::MeetingMinutes
            | Item::MineralSurvey
            | Item::MiningAnalytics
            | Item::MultimediaEntertainment
            | Item::NetworkAccessHistory
            | Item::NetworkSecurityProtocols
            | Item::NextOfKinRecords
            | Item::NocData
            | Item::OperationalManual
            | Item::OpinionPolls
            | Item::PatientHistory
            | Item::PatrolRoutes
            | Item::PayrollInformation
            | Item::PersonalLogs
            | Item::PharmaceuticalPatents
            | Item::PhotoAlbums
            | Item::PlantGrowthCharts
            | Item::PoliticalAffiliations
            | Item::PrisonerLogs
            | Item::ProductionReports
            | Item::ProductionSchedule
            | Item::Propaganda
            | Item::PurchaseRecords
            | Item::PurchaseRequests
            | Item::RadioactivityData
            | Item::ReactorOutputReview
            | Item::RecyclingLogs
            | Item::ResidentialDirectory
            | Item::RiskAssessments
            | Item::SalesRecords
            | Item::SecurityExpenses
            | Item::SeedGeneaology
            | Item::SettlementAssaultPlans
            | Item::SettlementDefencePlans
            | Item::ShareholderInformation
            | Item::SlushFundLogs
            | Item::SmearCampaignPlans
            | Item::SpectralAnalysisData
            | Item::Spyware
            | Item::StellarActivityLogs
            | Item::SurveillanceLogs
            | Item::TacticalPlans
            | Item::TaxRecords
            | Item::TopographicalSurveys
            | Item::TravelPermits
            | Item::TroopDeploymentRecords
            | Item::UnionMembership
            | Item::VaccinationRecords
            | Item::VaccineResearch
            | Item::VipSecurityDetail
            | Item::VirologyData
            | Item::Virus
            | Item::VisitorRegister
            | Item::WeaponInventory
            | Item::WeaponTestData
            | Item::XenoDefenceProtocols
            | Item::PowerplayAssociationData
            | Item::PowerplayIndustrialData
            | Item::PowerClassifiedData
            | Item::PowerplayResearchData
            | Item::PowerplayTrackerMalware
            | Item::PowerplayInjectionMalware => ItemCategory::Data,

            Item::AgriculturalProcessSample
            | Item::BiochemicalAgent
            | Item::BiologicalSample
            | Item::BuildingSchematic
            | Item::Californium
            | Item::CastFossil
            | Item::ChemicalProcessSample
            | Item::ChemicalSample
            | Item::CompactLibrary
            | Item::CompressionLiquefiedGas
            | Item::ContaminatedSpireRefineryCompound
            | Item::DeepMantleSample
            | Item::DegradedPowerRegulator
            | Item::GMeds
            | Item::GeneticRepairMeds
            | Item::HealthMonitor
            | Item::Hush
            | Item::InertiaCanister
            | Item::Infinity
            | Item::InorganicContaminant
            | Item::Insight
            | Item::InsightDataBank
            | Item::InsightEntertainmentSuite
            | Item::IonisedGas
            | Item::Lazarus
            | Item::MicrobialInhibitor
            | Item::MutagenicCatalyst
            | Item::NutritionalConcentrate
            | Item::PersonalComputer
            | Item::PersonalDocuments
            | Item::PetrifiedFossil
            | Item::PowerRegulator
            | Item::Push
            | Item::PyrolyticCatalyst
            | Item::RefinementProcessSample
            | Item::ShipSchematic
            | Item::SpireRefineryCompound
            | Item::SuitSchematic
            | Item::SurveillanceEquipment
            | Item::SyntheticGenome
            | Item::SyntheticPathogen
            | Item::TrueFormFossil
            | Item::UniversalTranslator
            | Item::VehicleSchematic
            | Item::WeaponSchematic
            | Item::PowerplayEnergyRegulator
            | Item::PowerplayExtractionSample
            | Item::PowerplayElectronics
            | Item::PowerplayPersonalProtectiveEquipment => ItemCategory::Item,

            Item::CircuitBoard
            | Item::CarbonFibrePlating
            | Item::CircuitSwitch
            | Item::ElectricalFuse
            | Item::ElectricalWiring
            | Item::Electromagnet
            | Item::IonBattery
            | Item::MetalCoil
            | Item::MicroSupercapacitor
            | Item::MicroTransformer
            | Item::Microelectrode
            | Item::Motor
            | Item::OpticalFibre
            | Item::Aerogel
            | Item::ChemicalCatalyst
            | Item::ChemicalSuperbase
            | Item::Epinephrine
            | Item::EpoxyAdhesive
            | Item::Graphene
            | Item::OxygenicBacteria
            | Item::PHNeutraliser
            | Item::RDX
            | Item::ViscoelasticPolymer
            | Item::EncryptedMemoryChip
            | Item::MemoryChip
            | Item::MicroHydraulics
            | Item::MicroThrusters
            | Item::OpticalLens
            | Item::Scrambler
            | Item::TitaniumPlating
            | Item::Transmitter
            | Item::TungstenCarbide
            | Item::WeaponComponent => ItemCategory::Component,

            Item::EnergyCell
            | Item::FragGranade
            | Item::Medkit
            | Item::ShieldDisruptor
            | Item::ShieldProjector
            | Item::EBreach => ItemCategory::Consumable,

            #[cfg(feature = "allow-unknown")]
            Item::Unknown(item) => ItemCategory::Unknown(format!("Unknown item: {}", item)),
        }
    }
}
