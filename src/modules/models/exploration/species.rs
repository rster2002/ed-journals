use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde::{Serialize, Deserialize};
use serde_json::Value;
use crate::modules::exobiology::models::spawn_condition::SpawnCondition;
use crate::modules::models::galaxy::atmosphere::{Atmosphere, AtmosphereDensity};
use crate::modules::models::galaxy::atmosphere_type::AtmosphereType;
use crate::modules::models::galaxy::planet_class::PlanetClass;
use crate::modules::models::galaxy::star_class::StarClass;
use crate::modules::models::galaxy::star_luminosity::StarLuminosity;
use crate::modules::models::galaxy::volcanism_type::VolcanismType;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Species {
    // Aleoids
    AleoidaLaminiae,
    AleoidaGravis,

    #[serde(rename = "$Codex_Ent_Aleoids_03_Name;")]
    AleoidaSpica,

    #[serde(rename = "$Codex_Ent_Aleoids_02_Name;")]
    AleoidaCoronamus,

    #[serde(rename = "$Codex_Ent_Aleoids_01_Name;")]
    AleoidaArcus,

    // Amphora plant
    AmphoraPlant,

    // Anemones
    AnemonePrasinus,
    AnemonePrasinumBioluminescent,
    AnemonePuniceus,
    AnemonePuniceum,

    AnemoneRoseus,
    AnemoneRoseumBioluminescent,
    AnemoneRoseum,
    AnemoneBlattinus,
    AnemoneBlatteumBioluminescent,
    AnemoneLuteus,
    AnemoneLuteolum,
    AnemoneRubens,
    AnemoneRubeumBioluminescent,
    AnemoneCroceus,
    AnemoneCroceum,

    // Bark mounds
    BarkMound,

    // Bacterium
    BacteriumNebulus,
    BacteriumAcies,
    BacteriumOmentum,
    BacteriumScopulum,
    BacteriumVerrata,
    BacteriumBullaris,

    #[serde(rename = "$Codex_Ent_Bacterial_06_Name;")]
    BacteriumAlcyoneum,

    #[serde(rename = "$Codex_Ent_Bacterial_05_Name;")]
    BacteriumVesicula,

    #[serde(rename = "$Codex_Ent_Bacterial_12_Name;")]
    BacteriumCerbrus,

    #[serde(rename = "$Codex_Ent_Bacterial_01_Name;")]
    BacteriumAurasus,

    #[serde(rename = "$Codex_Ent_Bacterial_08_Name;")]
    BacteriumInformem,

    #[serde(rename = "$Codex_Ent_Bacterial_09_Name;")]
    BacteriumVolu,

    #[serde(rename = "$Codex_Ent_Bacterial_07_Name;")]
    BacteriumTela,

    // Brain tree
    BrainTreeAureum,
    BrainTreeOstrinum,
    BrainTreePuniceum,
    BrainTreeLindigoticum,
    BrainTreeGypseeum,
    BrainTreeLividum,
    BrainTreeViride,
    BrainTreeRoseum,

    // Cactoida
    #[serde(rename = "$Codex_Ent_Cactoid_02_Name;")]
    CactoidaLapis,

    #[serde(rename = "$Codex_Ent_Cactoid_04_Name;")]
    CactoidaPullulanta,

    CactoidaCortexum,
    CactoidaVermis,
    CactoidaPeperatis,

    // Clypeus
    #[serde(rename = "$Codex_Ent_Clypeus_03_Name;")]
    ClypeusSpeculumi,

    ClypeusLacrimam,
    ClypeusMargaritus,

    // Conchas
    #[serde(rename = "$Codex_Ent_Conchas_01_Name;")]
    ConchaRenibus,

    #[serde(rename = "$Codex_Ent_Conchas_02_Name;")]
    ConchaAureolas,

    #[serde(rename = "$Codex_Ent_Conchas_03_Name;")]
    ConchaLabiata,

    #[serde(rename = "$Codex_Ent_Conchas_04_Name;")]
    ConchaBiconcavis,

    CrystallineShards,

    // Electricae
    #[serde(rename = "$Codex_Ent_Electricae_01_Name;")]
    ElectricaePluma,
    ElectricaeRadialem,

    // Fonticulus
    #[serde(rename = "$Codex_Ent_Fonticulus_02_Name;")]
    FonticuluaCampestris,

    FonticuluaSegmentatus,
    FonticuluaDigitos,

    #[serde(rename = "$Codex_Ent_Fonticulus_03_Name;")]
    FonticuluaUpupam,
    FonticuluaLapida,
    FonticuluaFluctus,

    // Frutexa
    #[serde(rename = "$Codex_Ent_Shrubs_01_Name;")]
    FrutexaFlabellum,

    FrutexaFlammasis,

    #[serde(rename = "$Codex_Ent_Shrubs_03_Name;")]
    FrutexaMetallicum,
    FrutexaAcus,
    FrutexaFera,
    FrutexaSponsae,
    FrutexaCollum,

    // Fumerola
    FumerolaAquatis,
    FumerolaCarbosis,
    FumerolaExtremus,
    FumerolaNitris,

    // Fungoida
    #[serde(rename = "$Codex_Ent_Fungoids_01_Name;")]
    FungoidaSetisis,

    #[serde(rename = "$Codex_Ent_Fungoids_02_Name;")]
    FungoidaStabitis,

    #[serde(rename = "$Codex_Ent_Fungoids_03_Name;")]
    FungoidaBullarum,

    #[serde(rename = "$Codex_Ent_Fungoids_04_Name;")]
    FungoidaGelata,

    // Osseus
    #[serde(rename = "$Codex_Ent_Osseus_01_Name;")]
    OsseusFractus,

    #[serde(rename = "$Codex_Ent_Osseus_03_Name;")]
    OsseusSpiralis,

    #[serde(rename = "$Codex_Ent_Osseus_04_Name;")]
    OsseusPumice,

    #[serde(rename = "$Codex_Ent_Osseus_05_Name;")]
    OsseusCornibus,

    OsseusPellebantus,
    OsseusDiscus,

    // Recepta
    #[serde(rename = "$Codex_Ent_Recepta_01_Name;")]
    ReceptaUmbrux,

    #[serde(rename = "$Codex_Ent_Recepta_02_Name;")]
    ReceptaDeltahedronix,

    #[serde(rename = "$Codex_Ent_Recepta_03_Name;")]
    ReceptaConditivus,

    // Sinuous tubers
    SinuousTubersAlbidum,
    SinuousTubersBlatteum,
    SinuousTubersCaeruleum,
    SinuousTubersLindigoticum,
    SinuousTubersPrasinum,
    SinuousTubersRoseum,
    SinuousTubersViolaceum,
    SinuousTubersViride,

    // Stratum
    #[serde(rename = "$Codex_Ent_Stratum_07_Name;")]
    StratumTectonicas,

    #[serde(rename = "$Codex_Ent_Stratum_02_Name;")]
    StratumPaleas,

    #[serde(rename = "$Codex_Ent_Stratum_08_Name;")]
    StratumFrigus,

    StratumLaminamus,
    StratumExcutitus,
    StratumLimaxus,
    StratumCucumisis,
    StratumAraneamus,

    // Tubus
    #[serde(rename = "$Codex_Ent_Tubus_01_Name;")]
    TubusConifer,

    TubusSororibus,
    TubusRosarium,
    TubusCavas,
    TubusCompagibus,

    // Tussock
    #[serde(rename = "$Codex_Ent_Tussocks_01_Name;")]
    TussockPennata,

    #[serde(rename = "$Codex_Ent_Tussocks_02_Name;")]
    TussockVentusa,

    #[serde(rename = "$Codex_Ent_Tussocks_03_Name;")]
    TussockIgnis,

    #[serde(rename = "$Codex_Ent_Tussocks_04_Name;")]
    TussockCultro,

    #[serde(rename = "$Codex_Ent_Tussocks_07_Name;")]
    TussockSerrati,

    #[serde(rename = "$Codex_Ent_Tussocks_08_Name;")]
    TussockAlbata,

    #[serde(rename = "$Codex_Ent_Tussocks_10_Name;")]
    TussockDivisa,

    #[serde(rename = "$Codex_Ent_Tussocks_11_Name;")]
    TussockCaputus,

    #[serde(rename = "$Codex_Ent_Tussocks_12_Name;")]
    TussockTriticum,

    #[serde(rename = "$Codex_Ent_Tussocks_13_Name;")]
    TussockStigmasis,

    #[serde(rename = "$Codex_Ent_Tussocks_15_Name;")]
    TussockCapillum,
    TussockCatena,
    TussockPropagito,
    TussockPennatis,
    TussockVirgam,
}

