use std::str::FromStr;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::modules::odyssey::Item;

/// Type of category for a given Odyssey item.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ItemCategory {
    #[serde(alias = "$MICRORESOURCE_CATEGORY_Data;")]
    Data,

    #[serde(alias = "Item", alias = "$MICRORESOURCE_CATEGORY_Item;")]
    Goods,

    #[serde(alias = "Component")]
    Chemicals,
    Circuits,
    Tech,
    Consumable,

    #[serde(alias = "Component", alias = "$MICRORESOURCE_CATEGORY_Component;")]
    Component,

    Mission,

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    #[serde(untagged)]
    Unknown(String),
}

#[derive(Debug, Error)]
pub enum ItemCategoryError {
    #[error("Unknown item category: '{0}'")]
    UnknownItemCategory(String),
}

impl FromStr for ItemCategory {
    type Err = ItemCategoryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
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
            | Item::XenoDefenceProtocols => ItemCategory::Data,

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
            | Item::WeaponSchematic => ItemCategory::Goods,

            Item::Aerogel
            | Item::ChemicalCatalyst
            | Item::ChemicalSuperbase
            | Item::Epinephrine
            | Item::EpoxyAdhesive
            | Item::Graphene
            | Item::OxygenicBacteria
            | Item::PHNeutraliser
            | Item::RDX
            | Item::ViscoelasticPolymer => ItemCategory::Chemicals,

            Item::CircuitBoard
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
            | Item::OpticalFibre => ItemCategory::Circuits,

            Item::CarbonFibrePlating
            | Item::EncryptedMemoryChip
            | Item::MemoryChip
            | Item::MicroHydraulics
            | Item::MicroThrusters
            | Item::OpticalLens
            | Item::Scrambler
            | Item::TitaniumPlating
            | Item::Transmitter
            | Item::TungstenCarbide
            | Item::WeaponComponent => ItemCategory::Tech,

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