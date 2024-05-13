use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde::{Serialize, Deserialize};
use serde_json::Value;

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
    AnemonePrasinusBioluminescent,
    AnemonePrasinumBioluminescent,
    AnemonePuniceus,
    AnemonePuniceum,
    AnemoneRoseusBioluminescent,
    AnemoneRoseumBioluminescent,
    AnemoneRoseus,
    AnemoneRoseum,
    AnemoneBlattinusBioluminescent,
    AnemoneBlatteumBioluminescent,
    AnemoneLuteus,
    AnemoneLuteolum,
    AnemoneRubensBioluminescent,
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
                Species::AleoidaLaminiae => "Aleoida Laminiae",
                Species::AleoidaGravis => "Aleoida Gravis",
                Species::AleoidaSpica => "Aleoida Spica",
                Species::AleoidaCoronamus => "Aleoida Coronamus",
                Species::AleoidaArcus => "Aleoida Arcus",
                Species::AmphoraPlant => "Amphora Plant",

                Species::AnemonePrasinusBioluminescent => "Anemone Prasinus Bioluminescent",
                Species::AnemonePrasinumBioluminescent => "Anemone Prasinum Bioluminescent",
                Species::AnemonePuniceus => "Anemone Puniceus",
                Species::AnemonePuniceum => "Anemone Puniceum",
                Species::AnemoneRoseusBioluminescent => "Anemone Roseus Bioluminescent",
                Species::AnemoneRoseumBioluminescent => "Anemone Roseum Bioluminescent",
                Species::AnemoneRoseus => "Anemone Roseus",
                Species::AnemoneRoseum => "Anemone Roseum",
                Species::AnemoneBlattinusBioluminescent => "Anemone Blattinus Bioluminescent",
                Species::AnemoneBlatteumBioluminescent => "Anemone Blatteum Bioluminescent",
                Species::AnemoneLuteus => "Anemone Luteus",
                Species::AnemoneLuteolum => "Anemone Luteolum",
                Species::AnemoneRubensBioluminescent => "Anemone Rubens Bioluminescent",
                Species::AnemoneRubeumBioluminescent => "Anemone Rubeum Bioluminescent",
                Species::AnemoneCroceus => "Anemone Croceus",
                Species::AnemoneCroceum => "Anemone Croceum",

                Species::BarkMound => "Bark Mound",

                Species::BacteriumNebulus => "Bacterium Nebulus",
                Species::BacteriumAcies => "Bacterium Acies",
                Species::BacteriumOmentum => "Bacterium Omentum",
                Species::BacteriumScopulum => "Bacterium Scopulum",
                Species::BacteriumVerrata => "Bacterium Verrata",
                Species::BacteriumBullaris => "Bacterium Bullaris",
                Species::BacteriumAlcyoneum => "Bacterium Alcyoneum",
                Species::BacteriumVesicula => "Bacterium Vesicula",
                Species::BacteriumCerbrus => "Bacterium Cerbrus",
                Species::BacteriumAurasus => "Bacterium Aurasus",
                Species::BacteriumInformem => "Bacterium Informem",
                Species::BacteriumVolu => "Bacterium Volu",
                Species::BacteriumTela => "Bacterium Tela",

                Species::BrainTreeAureum => "Brain Tree Aureum",
                Species::BrainTreeOstrinum => "Brain Tree Ostrinum",
                Species::BrainTreePuniceum => "Brain Tree Puniceum",
                Species::BrainTreeLindigoticum => "Brain Tree Lindigoticum",
                Species::BrainTreeGypseeum => "Brain Tree Gypseeum",
                Species::BrainTreeLividum => "Brain Tree Lividum",
                Species::BrainTreeViride => "Brain Tree Viride",
                Species::BrainTreeRoseum => "Brain Tree Roseum",

                Species::CactoidaLapis => "Cactoida Lapis",
                Species::CactoidaPullulanta => "Cactoida Pullulanta",
                Species::CactoidaCortexum => "Cactoida Cortexum",
                Species::CactoidaVermis => "Cactoida Vermis",

                Species::ClypeusSpeculumi => "Clypeus Speculumi",
                Species::ClypeusLacrimam => "Clypeus Lacrimam",
                Species::ClypeusMargaritus => "Clypeus Margaritus",

                Species::ConchaRenibus => "Concha Renibus",
                Species::ConchaAureolas => "Concha Aureolas",
                Species::ConchaLabiata => "Concha Labiata",
                Species::ConchaBiconcavis => "Concha Biconcavis",

                Species::ElectricaePluma => "Electricae Pluma",
                Species::ElectricaeRadialem => "Electricae Radialem",

                Species::FonticuluaCampestris => "Fonticulua Campestris",
                Species::FonticuluaSegmentatus => "Fonticulua Segmentatus",
                Species::FonticuluaDigitos => "Fonticulua Digitos",
                Species::FonticuluaUpupam => "Fonticulua Upupam",
                Species::FonticuluaLapida => "Fonticulua Lapida",
                Species::FonticuluaFluctus => "Fonticulua Fluctus",

                Species::FrutexaFlabellum => "Frutexa Flabellum",
                Species::FrutexaFlammasis => "Frutexa Flammasis",
                Species::FrutexaMetallicum => "Frutexa Metallicum",
                Species::FrutexaAcus => "Frutexa Acus",
                Species::FrutexaFera => "Frutexa Fera",
                Species::FrutexaSponsae => "Frutexa Sponsae",
                Species::FrutexaCollum => "Frutexa Collum",

                Species::FumerolaAquatis => "Fumerola Aquatis",
                Species::FumerolaCarbosis => "Fumerola Carbosis",
                Species::FumerolaExtremus => "Fumerola Extremus",
                Species::FumerolaNitris => "Fumerola Nitris",

                Species::FungoidaSetisis => "Fungoida Setisis",
                Species::FungoidaStabitis => "Fungoida Stabitis",
                Species::FungoidaBullarum => "Fungoida Bullarum",
                Species::FungoidaGelata => "Fungoida Gelata",

                Species::OsseusFractus => "Osseus Fractus",
                Species::OsseusSpiralis => "Osseus Spiralis",
                Species::OsseusPumice => "Osseus Pumice",
                Species::OsseusCornibus => "Osseus Cornibus",
                Species::OsseusPellebantus => "Osseus Pellebantus",
                Species::OsseusDiscus => "Osseus Discus",

                Species::ReceptaUmbrux => "Recepta Umbrux",
                Species::ReceptaDeltahedronix => "Recepta Deltahedronix",
                Species::ReceptaConditivus => "Recepta Conditivus",

                Species::SinuousTubersAlbidum => "Sinuous Tubers Albidum",
                Species::SinuousTubersBlatteum => "Sinuous Tubers Blatteum",
                Species::SinuousTubersCaeruleum => "Sinuous Tubers Caeruleum",
                Species::SinuousTubersLindigoticum => "Sinuous Tubers Lindigoticum",
                Species::SinuousTubersPrasinum => "Sinuous Tubers Prasinum",
                Species::SinuousTubersRoseum => "Sinuous Tubers Roseum",
                Species::SinuousTubersViolaceum => "Sinuous Tubers Violaceum",
                Species::SinuousTubersViride => "Sinuous Tubers Viride",

                Species::StratumTectonicas => "Stratum Tectonicas",
                Species::StratumPaleas => "Stratum Paleas",
                Species::StratumFrigus => "Stratum Frigus",
                Species::StratumLaminamus => "Stratum Laminamus",
                Species::StratumExcutitus => "Stratum Excutitus",
                Species::StratumLimaxus => "Stratum Limaxus",
                Species::StratumCucumisis => "Stratum Cucumisis",
                Species::StratumAraneamus => "Stratum Araneamus",

                Species::TubusConifer => "Tubus Conifer",
                Species::TubusSororibus => "Tubus Sororibus",
                Species::TubusRosarium => "Tubus Rosarium",
                Species::TubusCavas => "Tubus Cavas",
                Species::TubusCompagibus => "Tubus Compagibus",

                Species::TussockPennata => "Tussock Pennata",
                Species::TussockVentusa => "Tussock Ventusa",
                Species::TussockIgnis => "Tussock Ignis",
                Species::TussockCultro => "Tussock Cultro",
                Species::TussockSerrati => "Tussock Serrati",
                Species::TussockAlbata => "Tussock Albata",
                Species::TussockDivisa => "Tussock Divisa",
                Species::TussockCaputus => "Tussock Caputus",
                Species::TussockTriticum => "Tussock Triticum",
                Species::TussockStigmasis => "Tussock Stigmasis",
                Species::TussockCapillum => "Tussock Capillum",
                Species::TussockCatena => "Tussock Catena",
                Species::TussockPropagito => "Tussock Propagito",
                Species::TussockPennatis => "Tussock Pennatis",
                Species::TussockVirgam => "Tussock Virgam",
            }
        )
    }
}