impl FromStr for Species {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_value(Value::String(s.to_string()))
    }
}

impl Display for Species {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Species::AleoidaArcus => "Aleoida Arcus",
                Species::AleoidaCoronamus => "Aleoida Coronamus",
                Species::AleoidaGravis => "Aleoida Gravis",
                Species::AleoidaLaminiae => "Aleoida Laminiae",
                Species::AleoidaSpica => "Aleoida Spica",
                Species::AmphoraPlant => "Amphora Plant",

                Species::AnemoneBlatteumBioluminescent => "Anemone Blatteum Bioluminescent",
                Species::AnemoneBlattinus => "Anemone Blattinus Bioluminescent",
                Species::AnemoneCroceum => "Anemone Croceum",
                Species::AnemoneCroceus => "Anemone Croceus",
                Species::AnemoneLuteolum => "Anemone Luteolum",
                Species::AnemoneLuteus => "Anemone Luteus",
                Species::AnemonePrasinumBioluminescent => "Anemone Prasinum Bioluminescent",
                Species::AnemonePrasinus => "Anemone Prasinus Bioluminescent",
                Species::AnemonePuniceum => "Anemone Puniceum",
                Species::AnemonePuniceus => "Anemone Puniceus",
                Species::AnemoneRoseum => "Anemone Roseum",
                Species::AnemoneRoseumBioluminescent => "Anemone Roseum Bioluminescent",
                Species::AnemoneRoseus => "Anemone Roseus",
                Species::AnemoneRubens => "Anemone Rubens Bioluminescent",
                Species::AnemoneRubeumBioluminescent => "Anemone Rubeum Bioluminescent",

                Species::BarkMound => "Bark Mound",

                Species::BacteriumAcies => "Bacterium Acies",
                Species::BacteriumAlcyoneum => "Bacterium Alcyoneum",
                Species::BacteriumAurasus => "Bacterium Aurasus",
                Species::BacteriumBullaris => "Bacterium Bullaris",
                Species::BacteriumCerbrus => "Bacterium Cerbrus",
                Species::BacteriumInformem => "Bacterium Informem",
                Species::BacteriumNebulus => "Bacterium Nebulus",
                Species::BacteriumOmentum => "Bacterium Omentum",
                Species::BacteriumScopulum => "Bacterium Scopulum",
                Species::BacteriumTela => "Bacterium Tela",
                Species::BacteriumVerrata => "Bacterium Verrata",
                Species::BacteriumVesicula => "Bacterium Vesicula",
                Species::BacteriumVolu => "Bacterium Volu",

                Species::BrainTreeAureum => "Brain Tree Aureum",
                Species::BrainTreeGypseeum => "Brain Tree Gypseeum",
                Species::BrainTreeLindigoticum => "Brain Tree Lindigoticum",
                Species::BrainTreeLividum => "Brain Tree Lividum",
                Species::BrainTreeOstrinum => "Brain Tree Ostrinum",
                Species::BrainTreePuniceum => "Brain Tree Puniceum",
                Species::BrainTreeRoseum => "Brain Tree Roseum",
                Species::BrainTreeViride => "Brain Tree Viride",

                Species::CactoidaCortexum => "Cactoida Cortexum",
                Species::CactoidaLapis => "Cactoida Lapis",
                Species::CactoidaPullulanta => "Cactoida Pullulanta",
                Species::CactoidaVermis => "Cactoida Vermis",
                Species::CactoidaPeperatis => "Cactoida Peperatis",

                Species::CrystallineShards => "Crystaline Shards",

                Species::ClypeusLacrimam => "Clypeus Lacrimam",
                Species::ClypeusMargaritus => "Clypeus Margaritus",
                Species::ClypeusSpeculumi => "Clypeus Speculumi",

                Species::ConchaAureolas => "Concha Aureolas",
                Species::ConchaBiconcavis => "Concha Biconcavis",
                Species::ConchaLabiata => "Concha Labiata",
                Species::ConchaRenibus => "Concha Renibus",

                Species::ElectricaePluma => "Electricae Pluma",
                Species::ElectricaeRadialem => "Electricae Radialem",

                Species::FonticuluaCampestris => "Fonticulua Campestris",
                Species::FonticuluaDigitos => "Fonticulua Digitos",
                Species::FonticuluaFluctus => "Fonticulua Fluctus",
                Species::FonticuluaLapida => "Fonticulua Lapida",
                Species::FonticuluaSegmentatus => "Fonticulua Segmentatus",
                Species::FonticuluaUpupam => "Fonticulua Upupam",

                Species::FrutexaAcus => "Frutexa Acus",
                Species::FrutexaCollum => "Frutexa Collum",
                Species::FrutexaFera => "Frutexa Fera",
                Species::FrutexaFlabellum => "Frutexa Flabellum",
                Species::FrutexaFlammasis => "Frutexa Flammasis",
                Species::FrutexaMetallicum => "Frutexa Metallicum",
                Species::FrutexaSponsae => "Frutexa Sponsae",

