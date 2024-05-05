use std::str::FromStr;

use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize, Clone, PartialEq)]
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
