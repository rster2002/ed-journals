use std::str::FromStr;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;
use thiserror::Error;
use crate::from_str_deserialize_impl;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Item {
    // Data
    AccidentLogs,
    AirQualityReports,
    AtmosphericData,
    BlacklistData,
    CensusData,
    CombatantPerformance,
    EvacuationProtocols,
    ExtractionYieldData,
    FactionNews,
    GeneSequencingData,
    GeologicalData,
    MaintenanceLogs,
    NetworkSecurityProtocols,
    PersonalLogs,
    PharmaceuticalPatents,
    Propaganda,
    PurchaseRecords,
    RadioactivityData,
    ReactorOutputReview,
    SecurityExpenses,
    ShareholderInformation,
    StellarActivityLogs,
    TopographicalSurveys,
    VaccinationRecords,
    VirologyData,
    VisitorRegister,
    EnhancedInterrogationRecordings,

    // Goods
    BiologicalSample,
    BuildingSchematic,
    Californium,
    CastFossil,
    CompactLibrary,
    CompressionLiquefiedGas,
    DeepMantleSample,
    DegradedPowerRegulator,
    GMeds,
    HealthMonitor,
    Hush,
    InertiaCanister,
    Infinity,
    Insight,
    InsightDataBank,
    InsightEntertainmentSuite,
    IonisedGas,
    MicrobialInhibitor,
    NutritionalConcentrate,
    PersonalComputer,
    PersonalDocuments,
    PetrifiedFossil,
    PowerRegulator,
    Push,
    SuitSchematic,
    SyntheticPathogen,
    UniversalTranslator,
    VehicleSchematic,
    WeaponSchematic,

    // Chemicals
    Aerogel,
    ChemicalCatalyst,
    ChemicalSuperbase,
    Epinephrine,
    EpoxyAdhesive,
    Graphene,
    OxygenicBacteria,
    PHNeutraliser,
    RDX,
    ViscoelasticPolymer,

    // Circuits
    CircuitBoard,
    CircuitSwitch,
    ElectricalFuse,
    ElectricalWiring,
    Electromagnet,
    IonBattery,
    MetalCoil,
    MicroSupercapacitor,
    MicroTransformer,
    Microelectrode,
    Motor,
    OpticalFibre,

    // Tech
    CarbonFibrePlating,
    EncryptedMemoryChip,
    MemoryChip,
    MicroThrusters,
    OpticalLens,
    Scrambler,
    Transmitter,
    TungstenCarbide,
    WeaponComponent,

    // Consumables
    EnergyCell,
    FragGranade,
    Medkit,
    ShieldDisruptor,
    ShieldProjector,

    // Item
    LargeCapacityPowerRegulator,

    #[cfg(not(feature = "strict"))]
    Unknown(String),
}

#[derive(Debug, Error)]
pub enum ItemError {
    #[error("Unknown item: '{0}'")]
    UnknownItem(String),
}

