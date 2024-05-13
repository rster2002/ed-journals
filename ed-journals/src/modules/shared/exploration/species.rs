use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde::{Serialize, Deserialize};
use serde_json::Value;
use crate::modules::exobiology::models::spawn_condition::SpawnCondition;
use crate::shared::galaxy::atmosphere::{Atmosphere, AtmosphereDensity};
use crate::shared::galaxy::atmosphere_type::AtmosphereType;
use crate::shared::galaxy::planet_class::PlanetClass;
use crate::shared::galaxy::star_class::StarClass;
use crate::shared::galaxy::star_luminosity::StarLuminosity;
use crate::shared::galaxy::volcanism_type::VolcanismType;

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

                Species::CrystallineShards => "Crystaline Shards"

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
                SpawnCondition::Atmosphere(Atmosphere {
                    hot: false,
                    density: AtmosphereDensity::Thin,
                    kind: AtmosphereType::CarbonDioxide,
                }),
                SpawnCondition::MaxGravity(0.27),
                SpawnCondition::MinSurfaceTemperature(175.0),
                SpawnCondition::MaxSurfaceTemperature(180.0),
            ],
            Species::AleoidaCoronamus => vec![
                SpawnCondition::Atmosphere(Atmosphere {
                    hot: false,
                    density: AtmosphereDensity::Thin,
                    kind: AtmosphereType::CarbonDioxide,
                }),
                SpawnCondition::MaxGravity(0.27),
                SpawnCondition::MinSurfaceTemperature(180.0),
                SpawnCondition::MaxSurfaceTemperature(190.0),
            ],
            Species::AleoidaGravis => vec![
                SpawnCondition::Atmosphere(Atmosphere {
                    hot: false,
                    density: AtmosphereDensity::Thin,
                    kind: AtmosphereType::CarbonDioxide,
                }),
                SpawnCondition::MaxGravity(0.27),
                SpawnCondition::MinSurfaceTemperature(190.0),
                SpawnCondition::MaxSurfaceTemperature(195.0),
            ],
            Species::AleoidaLaminiae => vec![
                SpawnCondition::Atmosphere(Atmosphere {
                    hot: false,
                    density: AtmosphereDensity::Thin,
                    kind: AtmosphereType::Ammonia,
                }),
                SpawnCondition::MaxGravity(0.27),
            ],
            Species::AleoidaSpica => vec![
                SpawnCondition::Atmosphere(Atmosphere {
                    hot: false,
                    density: AtmosphereDensity::Thin,
                    kind: AtmosphereType::Ammonia,
                }),
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
                SpawnCondition::Atmosphere(Atmosphere {
                    hot: false,
                    density: AtmosphereDensity::Thin,
                    kind: AtmosphereType::Helium,
                }),
            ],
            Species::BacteriumAcies => vec![
                SpawnCondition::Any(vec![
                    SpawnCondition::Atmosphere(Atmosphere {
                        hot: false,
                        density: AtmosphereDensity::Thin,
                        kind: AtmosphereType::Neon,
                    }),
                    SpawnCondition::Atmosphere(Atmosphere {
                        hot: false,
                        density: AtmosphereDensity::Thin,
                        kind: AtmosphereType::NeonRich,
                    }),
                ]),
            ],
            Species::BacteriumOmentum => vec![
                SpawnCondition::Any(vec![
                    SpawnCondition::Atmosphere(Atmosphere {
                        hot: false,
                        density: AtmosphereDensity::Thin,
                        kind: AtmosphereType::Neon,
                    }),
                    SpawnCondition::Atmosphere(Atmosphere {
                        hot: false,
                        density: AtmosphereDensity::Thin,
                        kind: AtmosphereType::NeonRich,
                    }),
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
                    SpawnCondition::Atmosphere(Atmosphere {
                        hot: false,
                        density: AtmosphereDensity::Thin,
                        kind: AtmosphereType::Neon,
                    }),
                    SpawnCondition::Atmosphere(Atmosphere {
                        hot: false,
                        density: AtmosphereDensity::Thin,
                        kind: AtmosphereType::NeonRich,
                    }),
                ]),
                SpawnCondition::Any(vec![
                    SpawnCondition::VolcanismType(VolcanismType::CarbonDioxideGeysers),
                    SpawnCondition::VolcanismType(VolcanismType::MethaneGeysers),
                    SpawnCondition::VolcanismType(VolcanismType::MethaneMagma),
                ]),
            ],
            Species::BacteriumVerrata => vec![
                SpawnCondition::Any(vec![
                    SpawnCondition::Atmosphere(Atmosphere {
                        hot: false,
                        density: AtmosphereDensity::Thin,
                        kind: AtmosphereType::Neon,
                    }),
                    SpawnCondition::Atmosphere(Atmosphere {
                        hot: false,
                        density: AtmosphereDensity::Thin,
                        kind: AtmosphereType::NeonRich,
                    }),
                ]),
                SpawnCondition::Any(vec![
                    SpawnCondition::VolcanismType(VolcanismType::WaterMagma),
                    SpawnCondition::VolcanismType(VolcanismType::WaterGeysers),
                ]),
            ],
            Species::BacteriumBullaris => vec![
                SpawnCondition::Any(vec![
                    SpawnCondition::Atmosphere(Atmosphere {
                        hot: false,
                        density: AtmosphereDensity::Thin,
                        kind: AtmosphereType::Methane,
                    }),
                    SpawnCondition::Atmosphere(Atmosphere {
                        hot: false,
                        density: AtmosphereDensity::Thin,
                        kind: AtmosphereType::MethaneRich,
                    }),
                ]),
            ],
            Species::BacteriumAlcyoneum => vec![
                SpawnCondition::Atmosphere(Atmosphere {
                    hot: false,
                    density: AtmosphereDensity::Thin,
                    kind: AtmosphereType::Ammonia,
                }),
            ],
            Species::BacteriumVesicula => vec![
                SpawnCondition::Any(vec![
                    SpawnCondition::Atmosphere(Atmosphere {
                        hot: false,
                        density: AtmosphereDensity::Thin,
                        kind: AtmosphereType::Argon,
                    }),
                    SpawnCondition::Atmosphere(Atmosphere {
                        hot: false,
                        density: AtmosphereDensity::Thin,
                        kind: AtmosphereType::ArgonRich,
                    }),
                ]),
            ],
            Species::BacteriumCerbrus => vec![
                SpawnCondition::Atmosphere(Atmosphere {
                    hot: false,
                    density: AtmosphereDensity::Thin,
                    kind: AtmosphereType::SulfurDioxide,
                }),
            ],
            Species::BacteriumAurasus => vec![
                SpawnCondition::Any(vec![
                    SpawnCondition::Atmosphere(Atmosphere {
                        hot: false,
                        density: AtmosphereDensity::Thin,
                        kind: AtmosphereType::CarbonDioxide,
                    }),
                    SpawnCondition::Atmosphere(Atmosphere {
                        hot: false,
                        density: AtmosphereDensity::Thin,
                        kind: AtmosphereType::CarbonDioxideRich,
                    }),
                ]),
            ],
            Species::BacteriumInformem => vec![
                SpawnCondition::Atmosphere(Atmosphere {
                    hot: false,
                    density: AtmosphereDensity::Thin,
                    kind: AtmosphereType::Nitrogen,
                }),
            ],
            Species::BacteriumVolu => vec![
                SpawnCondition::Atmosphere(Atmosphere {
                    hot: false,
                    density: AtmosphereDensity::Thin,
                    kind: AtmosphereType::Oxygen,
                }),
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
                SpawnCondition::Atmosphere(Atmosphere {
                    hot: false,
                    density: AtmosphereDensity::Thin,
                    kind: AtmosphereType::Ammonia,
                }),
            ],
            Species::CactoidaPullulanta => vec![
                SpawnCondition::MaxGravity(0.27),
                SpawnCondition::Any(vec![
                    SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                    SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
                ]),
                SpawnCondition::Any(vec![
                    SpawnCondition::Atmosphere(Atmosphere {
                        hot: false,
                        density: AtmosphereDensity::Thin,
                        kind: AtmosphereType::CarbonDioxide,
                    }),
                    SpawnCondition::Atmosphere(Atmosphere {
                        hot: false,
                        density: AtmosphereDensity::Thin,
                        kind: AtmosphereType::CarbonDioxideRich,
                    }),
                ]),
                SpawnCondition::MinSurfaceTemperature(180.0),
                SpawnCondition::MaxSurfaceTemperature(195.0),
            ],
            Species::CactoidaCortexum => vec![
                SpawnCondition::MaxGravity(0.27),
                SpawnCondition::Any(vec![
                    SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                    SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
                ]),
                SpawnCondition::Any(vec![
                    SpawnCondition::Atmosphere(Atmosphere {
                        hot: false,
                        density: AtmosphereDensity::Thin,
                        kind: AtmosphereType::CarbonDioxide,
                    }),
                    SpawnCondition::Atmosphere(Atmosphere {
                        hot: false,
                        density: AtmosphereDensity::Thin,
                        kind: AtmosphereType::CarbonDioxideRich,
                    }),
                ]),
                SpawnCondition::MinSurfaceTemperature(180.0),
                SpawnCondition::MaxSurfaceTemperature(195.0),
            ],
            Species::CactoidaVermis => vec![
                SpawnCondition::MaxGravity(0.27),
                SpawnCondition::Any(vec![
                    SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                    SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
                ]),
                SpawnCondition::Atmosphere(Atmosphere {
                    hot: false,
                    density: AtmosphereDensity::Thin,
                    kind: AtmosphereType::Water,
                }),
            ],
            Species::CactoidaPeperatis => vec![
                SpawnCondition::MaxGravity(0.27),
                SpawnCondition::Any(vec![
                    SpawnCondition::PlanetClass(PlanetClass::RockyBody),
                    SpawnCondition::PlanetClass(PlanetClass::HighMetalContentBody),
                ]),
                SpawnCondition::Atmosphere(Atmosphere {
                    hot: false,
                    density: AtmosphereDensity::Thin,
                    kind: AtmosphereType::Ammonia,
                }),
            ],

            Species::ClypeusSpeculumi => vec![
                SpawnCondition::MaxGravity(0.27),
                SpawnCondition::MinSurfaceTemperature(190.0),
                SpawnCondition::Any(vec![
                    SpawnCondition::Atmosphere(Atmosphere {
                        hot: false,
                        density: AtmosphereDensity::Thin,
                        kind: AtmosphereType::CarbonDioxide,
                    }),
                    SpawnCondition::Atmosphere(Atmosphere {
                        hot: false,
                        density: AtmosphereDensity::Thin,
                        kind: AtmosphereType::Water,
                    }),
                ]),
                SpawnCondition::MinDistanceFromParentSun(5.0),
            ],
            Species::ClypeusLacrimam => vec![
                SpawnCondition::MaxGravity(0.27),
                SpawnCondition::MinSurfaceTemperature(190.0),
                SpawnCondition::Any(vec![
                    SpawnCondition::Atmosphere(Atmosphere {
                        hot: false,
                        density: AtmosphereDensity::Thin,
                        kind: AtmosphereType::CarbonDioxide,
                    }),
                    SpawnCondition::Atmosphere(Atmosphere {
                        hot: false,
                        density: AtmosphereDensity::Thin,
                        kind: AtmosphereType::Water,
                    }),
                ]),
            ],
            Species::ClypeusMargaritus => vec![
                SpawnCondition::MaxGravity(0.27),
                SpawnCondition::MinSurfaceTemperature(190.0),
                SpawnCondition::Any(vec![
                    SpawnCondition::Atmosphere(Atmosphere {
                        hot: false,
                        density: AtmosphereDensity::Thin,
                        kind: AtmosphereType::CarbonDioxide,
                    }),
                    SpawnCondition::Atmosphere(Atmosphere {
                        hot: false,
                        density: AtmosphereDensity::Thin,
                        kind: AtmosphereType::Water,
                    }),
                ]),
            ],

            Species::ConchaRenibus => vec![
                SpawnCondition::Any(vec![
                    SpawnCondition::All(vec![
                        SpawnCondition::MaxGravity(0.27),
                        SpawnCondition::Any(vec![
                            SpawnCondition::Atmosphere(Atmosphere {
                                hot: false,
                                density: AtmosphereDensity::Thin,
                                kind: AtmosphereType::CarbonDioxide,
                            }),
                            SpawnCondition::Atmosphere(Atmosphere {
                                hot: false,
                                density: AtmosphereDensity::Thin,
                                kind: AtmosphereType::CarbonDioxideRich,
                            }),
                        ]),
                        SpawnCondition::MinSurfaceTemperature(180.0),
                        SpawnCondition::MaxSurfaceTemperature(195.0),
                    ]),
                    SpawnCondition::All(vec![
                        SpawnCondition::MaxGravity(0.27),
                        SpawnCondition::Any(vec![
                            SpawnCondition::Atmosphere(Atmosphere {
                                hot: false,
                                density: AtmosphereDensity::Thin,
                                kind: AtmosphereType::Water,
                            }),
                            SpawnCondition::Atmosphere(Atmosphere {
                                hot: false,
                                density: AtmosphereDensity::Thin,
                                kind: AtmosphereType::WaterRich,
                            }),
                        ]),
                    ]),
                ]),
            ],
            Species::ConchaAureolas => vec![
                SpawnCondition::MaxGravity(0.27),
                SpawnCondition::Atmosphere(Atmosphere {
                    hot: false,
                    density: AtmosphereDensity::Thin,
                    kind: AtmosphereType::Ammonia,
                }),
            ],
            Species::ConchaLabiata => vec![
                SpawnCondition::MaxGravity(0.27),
                SpawnCondition::Any(vec![
                    SpawnCondition::Atmosphere(Atmosphere {
                        hot: false,
                        density: AtmosphereDensity::Thin,
                        kind: AtmosphereType::CarbonDioxide,
                    }),
                    SpawnCondition::Atmosphere(Atmosphere {
                        hot: false,
                        density: AtmosphereDensity::Thin,
                        kind: AtmosphereType::CarbonDioxideRich,
                    }),
                ]),
                SpawnCondition::MaxSurfaceTemperature(190.0),
            ],
            Species::ConchaBiconcavis => vec![
                SpawnCondition::MaxGravity(0.27),
                SpawnCondition::Atmosphere(Atmosphere {
                    hot: false,
                    density: AtmosphereDensity::Thin,
                    kind: AtmosphereType::Nitrogen,
                }),
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
                SpawnCondition::MinSurfaceTemperature(0.0),
                SpawnCondition::MaxSurfaceTemperature(273.0),
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
                    SpawnCondition::Atmosphere(Atmosphere {
                        hot: false,
                        density: AtmosphereDensity::Thin,
                        kind: AtmosphereType::Helium,
                    }),
                    SpawnCondition::Atmosphere(Atmosphere {
                        hot: false,
                        density: AtmosphereDensity::Thin,
                        kind: AtmosphereType::Neon,
                    }),
                    SpawnCondition::Atmosphere(Atmosphere {
                        hot: false,
                        density: AtmosphereDensity::Thin,
                        kind: AtmosphereType::Argon,
                    }),
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
                    SpawnCondition::Atmosphere(Atmosphere {
                        hot: false,
                        density: AtmosphereDensity::Thin,
                        kind: AtmosphereType::Helium,
                    }),
                    SpawnCondition::Atmosphere(Atmosphere {
                        hot: false,
                        density: AtmosphereDensity::Thin,
                        kind: AtmosphereType::Neon,
                    }),
                    SpawnCondition::Atmosphere(Atmosphere {
                        hot: false,
                        density: AtmosphereDensity::Thin,
                        kind: AtmosphereType::Argon,
                    }),
                ]),
                SpawnCondition::PlanetClass(PlanetClass::IcyBody),
                SpawnCondition::WithinNebulaRange(150.0),
                SpawnCondition::MaxGravity(0.27),
            ],

            Species::FonticuluaCampestris => vec![],
            Species::FonticuluaSegmentatus => vec![],
            Species::FonticuluaDigitos => vec![],
            Species::FonticuluaUpupam => vec![],
            Species::FonticuluaLapida => vec![],
            Species::FonticuluaFluctus => vec![],

            Species::FrutexaFlabellum => vec![],
            Species::FrutexaFlammasis => vec![],
            Species::FrutexaMetallicum => vec![],
            Species::FrutexaAcus => vec![],
            Species::FrutexaFera => vec![],
            Species::FrutexaSponsae => vec![],
            Species::FrutexaCollum => vec![],

            Species::FumerolaAquatis => vec![],
            Species::FumerolaCarbosis => vec![],
            Species::FumerolaExtremus => vec![],
            Species::FumerolaNitris => vec![],

            Species::FungoidaSetisis => vec![],
            Species::FungoidaStabitis => vec![],
            Species::FungoidaBullarum => vec![],
            Species::FungoidaGelata => vec![],

            Species::OsseusFractus => vec![],
            Species::OsseusSpiralis => vec![],
            Species::OsseusPumice => vec![],
            Species::OsseusCornibus => vec![],
            Species::OsseusPellebantus => vec![],
            Species::OsseusDiscus => vec![],

            Species::ReceptaUmbrux => vec![],
            Species::ReceptaDeltahedronix => vec![],
            Species::ReceptaConditivus => vec![],

            Species::SinuousTubersAlbidum => vec![],
            Species::SinuousTubersBlatteum => vec![],
            Species::SinuousTubersCaeruleum => vec![],
            Species::SinuousTubersLindigoticum => vec![],
            Species::SinuousTubersPrasinum => vec![],
            Species::SinuousTubersRoseum => vec![],
            Species::SinuousTubersViolaceum => vec![],
            Species::SinuousTubersViride => vec![],

            Species::StratumTectonicas => vec![],
            Species::StratumPaleas => vec![],
            Species::StratumFrigus => vec![],
            Species::StratumLaminamus => vec![],
            Species::StratumExcutitus => vec![],
            Species::StratumLimaxus => vec![],
            Species::StratumCucumisis => vec![],
            Species::StratumAraneamus => vec![],

            Species::TubusConifer => vec![],
            Species::TubusSororibus => vec![],
            Species::TubusRosarium => vec![],
            Species::TubusCavas => vec![],
            Species::TubusCompagibus => vec![],

            Species::TussockPennata => vec![],
            Species::TussockVentusa => vec![],
            Species::TussockIgnis => vec![],
            Species::TussockCultro => vec![],
            Species::TussockSerrati => vec![],
            Species::TussockAlbata => vec![],
            Species::TussockDivisa => vec![],
            Species::TussockCaputus => vec![],
            Species::TussockTriticum => vec![],
            Species::TussockStigmasis => vec![],
            Species::TussockCapillum => vec![],
            Species::TussockCatena => vec![],
            Species::TussockPropagito => vec![],
            Species::TussockPennatis => vec![],
            Species::TussockVirgam => vec![],
        }
    }
}
