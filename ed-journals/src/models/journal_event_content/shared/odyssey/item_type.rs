use crate::journal_event_content::shared::odyssey::item::Item;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum ItemType {
    #[serde(alias = "$MICRORESOURCE_CATEGORY_Data;")]
    Data,

    #[serde(alias = "Item", alias = "$MICRORESOURCE_CATEGORY_Item;")]
    Goods,

    #[serde(alias = "Component")]
    Chemicals,
    Circuits,
    Tech,
    Consumable,

    Mission,

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}

impl From<Item> for ItemType {
    fn from(value: Item) -> Self {
        match value {
            Item::AccidentLogs
            | Item::AirQualityReports
            | Item::AtmosphericData
            | Item::BlacklistData
            | Item::CensusData
            | Item::CombatantPerformance
            | Item::EvacuationProtocols
            | Item::ExtractionYieldData
            | Item::FactionNews
            | Item::GeneSequencingData
            | Item::GeologicalData
            | Item::MaintenanceLogs
            | Item::NetworkSecurityProtocols
            | Item::PersonalLogs
            | Item::PharmaceuticalPatents
            | Item::Propaganda
            | Item::PurchaseRecords
            | Item::RadioactivityData
            | Item::ReactorOutputReview
            | Item::SecurityExpenses
            | Item::ShareholderInformation
            | Item::StellarActivityLogs
            | Item::TopographicalSurveys
            | Item::VaccinationRecords
            | Item::VirologyData
            | Item::VisitorRegister
            | Item::EnhancedInterrogationRecordings => ItemType::Data,

            Item::BiologicalSample
            | Item::BuildingSchematic
            | Item::Californium
            | Item::CastFossil
            | Item::CompactLibrary
            | Item::CompressionLiquefiedGas
            | Item::DeepMantleSample
            | Item::DegradedPowerRegulator
            | Item::GMeds
            | Item::HealthMonitor
            | Item::Hush
            | Item::InertiaCanister
            | Item::Infinity
            | Item::Insight
            | Item::InsightDataBank
            | Item::InsightEntertainmentSuite
            | Item::IonisedGas
            | Item::MicrobialInhibitor
            | Item::NutritionalConcentrate
            | Item::PersonalComputer
            | Item::PersonalDocuments
            | Item::PetrifiedFossil
            | Item::PowerRegulator
            | Item::Push
            | Item::SuitSchematic
            | Item::SyntheticPathogen
            | Item::UniversalTranslator
            | Item::VehicleSchematic
            | Item::WeaponSchematic => ItemType::Goods,

            Item::Aerogel
            | Item::ChemicalCatalyst
            | Item::ChemicalSuperbase
            | Item::Epinephrine
            | Item::EpoxyAdhesive
            | Item::Graphene
            | Item::OxygenicBacteria
            | Item::PHNeutraliser
            | Item::RDX
            | Item::ViscoelasticPolymer => ItemType::Chemicals,

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
            | Item::OpticalFibre => ItemType::Circuits,

            Item::CarbonFibrePlating
            | Item::EncryptedMemoryChip
            | Item::MemoryChip
            | Item::MicroThrusters
            | Item::OpticalLens
            | Item::Scrambler
            | Item::Transmitter
            | Item::TungstenCarbide
            | Item::WeaponComponent => ItemType::Tech,

            Item::EnergyCell
            | Item::FragGranade
            | Item::Medkit
            | Item::ShieldDisruptor
            | Item::ShieldProjector => ItemType::Consumable,

            Item::LargeCapacityPowerRegulator
            | Item::MedicalRecords
            | Item::SurveillanceEquipment
            | Item::ChemicalSample
            | Item::MutagenicCatalyst => ItemType::Mission,

            #[cfg(not(feature = "strict"))]
            Item::Unknown(item) => ItemType::Unknown(format!("Unknown item: {}", item)),
        }
    }
}
