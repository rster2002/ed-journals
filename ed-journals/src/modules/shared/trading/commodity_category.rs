use std::fmt::{Display, Formatter};
use serde::Serialize;

use crate::modules::shared::trading::commodity::Commodity;

#[derive(Debug, Serialize)]
pub enum CommodityCategory {
    Chemicals,
    ConsumerItems,
    LegalDrugs,
    Foods,
    IndustrialMaterials,
    Machinery,
    Medicines,
    Metals,
    Minerals,
    Salvage,
    Slaves,
    Technology,
    Textiles,
    Waste,
    Weapons,
    Rare,

    None,

    #[cfg(not(feature = "strict"))]
    Unknown(String),
}

impl From<Commodity> for CommodityCategory {
    fn from(value: Commodity) -> Self {
        match value {
            Commodity::ArgonomicTreatment
            | Commodity::Explosives
            | Commodity::HydrogenFuel
            | Commodity::HydrogenPeroxide
            | Commodity::LiquidOxygen
            | Commodity::MineralOil
            | Commodity::NerveAgents
            | Commodity::Pesticides
            | Commodity::RockforthFertiliser
            | Commodity::SurfaceStabilisers
            | Commodity::SyntheticReagents
            | Commodity::Tritium
            | Commodity::Water => CommodityCategory::Chemicals,

            Commodity::Clothing
            | Commodity::ConsumerTechnology
            | Commodity::DomesticAppliances
            | Commodity::EvacuationShelter
            | Commodity::SurvivalEquipment => CommodityCategory::ConsumerItems,

            Commodity::Beer
            | Commodity::BootlegLiquor
            | Commodity::Liquor
            | Commodity::Narcotics
            | Commodity::OnionheadGammaStrain
            | Commodity::Tobacco
            | Commodity::Wine => CommodityCategory::LegalDrugs,

            Commodity::Algae
            | Commodity::AnimalMeat
            | Commodity::Coffee
            | Commodity::Fish
            | Commodity::FoodCartridges
            | Commodity::FruitAndVegetables
            | Commodity::Grain
            | Commodity::SyntheticMeat
            | Commodity::Tea => CommodityCategory::Foods,

            Commodity::CeramicComposites
            | Commodity::CMMComposite
            | Commodity::InsulatingMembrane
            | Commodity::MetaAlloys
            | Commodity::MicroWeaveCoolingHoses
            | Commodity::NeofabricInsulation
            | Commodity::Polymers
            | Commodity::Semiconductors
            | Commodity::Superconductors => CommodityCategory::IndustrialMaterials,

            Commodity::ArticulationMotors
            | Commodity::AtmosphericProcessors
            | Commodity::BuildingFabricators
            | Commodity::CropHarvesters
            | Commodity::EmergencyPowerCells
            | Commodity::EnergyGridAssembly
            | Commodity::ExhaustManifold
            | Commodity::GeologicalEquipment
            | Commodity::HeatsinkInterlink
            | Commodity::HNShockMount
            | Commodity::IonDistributor
            | Commodity::MagneticEmitterCoil
            | Commodity::MarineEquipment
            | Commodity::MicrobialFurnaces
            | Commodity::MineralExtractors
            | Commodity::ModularTerminals
            | Commodity::PowerConverter
            | Commodity::PowerGenerators
            | Commodity::PowerTransferBus
            | Commodity::RadiationBaffle
            | Commodity::ReinforcedMountingPlate
            | Commodity::SkimmerComponents
            | Commodity::ThermalCoolingUnits
            | Commodity::WaterPurifiers => CommodityCategory::Machinery,

            Commodity::AdvancedMedicines
            | Commodity::AgriMedicines
            | Commodity::BasicMedicines
            | Commodity::CombatStabilisers
            | Commodity::PerformanceEnhancers
            | Commodity::ProgenitorCells => CommodityCategory::Medicines,

            Commodity::Aluminium
            | Commodity::Beryllium
            | Commodity::Bismuth
            | Commodity::Cobalt
            | Commodity::Copper
            | Commodity::Gallium
            | Commodity::Gold
            | Commodity::Hafnium178
            | Commodity::Indium
            | Commodity::Lanthanum
            | Commodity::Lithium
            | Commodity::Osmium
            | Commodity::Palladium
            | Commodity::Platinum
            | Commodity::Praseodymium
            | Commodity::Samarium
            | Commodity::Silver
            | Commodity::Tantalum
            | Commodity::Thallium
            | Commodity::Thorium
            | Commodity::Titanium
            | Commodity::Uranium => CommodityCategory::Metals,

            Commodity::Alexandrite
            | Commodity::Bauxite
            | Commodity::Benitoite
            | Commodity::Bertrandite
            | Commodity::Bromellite
            | Commodity::Coltan
            | Commodity::Cryolite
            | Commodity::Gallite
            | Commodity::Goslarite
            | Commodity::Grandidierite
            | Commodity::Indite
            | Commodity::Jadeite
            | Commodity::Lepidolite
            | Commodity::LithiumHydroxide
            | Commodity::LowTemperatureDiamonds
            | Commodity::MethaneClathrate
            | Commodity::MethanolMonohydrateCrystals
            | Commodity::Moissanite
            | Commodity::Monazite
            | Commodity::Musgravite
            | Commodity::Painite
            | Commodity::Pyrophyllite
            | Commodity::Rhodplumsite
            | Commodity::Rutile
            | Commodity::Serendibite
            | Commodity::Taaffeite
            | Commodity::Uraninite
            | Commodity::VoidOpal => CommodityCategory::Minerals,

            Commodity::AIRelics
            | Commodity::AncientArtefact
            | Commodity::AncientKey
            | Commodity::AnomalyParticles
            | Commodity::AntimatterContainmentUnit
            | Commodity::AniqueJewellery
            | Commodity::Antiquities
            | Commodity::AssaultPlans
            | Commodity::BlackBox
            | Commodity::BoneFragments
            | Commodity::CausticTissueSample
            | Commodity::CommercialSamples
            | Commodity::CoralSap
            | Commodity::CystSpecimen
            | Commodity::DamagedEscapePod
            | Commodity::DataCore
            | Commodity::DiplomaticBag
            | Commodity::EarthRelics
            | Commodity::EncryptedCorrespondence
            | Commodity::EncryptedDataStorage
            | Commodity::ExperimentalChemicals
            | Commodity::FossilRemnants
            | Commodity::GeneBank
            | Commodity::GeologicalSamples
            | Commodity::GuardianCasket
            | Commodity::GuardianOrb
            | Commodity::GuardianRelic
            | Commodity::GuardianTablet
            | Commodity::GuardianTotem
            | Commodity::GuardianUrn
            | Commodity::Hostages
            | Commodity::ImpureSpireMineral
            | Commodity::LargeSurveyDataCache
            | Commodity::MilitaryIntelligence
            | Commodity::MilitaryPlans
            | Commodity::MolluscBrainTissue
            | Commodity::MolluscFluid
            | Commodity::MolluscMembrane
            | Commodity::MolluscMycelium
            | Commodity::MolluscSoftTissue
            | Commodity::MolluscSpores
            | Commodity::MysteriousIdol
            | Commodity::OccupiedEscapePod
            | Commodity::OrganSample
            | Commodity::PersonalEffects
            | Commodity::PodCoreTissue
            | Commodity::PodDeadTissue
            | Commodity::PodMesoglea
            | Commodity::PodOuterTissue
            | Commodity::PodShellTissue
            | Commodity::PodSurfaceTissue
            | Commodity::PodTissue
            | Commodity::PoliticalPrisoners
            | Commodity::PreciousGems
            | Commodity::ProhibitedResearchMaterials
            | Commodity::ProtectiveMembraneScrap
            | Commodity::PrototypeTech
            | Commodity::RareArtwork
            | Commodity::RebelTransmission
            | Commodity::SAP8CoreContainer
            | Commodity::ScientificResearch
            | Commodity::ScientificSamples
            | Commodity::SemiRefinesSpireMineral
            | Commodity::SmallSurveyDataCache
            | Commodity::SpacePioneerRelics
            | Commodity::TacticalData
            | Commodity::TechnicalBlueprints
            | Commodity::ThargoidBasiliskTissueSample
            | Commodity::ThargoidBioStorageCapsule
            | Commodity::ThargoidBiologicalMatter
            | Commodity::ThargoidCyclopsTissueSample
            | Commodity::ThargoidGlaiveTissueSample
            | Commodity::ThargoidHeart
            | Commodity::ThargoidHydraTissueSample
            | Commodity::ThargoidLink
            | Commodity::ThargoidMedusaTissueSample
            | Commodity::ThargoidOrthrusTissueSample
            | Commodity::ThargoidGeneratorTissueSample
            | Commodity::ThargoidProbe
            | Commodity::ThargoidResin
            | Commodity::ThargoidScoutTissueSample
            | Commodity::ThargoidScytheTissueSample
            | Commodity::ThargoidSensor
            | Commodity::ThargoidTechnologySamples
            | Commodity::TimeCapsule
            | Commodity::TitanDeepTissueSample
            | Commodity::TitanDriveComponent
            | Commodity::TitanMawDeepTissueSample
            | Commodity::TitanMawPartialTissueSample
            | Commodity::TitanMawTissueSample
            | Commodity::TitanPartialTissueSample
            | Commodity::TitanTissueSample
            | Commodity::TradeData
            | Commodity::TrinketsOfHiddenFortune
            | Commodity::UnclassifiedRelic
            | Commodity::UnoccupiedEscapePod
            | Commodity::UnstableDataCore
            | Commodity::WreckageComponents => CommodityCategory::Salvage,

            Commodity::ImperialSlaves | Commodity::Slaves => CommodityCategory::Slaves,

            Commodity::AdvancedCatalysers
            | Commodity::AnimalMonitors
            | Commodity::AquaponicSystems
            | Commodity::AutoFabricators
            | Commodity::BioreducingLichen
            | Commodity::ComputerComponents
            | Commodity::HESuits
            | Commodity::HardwareDiagnosticSensor
            | Commodity::LandEnrichmentSystems
            | Commodity::MedialDiagnosticEquipment
            | Commodity::MicroControllers
            | Commodity::MuonImager
            | Commodity::Nanobreakers
            | Commodity::ResonatingSeparators
            | Commodity::Robotics
            | Commodity::StructuralRegulators
            | Commodity::TelemetrySuite => CommodityCategory::Technology,

            Commodity::ConductiveFabrics
            | Commodity::Leather
            | Commodity::MilitaryGradeFabrics
            | Commodity::NaturalFabrics
            | Commodity::SyntheticFabrics => CommodityCategory::Textiles,

            Commodity::Biowaste
            | Commodity::ChemicalWaste
            | Commodity::Scrap
            | Commodity::ToxicWaste => CommodityCategory::Waste,

            Commodity::BattleWeapons
            | Commodity::Landmines
            | Commodity::NonLethalWeapons
            | Commodity::PersonalWeapons
            | Commodity::ReactiveArmour => CommodityCategory::Weapons,

            Commodity::JaquesQuinentianStill
            | Commodity::KinagoViolins
            | Commodity::ApaVietii
            | Commodity::GeawenDanceDust
            | Commodity::VanayequiCeratomorphaFur
            | Commodity::KaretiiCouture
            | Commodity::MukusubiiChitinos
            | Commodity::UltraCompactProcessorPrototypes
            | Commodity::EleuThermals
            | Commodity::KamorinHistoricWeapons
            | Commodity::CeremonialHeikeTea
            | Commodity::VidavantianLace
            | Commodity::KachiriginFilterLeeches
            | Commodity::LyraeWeed
            | Commodity::GalacticTravelGuide
            | Commodity::HarmaSilverSeaRum
            | Commodity::NgadandariFireOpals
            | Commodity::AlyaBodySoap
            | Commodity::HelvetitjPearls
            | Commodity::OchoengChillies
            | Commodity::OnionheadBetaStrain
            | Commodity::KamitraCigars
            | Commodity::NjangariSaddles
            | Commodity::HipOrganophosphates
            | Commodity::GilyaSignatureWeapons
            | Commodity::HR7221Wheat
            | Commodity::WheemeteWheatCakes
            | Commodity::RajukruMultiStoves
            | Commodity::Nanomedicines
            | Commodity::NonEuclidianExotanks
            | Commodity::NgunaModernAntiques
            | Commodity::XiheBiomorphicCompanions
            | Commodity::EsusekuCaviar
            | Commodity::OrrerianViciousBrew
            | Commodity::VHerculisBodyRub
            | Commodity::VoidExtractCoffee
            | Commodity::UszaianTreeGrub
            | Commodity::HaidenBlackBrew
            | Commodity::MotronaExperienceJelly
            | Commodity::JaradharrePuzzleBox
            | Commodity::PersonalGifts
            | Commodity::MulachiGiantFungus
            | Commodity::LTTHyperSweet
            | Commodity::MedbStarlube
            | Commodity::GiantVerrix
            | Commodity::HIP118311Swarm
            | Commodity::DisoMaCorn
            | Commodity::LavianBrandy
            | Commodity::AzureMilk
            | Commodity::LeestianEvilJuice
            | Commodity::CoquimSpongiformVictuals
            | Commodity::LeatheryEggs
            | Commodity::ShansCharisOrchid
            | Commodity::KonggaAle
            | Commodity::VegaSlimweed
            | Commodity::TiolceWaste2PasteUnits
            | Commodity::OphiuchExinoArtefacts
            | Commodity::AltairianSkin
            | Commodity::AganippeRush
            | Commodity::CD75KittenBrandCoffee
            | Commodity::HavasupaiDreamCatcher
            | Commodity::EraninPearlWhisky
            | Commodity::PavonisEarGrubs
            | Commodity::OnionheadAlphaStrain
            | Commodity::IndiBourbon
            | Commodity::BakedGreebles
            | Commodity::KarsukiLocusts
            | Commodity::MasterChefs
            | Commodity::YasoKondiLeaf
            | Commodity::BurnhamBileDistillate
            | Commodity::TheHuttonMug
            | Commodity::CentauriMegaGin
            | Commodity::UtgaroarMillennialEggs
            | Commodity::SoontillRelics
            | Commodity::ZeesszeAntGrubGlue
            | Commodity::TheWatersOfShintara
            | Commodity::BaltahsineVacuumKrill
            | Commodity::SanumaDecorativeMeat
            | Commodity::GiantIrukamaSnails
            | Commodity::AnduligaFireWorks
            | Commodity::CrystallineSpheres
            | Commodity::PantaaPrayerSticks
            | Commodity::ChiEridaniMarinePaste
            | Commodity::EthgrezeTeaBuds
            | Commodity::DeltaPhoenicisPalms
            | Commodity::TarachSpice
            | Commodity::WulpaHyperboreSystems
            | Commodity::LiveHecateSeaWorms
            | Commodity::KoroKungPellets
            | Commodity::BastSnakeGin
            | Commodity::TerraMaterBloodBores
            | Commodity::WuthieloKuFroth
            | Commodity::HonestyPills
            | Commodity::CromSilverFesh
            | Commodity::BorasetaniPathogenetics
            | Commodity::CetiRabbits
            | Commodity::AepyornisEgg
            | Commodity::UzumokuLowGWings
            | Commodity::CherbonesBloodCrystals
            | Commodity::ToxandjiVirocide
            | Commodity::Onionhead
            | Commodity::LucanOnionhead
            | Commodity::TanmarkTranquilTea
            | Commodity::ThrutisCream
            | Commodity::AlacarakmoSkinArt
            | Commodity::PlatinumAlloy
            | Commodity::MokojingBeastFeast
            | Commodity::EdenApplesOfAerial
            | Commodity::ChameleonCloth
            | Commodity::TauriChimes
            | Commodity::RusaniOldSmokey
            | Commodity::AZCancriFormula42
            | Commodity::GomanYauponCoffee
            | Commodity::GerasianGueuzeBeer
            | Commodity::JarouaRice
            | Commodity::AnyNaCoffee
            | Commodity::FujinTea
            | Commodity::HIP10175BushMeat
            | Commodity::MomusBogSpaniel
            | Commodity::WitchhaulKobeBeef
            | Commodity::SaxonWine
            | Commodity::AroucaConventualSweets
            | Commodity::AlbinoQuechuaMammothMeat
            | Commodity::Duradrives
            | Commodity::HolvaDuellingBlades
            | Commodity::RapaBaoSnakeSkins
            | Commodity::WolfFesh
            | Commodity::EshuUmbrellas
            | Commodity::NeritusBerries
            | Commodity::JotunMookah
            | Commodity::ChateauDeAegaeon
            | Commodity::BelalansRayLeather
            | Commodity::DamnaCarapaces
            | Commodity::HIPProtoSquid
            | Commodity::MechucosHighTea
            | Commodity::DeuringasTruffles
            | Commodity::BankiAmphibiousLeather
            | Commodity::SothisCrystallineGold
            | Commodity::TiegfriesSynthSilk
            | Commodity::VolkhabBeeDrones
            | Commodity::BuckyballBeerMats
            | Commodity::ClassifiedExperimentalEquipment => CommodityCategory::Rare,

            Commodity::Limpet => CommodityCategory::None,

            #[cfg(not(feature = "strict"))]
            Commodity::Unknown(value) => {
                CommodityCategory::Unknown(format!("Unknown commodity: '{}'", value))
            }
        }
    }
}

impl Display for CommodityCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CommodityCategory::Chemicals => "Chemicals",
                CommodityCategory::ConsumerItems => "Consumer Items",
                CommodityCategory::LegalDrugs => "Legal Drugs",
                CommodityCategory::Foods => "Foods",
                CommodityCategory::IndustrialMaterials => "Industrial Materials",
                CommodityCategory::Machinery => "Machinery",
                CommodityCategory::Medicines => "Medicines",
                CommodityCategory::Metals => "Metals",
                CommodityCategory::Minerals => "Minerals",
                CommodityCategory::Salvage => "Salvage",
                CommodityCategory::Slaves => "Slaves",
                CommodityCategory::Technology => "Technology",
                CommodityCategory::Textiles => "Textiles",
                CommodityCategory::Waste => "Waste",
                CommodityCategory::Weapons => "Weapons",
                CommodityCategory::Rare => "Rare",
                CommodityCategory::None => "None",

                #[cfg(not(feature = "strict"))]
                CommodityCategory::Unknown(unknown) => unknown,
            }
        )
    }
}