impl Item {
    fn name_to_item(name: &str) -> Result<Item, ItemError> {
        let lower_case: &str = &name.to_ascii_lowercase();

        Ok(match lower_case {
            "accidentlogs" => Item::AccidentLogs,
            "airqualityreports" => Item::AirQualityReports,
            "atmosphericdata" => Item::AtmosphericData,
            "blacklistdata" => Item::BlacklistData,
            "censusdata" => Item::CensusData,
            "combatantperformance" => Item::CombatantPerformance,
            "evacuationprotocols" => Item::EvacuationProtocols,
            "extractionyielddata" => Item::ExtractionYieldData,
            "factionnews" => Item::FactionNews,
            "genesequencingdata" => Item::GeneSequencingData,
            "geologicaldata" => Item::GeologicalData,
            "maintenancelogs" => Item::MaintenanceLogs,
            "networksecurityprotocols" => Item::NetworkSecurityProtocols,
            "personallogs" => Item::PersonalLogs,
            "pharmaceuticalpatents" => Item::PharmaceuticalPatents,
            "propaganda" => Item::Propaganda,
            "purchaserecords" => Item::PurchaseRecords,
            "radioactivitydata" => Item::RadioactivityData,
            "reactoroutputreview" => Item::ReactorOutputReview,
            "securityexpenses" => Item::SecurityExpenses,
            "shareholderinformation" => Item::ShareholderInformation,
            "stellaractivitylogs" => Item::StellarActivityLogs,
            "topographicalsurveys" => Item::TopographicalSurveys,
            "vaccinationrecords" => Item::VaccinationRecords,
            "virologydata" => Item::VirologyData,
            "visitorregister" => Item::VisitorRegister,
            "enhancedinterrogationrecordings" => Item::EnhancedInterrogationRecordings,

            "biologicalsample" => Item::BiologicalSample,
            "buildingschematic" => Item::BuildingSchematic,
            "californium" => Item::Californium,
            "castfossil" => Item::CastFossil,
            "compactlibrary" => Item::CompactLibrary,
            "compressionliquefiedgas" => Item::CompressionLiquefiedGas,
            "deepmantlesample" => Item::DeepMantleSample,
            "degradedpowerregulator" => Item::DegradedPowerRegulator,
            "gmeds" => Item::GMeds,
            "healthmonitor" => Item::HealthMonitor,
            "hush" => Item::Hush,
            "inertiacanister" => Item::InertiaCanister,
            "infinity" => Item::Infinity,
            "insight" => Item::Insight,
            "insightdatabank" => Item::InsightDataBank,
            "insightentertainmentsuite" => Item::InsightEntertainmentSuite,
            "ionisedgas" => Item::IonisedGas,
            "microbialinhibitor" => Item::MicrobialInhibitor,
            "nutritionalconcentrate" => Item::NutritionalConcentrate,
            "personalcomputer" => Item::PersonalComputer,
            "personaldocuments" => Item::PersonalDocuments,
            "petrifiedfossil" => Item::PetrifiedFossil,
            "powerregulator" => Item::PowerRegulator,
            "push" => Item::Push,
            "suitschematic" => Item::SuitSchematic,
            "syntheticpathogen" => Item::SyntheticPathogen,
            "universaltranslator" => Item::UniversalTranslator,
            "vehicleschematic" => Item::VehicleSchematic,
            "weaponschematic" => Item::WeaponSchematic,

            "aerogel" => Item::Aerogel,
            "chemicalcatalyst" => Item::ChemicalCatalyst,
            "chemicalsuperbase" => Item::ChemicalSuperbase,
            "epinephrine" => Item::Epinephrine,
            "epoxyadhesive" => Item::EpoxyAdhesive,
            "graphene" => Item::Graphene,
            "oxygenicbacteria" => Item::OxygenicBacteria,
            "phneutraliser" => Item::PHNeutraliser,
            "rdx" => Item::RDX,
            "viscoelasticpolymer" => Item::ViscoelasticPolymer,

            "circuitboard" => Item::CircuitBoard,
            "circuitswitch" => Item::CircuitSwitch,
            "electricalfuse" => Item::ElectricalFuse,
            "electricalwiring" => Item::ElectricalWiring,
            "electromagnet" => Item::Electromagnet,
            "ionbattery" => Item::IonBattery,
            "metalcoil" => Item::MetalCoil,
            "microsupercapacitor" => Item::MicroSupercapacitor,
            "microtransformer" => Item::MicroTransformer,
            "microelectrode" => Item::Microelectrode,
            "motor" => Item::Motor,
            "opticalfibre" => Item::OpticalFibre,

            "carbonfibreplating" => Item::CarbonFibrePlating,
            "encryptedmemorychip" => Item::EncryptedMemoryChip,
            "memorychip" => Item::MemoryChip,
            "microthrusters" => Item::MicroThrusters,
            "opticallens" => Item::OpticalLens,
            "scrambler" => Item::Scrambler,
            "transmitter" => Item::Transmitter,
            "tungstencarbide" => Item::TungstenCarbide,
            "weaponcomponent" => Item::WeaponComponent,

            "energycell" => Item::EnergyCell,
            "amm_grenade_frag" => Item::FragGranade,
            "healthpack" => Item::Medkit,
            "amm_grenade_emp" => Item::ShieldDisruptor,
            "amm_grenade_shield" => Item::ShieldProjector,

            "largecapacitypowerregulator" => Item::LargeCapacityPowerRegulator,

            #[cfg(not(feature = "strict"))]
            _ => Item::Unknown(name.to_string()),

            #[cfg(feature = "strict")]
            _ => return Err(ItemError::UnknownItem(name.to_string())),
        })
    }
}

const ITEM_NAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^\$?([a-zA-Z_]+?)(_[nN]ame;)?$"#).unwrap());

impl FromStr for Item {
    type Err = ItemError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(captures) = ITEM_NAME_REGEX.captures(s) else {
            return Err(ItemError::UnknownItem(s.to_string()));
        };

        Item::name_to_item(captures.get(1)
            .expect("Should have been captured already")
            .as_str())
    }
}

from_str_deserialize_impl!(Item);

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::models::journal_event_kind::shared::odyssey::item::Item;

    #[test]
    fn item_test_cases_are_parsed_correctly() {
        let test_cases = [
            ("$EnhancedInterrogationRecordings_Name;", Item::EnhancedInterrogationRecordings),
        ];

        for (case, expected) in test_cases {
            let result = Item::from_str(case);

            assert!(result.is_ok());
            assert_eq!(result.unwrap(), expected);
        }
    }
}
