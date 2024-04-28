use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Variant {
    // Tussock
    #[serde(rename = "$Codex_Ent_Tussocks_10_K_Name;")]
    TussockDivisaGreen,

    #[serde(rename = "$Codex_Ent_Tussocks_10_F_Name;")]
    TussockDivisaYellow,

    #[serde(rename = "$Codex_Ent_Tussocks_10_T_Name;")]
    TussockDivisaTeal,

    #[serde(rename = "$Codex_Ent_Tussocks_12_F_Name;")]
    TussockTriticumYellow,

    #[serde(rename = "$Codex_Ent_Tussocks_03_M_Name;")]
    TussockIgnisEmerald,

    #[serde(rename = "$Codex_Ent_Tussocks_11_F_Name;")]
    TussockCaputusYellow,

    #[serde(rename = "$Codex_Ent_Tussocks_08_G_Name;")]
    TussockAlbataLime,

    #[serde(rename = "$Codex_Ent_Tussocks_04_K_Name;")]
    TussockCultroGreen,

    #[serde(rename = "$Codex_Ent_Tussocks_07_F_Name;")]
    TussockSerratiYellow,

    // Clypeus
    #[serde(rename = "$Codex_Ent_Clypeus_03_F_Name;")]
    ClypeusSpeculumiMauve,

    // Stratum
    #[serde(rename = "$Codex_Ent_Stratum_07_T_Name;")]
    StratumTectonicasGrey,

    #[serde(rename = "$Codex_Ent_Stratum_07_M_Name;")]
    StratumTectonicasGreen,

    #[serde(rename = "$Codex_Ent_Stratum_07_F_Name;")]
    StratumTectonicasEmerald,

    #[serde(rename = "$Codex_Ent_Stratum_07_K_Name;")]
    StratumTectonicasLime,

    #[serde(rename = "$Codex_Ent_Stratum_02_F_Name;")]
    StratumPaleasEmerald,

    #[serde(rename = "$Codex_Ent_Stratum_08_M_Name;")]
    StratumFrigusGreen,

    #[serde(rename = "$Codex_Ent_Stratum_08_F_Name;")]
    StratumFrigusEmerald,

    // Cactoid
    #[serde(rename = "$Codex_Ent_Cactoid_02_T_Name;")]
    CactoidLapisOrange,

    #[serde(rename = "$Codex_Ent_Cactoid_04_F_Name;")]
    CactoidPullulantaYellow,

    // Osseus
    #[serde(rename = "$Codex_Ent_Osseus_01_T_Name;")]
    OsseusFractusEmerald,

    #[serde(rename = "$Codex_Ent_Osseus_03_F_Name;")]
    OsseusSpiralisTurquoise,

    #[serde(rename = "$Codex_Ent_Osseus_03_K_Name;")]
    OsseusSpiralisIndigo,

    #[serde(rename = "$Codex_Ent_Osseus_05_F_Name;")]
    OsseusCornibusTurquoise,

    // Recepta
    #[serde(rename = "$Codex_Ent_Recepta_03_Yttrium_Name;")]
    ReceptraConditivusGreen,

    // Aleoids
    #[serde(rename = "$Codex_Ent_Aleoids_03_F_Name;")]
    AleoidaSpicaTeal,

    #[serde(rename = "$Codex_Ent_Aleoids_02_F_Name;")]
    AleoidaCoronamusTeal,

    #[serde(rename = "$Codex_Ent_Aleoids_01_K_Name;")]
    AleoidaArcusTurquoise,

    #[serde(rename = "$Codex_Ent_Aleoids_01_A_Name;")]
    AleoidaArcusGreen,

    // Bacterium
    #[serde(rename = "$Codex_Ent_Bacterial_06_F_Name;")]
    BacteriumAlcyoneumLime,

    #[serde(rename = "$Codex_Ent_Bacterial_06_T_Name;")]
    BacteriumAlcyoneumRed,

    #[serde(rename = "$Codex_Ent_Bacterial_06_K_Name;")]
    BacteriumAlcyoneumGreen,

    #[serde(rename = "$Codex_Ent_Bacterial_05_Yttrium_Name;")]
    BacteriumVesiculaLime,

    #[serde(rename = "$Codex_Ent_Bacterial_05_Ruthenium_Name;")]
    BacteriumVesiculaMulberry,

    #[serde(rename = "$Codex_Ent_Bacterial_05_Tellurium_Name;")]
    BacteriumVesiculaRed,

    #[serde(rename = "$Codex_Ent_Bacterial_05_Polonium_Name;")]
    BacteriumVesiculaOrange,

    #[serde(rename = "$Codex_Ent_Bacterial_12_K_Name;")]
    BacteriumCerbrusGreen,

    #[serde(rename = "$Codex_Ent_Bacterial_12_M_Name;")]
    BacteriumCerbrusTeal,

    #[serde(rename = "$Codex_Ent_Bacterial_12_F_Name;")]
    BacteriumCerbrusLime,

    #[serde(rename = "$Codex_Ent_Bacterial_01_T_Name;")]
    BacteriumAurasusRed,

    #[serde(rename = "$Codex_Ent_Bacterial_01_F_Name;")]
    BacteriumAurasusLime,

    #[serde(rename = "$Codex_Ent_Bacterial_01_M_Name;")]
    BacteriumAurasusTeal,

    #[serde(rename = "$Codex_Ent_Bacterial_01_G_Name;")]
    BacteriumAurasusEmerald,

    #[serde(rename = "$Codex_Ent_Bacterial_08_Antimony_Name;")]
    BacteriumInformemRed,

    // Fungoida
    #[serde(rename = "$Codex_Ent_Fungoids_01_Yttrium_Name;")]
    FungoidaSetisisOrange,

    #[serde(rename = "$Codex_Ent_Fungoids_01_Tellurium_Name;")]
    FungoidaSetisisYellow,

    #[serde(rename = "$Codex_Ent_Fungoids_01_Antimony_Name;")]
    FungoidaSetisisPeach,

    #[serde(rename = "$Codex_Ent_Fungoids_04_Cadmium_Name;")]
    FungoidaGelataCyan,

    #[serde(rename = "$Codex_Ent_Fungoids_04_Tungsten_Name;")]
    FungoidaGelataOrange,

    #[serde(rename = "$Codex_Ent_Fungoids_04_Molybdenum_Name;")]
    FungoidaGelataMulberry,

    // Tubus
    #[serde(rename = "$Codex_Ent_Tubus_01_M_Name;")]
    TubusConiferTeal,

    #[serde(rename = "$Codex_Ent_Tubus_01_A_Name;")]
    TubusConiferIndigo,

    #[serde(rename = "$Codex_Ent_Tubus_01_F_Name;")]
    TubusConiferGrey,

    // Fonticulus
    #[serde(rename = "$Codex_Ent_Fonticulus_02_M_Name;")]
    FonticulusCampestrisAmethyst,

    // Frutexa
    #[serde(rename = "$Codex_Ent_Shrubs_01_F_Name;")]
    TrutexaFlabellumGreen,

    // Conchas
    #[serde(rename = "$Codex_Ent_Conchas_01_Cadmium_Name;")]
    ConchasRenibusRed,

    #[serde(rename = "$Codex_Ent_Conchas_01_Molybdenum_Name;")]
    ConchasRenibusPeach,
}