                Species::FumerolaAquatis => "Fumerola Aquatis",
                Species::FumerolaCarbosis => "Fumerola Carbosis",
                Species::FumerolaExtremus => "Fumerola Extremus",
                Species::FumerolaNitris => "Fumerola Nitris",

                Species::FungoidaBullarum => "Fungoida Bullarum",
                Species::FungoidaGelata => "Fungoida Gelata",
                Species::FungoidaSetisis => "Fungoida Setisis",
                Species::FungoidaStabitis => "Fungoida Stabitis",

                Species::OsseusCornibus => "Osseus Cornibus",
                Species::OsseusDiscus => "Osseus Discus",
                Species::OsseusFractus => "Osseus Fractus",
                Species::OsseusPellebantus => "Osseus Pellebantus",
                Species::OsseusPumice => "Osseus Pumice",
                Species::OsseusSpiralis => "Osseus Spiralis",

                Species::ReceptaConditivus => "Recepta Conditivus",
                Species::ReceptaDeltahedronix => "Recepta Deltahedronix",
                Species::ReceptaUmbrux => "Recepta Umbrux",

                Species::SinuousTubersAlbidum => "Sinuous Tubers Albidum",
                Species::SinuousTubersBlatteum => "Sinuous Tubers Blatteum",
                Species::SinuousTubersCaeruleum => "Sinuous Tubers Caeruleum",
                Species::SinuousTubersLindigoticum => "Sinuous Tubers Lindigoticum",
                Species::SinuousTubersPrasinum => "Sinuous Tubers Prasinum",
                Species::SinuousTubersRoseum => "Sinuous Tubers Roseum",
                Species::SinuousTubersViolaceum => "Sinuous Tubers Violaceum",
                Species::SinuousTubersViride => "Sinuous Tubers Viride",

                Species::StratumAraneamus => "Stratum Araneamus",
                Species::StratumCucumisis => "Stratum Cucumisis",
                Species::StratumExcutitus => "Stratum Excutitus",
                Species::StratumFrigus => "Stratum Frigus",
                Species::StratumLaminamus => "Stratum Laminamus",
                Species::StratumLimaxus => "Stratum Limaxus",
                Species::StratumPaleas => "Stratum Paleas",
                Species::StratumTectonicas => "Stratum Tectonicas",

                Species::TubusCavas => "Tubus Cavas",
                Species::TubusCompagibus => "Tubus Compagibus",
                Species::TubusConifer => "Tubus Conifer",
                Species::TubusRosarium => "Tubus Rosarium",
                Species::TubusSororibus => "Tubus Sororibus",

                Species::TussockAlbata => "Tussock Albata",
                Species::TussockCapillum => "Tussock Capillum",
                Species::TussockCaputus => "Tussock Caputus",
                Species::TussockCatena => "Tussock Catena",
                Species::TussockCultro => "Tussock Cultro",
                Species::TussockDivisa => "Tussock Divisa",
                Species::TussockIgnis => "Tussock Ignis",
                Species::TussockPennata => "Tussock Pennata",
                Species::TussockPennatis => "Tussock Pennatis",
                Species::TussockPropagito => "Tussock Propagito",
                Species::TussockSerrati => "Tussock Serrati",
                Species::TussockStigmasis => "Tussock Stigmasis",
                Species::TussockTriticum => "Tussock Triticum",
                Species::TussockVentusa => "Tussock Ventusa",
                Species::TussockVirgam => "Tussock Virgam",
            }
        )
    }
}

