use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Species {
    // Tussock
    #[serde(rename = "$Codex_Ent_Tussocks_10_Name;")]
    TussockDivisa,

    #[serde(rename = "$Codex_Ent_Tussocks_12_Name;")]
    TussockTriticum,

    #[serde(rename = "$Codex_Ent_Tussocks_03_Name;")]
    TussockIgnis,

    #[serde(rename = "$Codex_Ent_Tussocks_11_Name;")]
    TussockCaputus,

    #[serde(rename = "$Codex_Ent_Tussocks_08_Name;")]
    TussockAlbata,

    #[serde(rename = "$Codex_Ent_Tussocks_04_Name;")]
    TussockCultro,

    #[serde(rename = "$Codex_Ent_Tussocks_07_Name;")]
    TussockSerrati,

    // Clypeus
    #[serde(rename = "$Codex_Ent_Clypeus_03_Name;")]
    ClypeusSpeculumi,

    // Stratum
    #[serde(rename = "$Codex_Ent_Stratum_07_Name;")]
    StratumTectonicas,

    #[serde(rename = "$Codex_Ent_Stratum_02_Name;")]
    StratumPaleas,

    #[serde(rename = "$Codex_Ent_Stratum_08_Name;")]
    StratumFrigus,

    // Cactoid
    #[serde(rename = "$Codex_Ent_Cactoid_02_Name;")]
    CactoidLapis,

    #[serde(rename = "$Codex_Ent_Cactoid_04_Name;")]
    CactoidPullulanta,

    // Osseus
    #[serde(rename = "$Codex_Ent_Osseus_01_Name;")]
    OsseusFractus,

    #[serde(rename = "$Codex_Ent_Osseus_03_Name;")]
    OsseusSpiralis,

    #[serde(rename = "$Codex_Ent_Osseus_05_Name;")]
    OsseusCornibus,

    // Recepta
    #[serde(rename = "$Codex_Ent_Recepta_03_Name;")]
    ReceptraConditivus,

    // Aleoids
    #[serde(rename = "$Codex_Ent_Aleoids_03_Name;")]
    AleoidaSpica,

    #[serde(rename = "$Codex_Ent_Aleoids_02_Name;")]
    AleoidaCoronamus,

    #[serde(rename = "$Codex_Ent_Aleoids_01_Name;")]
    AleoidaArcus,

    // Bacterium
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

    // Fungoida
    #[serde(rename = "$Codex_Ent_Fungoids_01_Name;")]
    FungoidaSetisis,

    #[serde(rename = "$Codex_Ent_Fungoids_04_Name;")]
    FungoidaGelata,

    // Tubus
    #[serde(rename = "$Codex_Ent_Tubus_01_Name;")]
    TubusConifer,

    // Fonticulus
    #[serde(rename = "$Codex_Ent_Fonticulus_02_Name;")]
    FonticulusCampestris,

    // Frutexa
    #[serde(rename = "$Codex_Ent_Shrubs_01_Name;")]
    TrutexaFlabellum,

    // Conchas
    #[serde(rename = "$Codex_Ent_Conchas_01_Name;")]
    ConchasRenibus,
}