impl Species {
    pub fn spawn_conditions(&self) -> Vec<SpawnCondition> {
        match self {
            Species::AleoidaArcus => vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
                SpawnCondition::MaxGravity(0.27),
                SpawnCondition::MinMeanTemperature(175.0),
                SpawnCondition::MaxMeanTemperature(180.0),
            ],
            Species::AleoidaCoronamus => vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
                SpawnCondition::MaxGravity(0.27),
                SpawnCondition::MinMeanTemperature(180.0),
                SpawnCondition::MaxMeanTemperature(190.0),
            ],
            Species::AleoidaGravis => vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
                SpawnCondition::MaxGravity(0.27),
                SpawnCondition::MinMeanTemperature(190.0),
                SpawnCondition::MaxMeanTemperature(195.0),
            ],
            Species::AleoidaLaminiae => vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::Ammonia),
                SpawnCondition::MaxGravity(0.27),
            ],
            Species::AleoidaSpica => vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::Ammonia),
                SpawnCondition::MaxGravity(0.27),
            ],

            Species::AmphoraPlant => vec![
                SpawnCondition::ParentStarClass(StarClass::A),
                SpawnCondition::NoAtmosphere,
                SpawnCondition::PlanetClass(PlanetClass::MetalRichBody),
                SpawnCondition::Any(vec![
                    SpawnCondition::SystemContainsPlanetClass(PlanetClass::EarthlikeBody),
                    SpawnCondition::SystemContainsPlanetClass(PlanetClass::AmmoniaWorld),
                    SpawnCondition::SystemContainsPlanetClass(PlanetClass::GasGiantWithWaterBasedLife),
                    SpawnCondition::SystemContainsPlanetClass(PlanetClass::GasGiantWithAmmoniaBasedLife),
                    SpawnCondition::SystemContainsPlanetClass(PlanetClass::WaterGiant),
                ]),
            ],

            Species::AnemonePrasinus => vec![
                SpawnCondition::NoAtmosphere,
                SpawnCondition::ParentStarClass(StarClass::O),
                SpawnCondition::Any(vec![
                    SpawnCondition::PlanetClass(PlanetClass::MetalRichBody),
                    SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
                    SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                ]),
            ],
            Species::AnemonePrasinumBioluminescent => vec![
                SpawnCondition::NoAtmosphere,
                SpawnCondition::ParentStarClass(StarClass::O),
                SpawnCondition::Any(vec![
                    SpawnCondition::PlanetClass(PlanetClass::MetalRichBody),
                    SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
                    SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                ]),
            ],
            Species::AnemonePuniceus => vec![
                SpawnCondition::NoAtmosphere,
                SpawnCondition::ParentStarClass(StarClass::O),
                SpawnCondition::Any(vec![
                    SpawnCondition::PlanetClass(PlanetClass::IcyBody),
                    SpawnCondition::PlanetClass(PlanetClass::RockyIceBody),
                ]),
            ],
            Species::AnemonePuniceum => vec![
                SpawnCondition::NoAtmosphere,
                SpawnCondition::ParentStarClass(StarClass::O),
                SpawnCondition::Any(vec![
                    SpawnCondition::PlanetClass(PlanetClass::IcyBody),
                    SpawnCondition::PlanetClass(PlanetClass::RockyIceBody),
                ]),
            ],
            Species::AnemoneRoseus => vec![
                SpawnCondition::NoAtmosphere,
                SpawnCondition::ParentStarClass(StarClass::B),
                SpawnCondition::Any(vec![
                    SpawnCondition::ParentStarLuminosity(StarLuminosity::I),
                    SpawnCondition::ParentStarLuminosity(StarLuminosity::II),
                    SpawnCondition::ParentStarLuminosity(StarLuminosity::III),
                ]),
                SpawnCondition::Any(vec![
                    SpawnCondition::PlanetClass(PlanetClass::MetalRichBody),
                    SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
                    SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                ]),
            ],
            Species::AnemoneRoseumBioluminescent => vec![
                SpawnCondition::NoAtmosphere,
                SpawnCondition::ParentStarClass(StarClass::B),
                SpawnCondition::Any(vec![
                    SpawnCondition::ParentStarLuminosity(StarLuminosity::I),
                    SpawnCondition::ParentStarLuminosity(StarLuminosity::II),
                    SpawnCondition::ParentStarLuminosity(StarLuminosity::III),
                ]),
                SpawnCondition::Any(vec![
                    SpawnCondition::PlanetClass(PlanetClass::MetalRichBody),
                    SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
                ]),
            ],
            Species::AnemoneRoseum => vec![
                SpawnCondition::NoAtmosphere,
                SpawnCondition::ParentStarClass(StarClass::B),
                SpawnCondition::Any(vec![
                    SpawnCondition::ParentStarLuminosity(StarLuminosity::I),
                    SpawnCondition::ParentStarLuminosity(StarLuminosity::II),
                    SpawnCondition::ParentStarLuminosity(StarLuminosity::III),
                ]),
                SpawnCondition::Any(vec![
                    SpawnCondition::PlanetClass(PlanetClass::MetalRichBody),
                    SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
                ]),
            ],
            Species::AnemoneBlattinus => vec![
                SpawnCondition::NoAtmosphere,
                SpawnCondition::ParentStarClass(StarClass::B),
                SpawnCondition::Any(vec![
                    SpawnCondition::ParentStarLuminosity(StarLuminosity::IV),
                    SpawnCondition::ParentStarLuminosity(StarLuminosity::V),
                ]),
                SpawnCondition::Any(vec![
                    SpawnCondition::PlanetClass(PlanetClass::MetalRichBody),
                    SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
                ]),
            ],
            Species::AnemoneBlatteumBioluminescent => vec![
                SpawnCondition::NoAtmosphere,
                SpawnCondition::ParentStarClass(StarClass::B),
                SpawnCondition::Any(vec![
                    SpawnCondition::ParentStarLuminosity(StarLuminosity::IV),
                    SpawnCondition::ParentStarLuminosity(StarLuminosity::V),
                ]),
                SpawnCondition::Any(vec![
                    SpawnCondition::PlanetClass(PlanetClass::MetalRichBody),
                    SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
                ]),
            ],
            Species::AnemoneLuteus => vec![
                SpawnCondition::NoAtmosphere,
                SpawnCondition::ParentStarClass(StarClass::B),
                SpawnCondition::Any(vec![
                    SpawnCondition::ParentStarLuminosity(StarLuminosity::IV),
                    SpawnCondition::ParentStarLuminosity(StarLuminosity::V),
                ]),
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
            ],
            Species::AnemoneLuteolum => vec![
                SpawnCondition::NoAtmosphere,
                SpawnCondition::ParentStarClass(StarClass::B),
                SpawnCondition::Any(vec![
                    SpawnCondition::ParentStarLuminosity(StarLuminosity::IV),
                    SpawnCondition::ParentStarLuminosity(StarLuminosity::V),
                ]),
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
            ],
            Species::AnemoneRubens => vec![
                SpawnCondition::NoAtmosphere,
                SpawnCondition::Any(vec![
                    SpawnCondition::All(vec![
                        SpawnCondition::ParentStarClass(StarClass::B),
                        SpawnCondition::ParentStarLuminosity(StarLuminosity::IV),
                    ]),
                    SpawnCondition::All(vec![
                        SpawnCondition::ParentStarClass(StarClass::A),
                        SpawnCondition::ParentStarLuminosity(StarLuminosity::III),
                    ]),
                ]),
                SpawnCondition::Any(vec![
                    SpawnCondition::PlanetClass(PlanetClass::MetalRichBody),
                    SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
                ]),
            ],
            Species::AnemoneRubeumBioluminescent => vec![
                SpawnCondition::NoAtmosphere,
                SpawnCondition::ParentStarClass(StarClass::B),
                SpawnCondition::Any(vec![
                    SpawnCondition::PlanetClass(PlanetClass::MetalRichBody),
                    SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
                ]),
            ],
            Species::AnemoneCroceus => vec![
                SpawnCondition::NoAtmosphere,
                SpawnCondition::ParentStarClass(StarClass::B),
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
            ],
            Species::AnemoneCroceum => vec![
                SpawnCondition::NoAtmosphere,
                SpawnCondition::ParentStarClass(StarClass::B),
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
            ],

            Species::BarkMound => vec![
                SpawnCondition::NoAtmosphere,
                SpawnCondition::WithinNebulaRange(150.0),
            ],

            Species::BacteriumNebulus => vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::Helium),
            ],
            Species::BacteriumAcies => vec![
                SpawnCondition::Any(vec![
                    SpawnCondition::ThinAtmosphere(AtmosphereType::Neon),
                    SpawnCondition::ThinAtmosphere(AtmosphereType::NeonRich),
                ]),
            ],
            Species::BacteriumOmentum => vec![
                SpawnCondition::Any(vec![
                    SpawnCondition::ThinAtmosphere(AtmosphereType::Neon),
                    SpawnCondition::ThinAtmosphere(AtmosphereType::NeonRich),
                ]),
                SpawnCondition::Any(vec![
                    SpawnCondition::VolcanismType(VolcanismType::NitrogenMagma),
                    SpawnCondition::VolcanismType(VolcanismType::NitrogenGeysers),
                    SpawnCondition::VolcanismType(VolcanismType::AmmoniaMagma),
                    SpawnCondition::VolcanismType(VolcanismType::AmmoniaGeysers),
                ]),
            ],
            Species::BacteriumScopulum => vec![
                SpawnCondition::Any(vec![
                    SpawnCondition::ThinAtmosphere(AtmosphereType::Neon),
                    SpawnCondition::ThinAtmosphere(AtmosphereType::NeonRich),
                ]),
                SpawnCondition::Any(vec![
                    SpawnCondition::VolcanismType(VolcanismType::CarbonDioxideGeysers),
                    SpawnCondition::VolcanismType(VolcanismType::MethaneGeysers),
                    SpawnCondition::VolcanismType(VolcanismType::MethaneMagma),
                ]),
            ],
            Species::BacteriumVerrata => vec![
                SpawnCondition::Any(vec![
                    SpawnCondition::ThinAtmosphere(AtmosphereType::Neon),
                    SpawnCondition::ThinAtmosphere(AtmosphereType::NeonRich),
                ]),
                SpawnCondition::Any(vec![
                    SpawnCondition::VolcanismType(VolcanismType::WaterMagma),
                    SpawnCondition::VolcanismType(VolcanismType::WaterGeysers),
                ]),
            ],
            Species::BacteriumBullaris => vec![
                SpawnCondition::Any(vec![
                    SpawnCondition::ThinAtmosphere(AtmosphereType::Methane),
                    SpawnCondition::ThinAtmosphere(AtmosphereType::MethaneRich),
                ]),
            ],
            Species::BacteriumAlcyoneum => vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::Ammonia),
            ],
            Species::BacteriumVesicula => vec![
                SpawnCondition::Any(vec![
                    SpawnCondition::ThinAtmosphere(AtmosphereType::Argon),
                    SpawnCondition::ThinAtmosphere(AtmosphereType::ArgonRich),
                ]),
            ],
            Species::BacteriumCerbrus => vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::SulfurDioxide),
            ],
            Species::BacteriumAurasus => vec![
                SpawnCondition::Any(vec![
                    SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
                    SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxideRich),
                ]),
            ],
            Species::BacteriumInformem => vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::Nitrogen),
            ],
            Species::BacteriumVolu => vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::Oxygen),
            ],
            Species::BacteriumTela => vec![
                SpawnCondition::Any(vec![
                    SpawnCondition::VolcanismType(VolcanismType::HeliumGeysers),
                    SpawnCondition::VolcanismType(VolcanismType::SilicateMagma),
                    SpawnCondition::VolcanismType(VolcanismType::SilicateVapourGeysers),
                ]),
            ],

            Species::BrainTreeAureum => vec![
                SpawnCondition::NoAtmosphere,
                SpawnCondition::AnyVolcanism,
            ],
            Species::BrainTreeOstrinum => vec![
                SpawnCondition::NoAtmosphere,
                SpawnCondition::AnyVolcanism,
            ],
            Species::BrainTreePuniceum => vec![
                SpawnCondition::NoAtmosphere,
                SpawnCondition::AnyVolcanism,
            ],
            Species::BrainTreeLindigoticum => vec![
                SpawnCondition::NoAtmosphere,
                SpawnCondition::AnyVolcanism,
            ],
            Species::BrainTreeGypseeum => vec![
                SpawnCondition::NoAtmosphere,
                SpawnCondition::AnyVolcanism,
            ],
            Species::BrainTreeLividum => vec![
                SpawnCondition::NoAtmosphere,
                SpawnCondition::AnyVolcanism,
            ],
            Species::BrainTreeViride => vec![
                SpawnCondition::NoAtmosphere,
                SpawnCondition::AnyVolcanism,
            ],
            Species::BrainTreeRoseum => vec![
                SpawnCondition::NoAtmosphere,
                SpawnCondition::AnyVolcanism,
            ],

            Species::CactoidaLapis => vec![
                SpawnCondition::MaxGravity(0.27),
                SpawnCondition::Any(vec![
                    SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                    SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
                ]),
                SpawnCondition::ThinAtmosphere(AtmosphereType::Ammonia),
            ],
            Species::CactoidaPullulanta => vec![
                SpawnCondition::MaxGravity(0.27),
                SpawnCondition::Any(vec![
                    SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                    SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
                ]),
                SpawnCondition::Any(vec![
                    SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
                    SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxideRich),
                ]),
                SpawnCondition::MinMeanTemperature(180.0),
                SpawnCondition::MaxMeanTemperature(195.0),
            ],
            Species::CactoidaCortexum => vec![
                SpawnCondition::MaxGravity(0.27),
                SpawnCondition::Any(vec![
                    SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                    SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
                ]),
                SpawnCondition::Any(vec![
                    SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
                    SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxideRich),
                ]),
                SpawnCondition::MinMeanTemperature(180.0),
                SpawnCondition::MaxMeanTemperature(195.0),
            ],
            Species::CactoidaVermis => vec![
                SpawnCondition::MaxGravity(0.27),
                SpawnCondition::Any(vec![
                    SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                    SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
                ]),
                SpawnCondition::ThinAtmosphere(AtmosphereType::Water),
            ],
            Species::CactoidaPeperatis => vec![
                SpawnCondition::MaxGravity(0.27),
                SpawnCondition::Any(vec![
                    SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                    SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
                ]),
                SpawnCondition::ThinAtmosphere(AtmosphereType::Ammonia),
            ],

            Species::ClypeusSpeculumi => vec![
                SpawnCondition::MaxGravity(0.27),
                SpawnCondition::MinMeanTemperature(190.0),
                SpawnCondition::Any(vec![
                    SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
                    SpawnCondition::ThinAtmosphere(AtmosphereType::Water),
                ]),
                SpawnCondition::MinDistanceFromParentSun(5.0),
            ],
            Species::ClypeusLacrimam => vec![
                SpawnCondition::MaxGravity(0.27),
                SpawnCondition::MinMeanTemperature(190.0),
                SpawnCondition::Any(vec![
                    SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
                    SpawnCondition::ThinAtmosphere(AtmosphereType::Water),
                ]),
            ],
            Species::ClypeusMargaritus => vec![
                SpawnCondition::MaxGravity(0.27),
                SpawnCondition::MinMeanTemperature(190.0),
                SpawnCondition::Any(vec![
                    SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
                    SpawnCondition::ThinAtmosphere(AtmosphereType::Water),
                ]),
            ],

            Species::ConchaRenibus => vec![
                SpawnCondition::Any(vec![
                    SpawnCondition::All(vec![
                        SpawnCondition::MaxGravity(0.27),
                        SpawnCondition::Any(vec![
                            SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
                            SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxideRich),
                        ]),
                        SpawnCondition::MinMeanTemperature(180.0),
                        SpawnCondition::MaxMeanTemperature(195.0),
                    ]),
                    SpawnCondition::All(vec![
                        SpawnCondition::MaxGravity(0.27),
                        SpawnCondition::Any(vec![
                            SpawnCondition::ThinAtmosphere(AtmosphereType::Water),
                            SpawnCondition::ThinAtmosphere(AtmosphereType::WaterRich),
                        ]),
                    ]),
                ]),
            ],
            Species::ConchaAureolas => vec![
                SpawnCondition::MaxGravity(0.27),
                SpawnCondition::ThinAtmosphere(AtmosphereType::Ammonia),
            ],
            Species::ConchaLabiata => vec![
                SpawnCondition::MaxGravity(0.27),
                SpawnCondition::Any(vec![
                    SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
                    SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxideRich),
                ]),
                SpawnCondition::MaxMeanTemperature(190.0),
            ],
            Species::ConchaBiconcavis => vec![
                SpawnCondition::MaxGravity(0.27),
                SpawnCondition::ThinAtmosphere(AtmosphereType::Nitrogen),
            ],

            Species::CrystallineShards => vec![
                SpawnCondition::NoAtmosphere,
                SpawnCondition::Any(vec![
                    SpawnCondition::ParentStarClass(StarClass::A),
                    SpawnCondition::ParentStarClass(StarClass::F),
                    SpawnCondition::ParentStarClass(StarClass::G),
                    SpawnCondition::ParentStarClass(StarClass::K),
                    SpawnCondition::ParentStarClass(StarClass::M),
                    SpawnCondition::ParentStarClass(StarClass::S),
                ]),
                SpawnCondition::MinMeanTemperature(0.0),
                SpawnCondition::MaxMeanTemperature(273.0),
                SpawnCondition::Any(vec![
                    SpawnCondition::SystemContainsPlanetClass(PlanetClass::EarthlikeBody),
                    SpawnCondition::SystemContainsPlanetClass(PlanetClass::AmmoniaWorld),
                    SpawnCondition::SystemContainsPlanetClass(PlanetClass::GasGiantWithWaterBasedLife),
                    SpawnCondition::SystemContainsPlanetClass(PlanetClass::GasGiantWithAmmoniaBasedLife),
                    SpawnCondition::SystemContainsPlanetClass(PlanetClass::WaterGiant),
                ]),
                SpawnCondition::MinDistanceFromParentSun(24.0),
            ],

            Species::ElectricaePluma => vec![
                SpawnCondition::Any(vec![
                    SpawnCondition::ThinAtmosphere(AtmosphereType::Helium),
                    SpawnCondition::ThinAtmosphere(AtmosphereType::Neon),
                    SpawnCondition::ThinAtmosphere(AtmosphereType::Argon),
                ]),
                SpawnCondition::PlanetClass(PlanetClass::IcyBody),
                SpawnCondition::ParentStarClass(StarClass::A),
                SpawnCondition::Any(vec![
                    SpawnCondition::MinOrEqualParentStarLuminosity(StarLuminosity::V),
                    SpawnCondition::ParentStarClass(StarClass::N),
                ]),
                SpawnCondition::MaxGravity(0.27),
            ],
            Species::ElectricaeRadialem => vec![
                SpawnCondition::Any(vec![
                    SpawnCondition::ThinAtmosphere(AtmosphereType::Helium),
                    SpawnCondition::ThinAtmosphere(AtmosphereType::Neon),
                    SpawnCondition::ThinAtmosphere(AtmosphereType::Argon),
                ]),
                SpawnCondition::PlanetClass(PlanetClass::IcyBody),
                SpawnCondition::WithinNebulaRange(150.0),
                SpawnCondition::MaxGravity(0.27),
            ],

            Species::FonticuluaCampestris => vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::Argon),
                SpawnCondition::Any(vec![
                    SpawnCondition::PlanetClass(PlanetClass::IcyBody),
                    SpawnCondition::PlanetClass(PlanetClass::RockyIceBody),
                ]),
                SpawnCondition::MaxGravity(0.27),
            ],
            Species::FonticuluaSegmentatus => vec![
                SpawnCondition::Any(vec![
                    SpawnCondition::ThinAtmosphere(AtmosphereType::Neon),
                    SpawnCondition::ThinAtmosphere(AtmosphereType::NeonRich),
                ]),
                SpawnCondition::Any(vec![
                    SpawnCondition::PlanetClass(PlanetClass::IcyBody),
                    SpawnCondition::PlanetClass(PlanetClass::RockyIceBody),
                ]),
                SpawnCondition::MaxGravity(0.27),
            ],
            Species::FonticuluaDigitos => vec![
                SpawnCondition::Any(vec![
                    SpawnCondition::ThinAtmosphere(AtmosphereType::Methane),
                    SpawnCondition::ThinAtmosphere(AtmosphereType::MethaneRich),
                ]),
                SpawnCondition::Any(vec![
                    SpawnCondition::PlanetClass(PlanetClass::IcyBody),
                    SpawnCondition::PlanetClass(PlanetClass::RockyIceBody),
                ]),
                SpawnCondition::MaxGravity(0.27),
            ],
            Species::FonticuluaUpupam => vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::Argon),
                SpawnCondition::Any(vec![
                    SpawnCondition::PlanetClass(PlanetClass::IcyBody),
                    SpawnCondition::PlanetClass(PlanetClass::RockyIceBody),
                ]),
                SpawnCondition::MaxGravity(0.27),
            ],
            Species::FonticuluaLapida => vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::Nitrogen),
                SpawnCondition::Any(vec![
                    SpawnCondition::PlanetClass(PlanetClass::IcyBody),
                    SpawnCondition::PlanetClass(PlanetClass::RockyIceBody),
                ]),
                SpawnCondition::MaxGravity(0.27),
            ],
            Species::FonticuluaFluctus => vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::Oxygen),
                SpawnCondition::Any(vec![
                    SpawnCondition::PlanetClass(PlanetClass::IcyBody),
                    SpawnCondition::PlanetClass(PlanetClass::RockyIceBody),
                ]),
                SpawnCondition::MaxGravity(0.27),
            ],

            Species::FrutexaAcus => vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                SpawnCondition::MaxGravity(0.27),
                SpawnCondition::MaxMeanTemperature(195.0),
            ],
            Species::FrutexaCollum => vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::SulfurDioxide),
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                SpawnCondition::MaxGravity(0.27),
            ],
            Species::FrutexaFera => vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                SpawnCondition::MaxGravity(0.27),
                SpawnCondition::MaxMeanTemperature(195.0),
            ],
            Species::FrutexaFlabellum => vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::Ammonia),
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                SpawnCondition::MaxGravity(0.27),
            ],
            Species::FrutexaFlammasis => vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::Ammonia),
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                SpawnCondition::MaxGravity(0.27),
            ],
            Species::FrutexaMetallicum => vec![
                SpawnCondition::Any(vec![
                    SpawnCondition::All(vec![
                        SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
                        SpawnCondition::MaxMeanTemperature(195.0),
                    ]),
                    SpawnCondition::ThinAtmosphere(AtmosphereType::Ammonia),
                ]),
                SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
                SpawnCondition::MaxGravity(0.27),
            ],
            Species::FrutexaSponsae => vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::Water),
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                SpawnCondition::MaxGravity(0.27),
            ],

            Species::FumerolaAquatis => vec![
                SpawnCondition::AnyThinAtmosphere,
                SpawnCondition::GeologicalSignalsPresent,
                SpawnCondition::Any(vec![
                    SpawnCondition::VolcanismType(VolcanismType::WaterMagma),
                    SpawnCondition::VolcanismType(VolcanismType::WaterGeysers),
                ]),
                SpawnCondition::MaxGravity(0.27),
            ],
            Species::FumerolaCarbosis => vec![
                SpawnCondition::AnyThinAtmosphere,
                SpawnCondition::GeologicalSignalsPresent,
                SpawnCondition::Any(vec![
                    SpawnCondition::VolcanismType(VolcanismType::CarbonDioxideGeysers),
                    SpawnCondition::VolcanismType(VolcanismType::MethaneMagma),
                    SpawnCondition::VolcanismType(VolcanismType::MethaneGeysers),
                ]),
                SpawnCondition::MaxGravity(0.27),
            ],
            Species::FumerolaExtremus => vec![
                SpawnCondition::AnyThinAtmosphere,
                SpawnCondition::GeologicalSignalsPresent,
                SpawnCondition::Any(vec![
                    SpawnCondition::VolcanismType(VolcanismType::CarbonDioxideGeysers),
                    SpawnCondition::VolcanismType(VolcanismType::MethaneMagma),
                    SpawnCondition::VolcanismType(VolcanismType::MethaneGeysers),
                ]),
                SpawnCondition::MaxGravity(0.27),
            ],
            Species::FumerolaNitris => vec![
                SpawnCondition::AnyThinAtmosphere,
                SpawnCondition::GeologicalSignalsPresent,
                SpawnCondition::Any(vec![
                    SpawnCondition::VolcanismType(VolcanismType::NitrogenMagma),
                    SpawnCondition::VolcanismType(VolcanismType::NitrogenGeysers),
                    SpawnCondition::VolcanismType(VolcanismType::AmmoniaMagma),
                    SpawnCondition::VolcanismType(VolcanismType::AmmoniaGeysers),
                ]),
                SpawnCondition::MaxGravity(0.27),
            ],

            Species::FungoidaBullarum => vec![
                SpawnCondition::Any(vec![
                    SpawnCondition::ThinAtmosphere(AtmosphereType::Argon),
                    SpawnCondition::ThinAtmosphere(AtmosphereType::ArgonRich),
                ]),
                SpawnCondition::MaxGravity(0.27),
            ],
            Species::FungoidaGelata => vec![
                SpawnCondition::Any(vec![
                    SpawnCondition::ThinAtmosphere(AtmosphereType::Water),
                    SpawnCondition::All(vec![
                        SpawnCondition::ThinAtmosphere(AtmosphereType::ArgonRich),
                        SpawnCondition::MinMeanTemperature(180.0),
                        SpawnCondition::MaxMeanTemperature(195.0),
                    ]),
                ]),
                SpawnCondition::MaxGravity(0.27),
            ],
            Species::FungoidaSetisis => vec![
                SpawnCondition::Any(vec![
                    SpawnCondition::ThinAtmosphere(AtmosphereType::Ammonia),
                    SpawnCondition::ThinAtmosphere(AtmosphereType::Methane),
                ]),
                SpawnCondition::MaxGravity(0.27),
            ],
            Species::FungoidaStabitis => vec![
                SpawnCondition::Any(vec![
                    SpawnCondition::ThinAtmosphere(AtmosphereType::Water),
                    SpawnCondition::All(vec![
                        SpawnCondition::ThinAtmosphere(AtmosphereType::ArgonRich),
                        SpawnCondition::MinMeanTemperature(180.0),
                        SpawnCondition::MaxMeanTemperature(195.0),
                    ]),
                ]),
                SpawnCondition::MaxGravity(0.27),
            ],

            Species::OsseusCornibus => vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
                SpawnCondition::Any(vec![
                    SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                    SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
                ]),
                SpawnCondition::MaxGravity(0.27),
                SpawnCondition::MinMeanTemperature(180.0),
                SpawnCondition::MaxMeanTemperature(195.0),
            ],
            Species::OsseusDiscus => vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::Water),
                SpawnCondition::Any(vec![
                    SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                    SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
                ]),
                SpawnCondition::MaxGravity(0.27),
            ],
            Species::OsseusFractus => vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
                SpawnCondition::Any(vec![
                    SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                    SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
                ]),
                SpawnCondition::MaxGravity(0.27),
                SpawnCondition::MinMeanTemperature(180.0),
                SpawnCondition::MaxMeanTemperature(195.0),
            ],
            Species::OsseusPellebantus => vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
                SpawnCondition::Any(vec![
                    SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                    SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
                ]),
                SpawnCondition::MaxGravity(0.27),
                SpawnCondition::MinMeanTemperature(180.0),
                SpawnCondition::MaxMeanTemperature(195.0),
            ],
            Species::OsseusPumice => vec![
                SpawnCondition::Any(vec![
                    SpawnCondition::ThinAtmosphere(AtmosphereType::Argon),
                    SpawnCondition::ThinAtmosphere(AtmosphereType::Methane),
                    SpawnCondition::ThinAtmosphere(AtmosphereType::Nitrogen),
                ]),
                SpawnCondition::Any(vec![
                    SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                    SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
                    SpawnCondition::PlanetClass(PlanetClass::RockyIceBody),
                ]),
                SpawnCondition::MaxGravity(0.27),
            ],
            Species::OsseusSpiralis => vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::Ammonia),
                SpawnCondition::Any(vec![
                    SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                    SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
                ]),
                SpawnCondition::MaxGravity(0.27),
            ],

            Species::ReceptaConditivus => vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::SulfurDioxide),
                SpawnCondition::Any(vec![
                    SpawnCondition::PlanetClass(PlanetClass::IcyBody),
                    SpawnCondition::PlanetClass(PlanetClass::RockyIceBody),
                ]),
                SpawnCondition::MaxGravity(0.27),
            ],
            Species::ReceptaDeltahedronix => vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::SulfurDioxide),
                SpawnCondition::Any(vec![
                    SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                    SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
                ]),
                SpawnCondition::MaxGravity(0.27),
            ],
            Species::ReceptaUmbrux => vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::SulfurDioxide),
                SpawnCondition::MaxGravity(0.27),
            ],

            Species::SinuousTubersAlbidum => vec![
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
            ],
            Species::SinuousTubersBlatteum => vec![
                SpawnCondition::Any(vec![
                    SpawnCondition::PlanetClass(PlanetClass::MetalRichBody),
                    SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
                ]),
            ],
            Species::SinuousTubersCaeruleum => vec![
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
            ],
            Species::SinuousTubersLindigoticum => vec![
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
            ],
            Species::SinuousTubersPrasinum => vec![
                SpawnCondition::Any(vec![
                    SpawnCondition::PlanetClass(PlanetClass::MetalRichBody),
                    SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
                ]),
            ],
            Species::SinuousTubersRoseum => vec![
                SpawnCondition::VolcanismType(VolcanismType::SilicateMagma),
            ],
            Species::SinuousTubersViolaceum => vec![
                SpawnCondition::Any(vec![
                    SpawnCondition::PlanetClass(PlanetClass::MetalRichBody),
                    SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
                ]),
            ],
            Species::SinuousTubersViride => vec![
                SpawnCondition::Any(vec![
                    SpawnCondition::PlanetClass(PlanetClass::MetalRichBody),
                    SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
                ]),
            ],

            Species::StratumAraneamus => vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::SulfurDioxide),
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                SpawnCondition::MinMeanTemperature(165.0),
            ],
            Species::StratumCucumisis => vec![
                SpawnCondition::Any(vec![
                    SpawnCondition::ThinAtmosphere(AtmosphereType::SulfurDioxide),
                    SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
                ]),
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                SpawnCondition::MinMeanTemperature(190.0),
            ],
            Species::StratumExcutitus => vec![
                SpawnCondition::Any(vec![
                    SpawnCondition::ThinAtmosphere(AtmosphereType::SulfurDioxide),
                    SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
                ]),
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                SpawnCondition::MinMeanTemperature(165.0),
                SpawnCondition::MaxMeanTemperature(190.0),
            ],
            Species::StratumFrigus => vec![
                SpawnCondition::Any(vec![
                    SpawnCondition::ThinAtmosphere(AtmosphereType::SulfurDioxide),
                    SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
                ]),
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                SpawnCondition::MinMeanTemperature(190.0),
            ],
            Species::StratumLaminamus => vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::Ammonia),
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                SpawnCondition::MinMeanTemperature(165.0),
            ],
            Species::StratumLimaxus => vec![
                SpawnCondition::Any(vec![
                    SpawnCondition::ThinAtmosphere(AtmosphereType::SulfurDioxide),
                    SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
                ]),
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                SpawnCondition::MinMeanTemperature(165.0),
                SpawnCondition::MaxMeanTemperature(190.0),
            ],
            Species::StratumPaleas => vec![
                SpawnCondition::Any(vec![
                    SpawnCondition::ThinAtmosphere(AtmosphereType::Ammonia),
                    SpawnCondition::ThinAtmosphere(AtmosphereType::Water),
                    SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
                ]),
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                SpawnCondition::MinMeanTemperature(165.0),
            ],
            Species::StratumTectonicas => vec![
                SpawnCondition::AnyThinAtmosphere,
                SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
                SpawnCondition::MinMeanTemperature(165.0),
            ],

            Species::TubusCavas => vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                SpawnCondition::MaxGravity(0.15),
                SpawnCondition::MinMeanTemperature(160.0),
                SpawnCondition::MaxMeanTemperature(190.0),
            ],
            Species::TubusCompagibus => vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                SpawnCondition::MaxGravity(0.15),
                SpawnCondition::MinMeanTemperature(160.0),
                SpawnCondition::MaxMeanTemperature(190.0),
            ],
            Species::TubusConifer => vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                SpawnCondition::MaxGravity(0.15),
                SpawnCondition::MinMeanTemperature(160.0),
                SpawnCondition::MaxMeanTemperature(190.0),
            ],
            Species::TubusRosarium => vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::Ammonia),
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                SpawnCondition::MaxGravity(0.15),
                SpawnCondition::MinMeanTemperature(160.0),
            ],
            Species::TubusSororibus => vec![
                SpawnCondition::Any(vec![
                    SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
                    SpawnCondition::ThinAtmosphere(AtmosphereType::Ammonia),
                ]),
                SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
                SpawnCondition::MaxGravity(0.15),
                SpawnCondition::MinMeanTemperature(160.0),
                SpawnCondition::MaxMeanTemperature(190.0),
            ],

            Species::TussockAlbata => vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                SpawnCondition::MaxGravity(0.27),
                SpawnCondition::MinMeanTemperature(175.0),
                SpawnCondition::MaxMeanTemperature(180.0),
            ],
            Species::TussockCapillum => vec![
                SpawnCondition::Any(vec![
                    SpawnCondition::ThinAtmosphere(AtmosphereType::Argon),
                    SpawnCondition::ThinAtmosphere(AtmosphereType::Methane),
                ]),
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                SpawnCondition::MaxGravity(0.27),
            ],
            Species::TussockCaputus => vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                SpawnCondition::MaxGravity(0.27),
                SpawnCondition::MinMeanTemperature(180.0),
                SpawnCondition::MaxMeanTemperature(190.0),
            ],
            Species::TussockCatena => vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::Ammonia),
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                SpawnCondition::MaxGravity(0.27),
            ],
            Species::TussockCultro => vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::Ammonia),
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                SpawnCondition::MaxGravity(0.27),
            ],
            Species::TussockDivisa => vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::Ammonia),
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                SpawnCondition::MaxGravity(0.27),
            ],
            Species::TussockIgnis => vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                SpawnCondition::MaxGravity(0.27),
                SpawnCondition::MinMeanTemperature(160.0),
                SpawnCondition::MaxMeanTemperature(170.0),
            ],
            Species::TussockPennata => vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                SpawnCondition::MaxGravity(0.27),
                SpawnCondition::MinMeanTemperature(145.0),
                SpawnCondition::MaxMeanTemperature(155.0),
            ],
            Species::TussockPennatis => vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                SpawnCondition::MaxGravity(0.27),
                SpawnCondition::MaxMeanTemperature(195.0),
            ],
            Species::TussockPropagito => vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                SpawnCondition::MaxGravity(0.27),
                SpawnCondition::MaxMeanTemperature(195.0),
            ],
            Species::TussockSerrati => vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                SpawnCondition::MaxGravity(0.27),
                SpawnCondition::MinMeanTemperature(170.0),
                SpawnCondition::MaxMeanTemperature(175.0),
            ],
            Species::TussockStigmasis => vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::SulfurDioxide),
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                SpawnCondition::MaxGravity(0.27),
            ],
            Species::TussockTriticum => vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                SpawnCondition::MaxGravity(0.27),
                SpawnCondition::MinMeanTemperature(190.0),
                SpawnCondition::MaxMeanTemperature(195.0),
            ],
            Species::TussockVentusa => vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::CarbonDioxide),
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                SpawnCondition::MaxGravity(0.27),
                SpawnCondition::MinMeanTemperature(155.0),
                SpawnCondition::MaxMeanTemperature(160.0),
            ],
            Species::TussockVirgam => vec![
                SpawnCondition::ThinAtmosphere(AtmosphereType::Water),
                SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                SpawnCondition::MaxGravity(0.27),
            ],
        }
    }
}
