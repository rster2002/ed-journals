use serde::Deserialize;
use crate::models::journal_event_content::shared::exploration::species::Species;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum Genus {
    #[serde(rename = "$Codex_Ent_Aleoids_Genus_Name;")]
    Aleoida,

    AmphoraPlant,
    Anemone,
    BarkMound,

    #[serde(rename = "$Codex_Ent_Bacterial_Genus_Name;")]
    Bacterium,

    #[serde(rename = "$Codex_Ent_Brancae_Name;")]
    BrainTree,

    #[serde(rename = "$Codex_Ent_Cactoid_Genus_Name;")]
    Cactoida,

    #[serde(rename = "$Codex_Ent_Clypeus_Genus_Name;")]
    Clypeus,

    #[serde(rename = "$Codex_Ent_Conchas_Genus_Name;")]
    Concha,

    #[serde(rename = "$Codex_Ent_Ground_Struct_Ice_Name;")]
    CrystallineShards,

    #[serde(rename = "$Codex_Ent_Electricae_Genus_Name;")]
    Electricae,

    #[serde(rename = "$Codex_Ent_Fonticulus_Genus_Name;")]
    Fonticulua,

    #[serde(rename = "$Codex_Ent_Shrubs_Genus_Name;")]
    Fruxeta,

    Fumerola,

    #[serde(rename = "$Codex_Ent_Fungoids_Genus_Name;")]
    Fungoida,

    #[serde(rename = "$Codex_Ent_Osseus_Genus_Name;")]
    Osseus,

    #[serde(rename = "$Codex_Ent_Recepta_Genus_Name;")]
    Recepta,

    SinuousTubers,

    #[serde(rename = "$Codex_Ent_Stratum_Genus_Name;")]
    Stratum,

    #[serde(rename = "$Codex_Ent_Tubus_Genus_Name;")]
    Tubus,

    #[serde(rename = "$Codex_Ent_Tussocks_Genus_Name;")]
    Tussock,

    Trutexa,

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}

impl From<&Species> for Genus {
    fn from(value: &Species) -> Self {
        match value {
            Species::AleoidaLaminiae
            | Species::AleoidaGravis
            | Species::AleoidaSpica
            | Species::AleoidaCoronamus
            | Species::AleoidaArcus => Genus::Aleoida,

            Species::AmphoraPlant => Genus::AmphoraPlant,

            Species::AnemonePrasinusBioluminescent
            | Species::AnemonePrasinumBioluminescent
            | Species::AnemonePuniceus
            | Species::AnemonePuniceum
            | Species::AnemoneRoseusBioluminescent
            | Species::AnemoneRoseumBioluminescent
            | Species::AnemoneRoseus
            | Species::AnemoneRoseum
            | Species::AnemoneBlattinusBioluminescent
            | Species::AnemoneBlatteumBioluminescent
            | Species::AnemoneLuteus
            | Species::AnemoneLuteolum
            | Species::AnemoneRubensBioluminescent
            | Species::AnemoneRubeumBioluminescent
            | Species::AnemoneCroceus
            | Species::AnemoneCroceum => Genus::Anemone,

            Species::BarkMound => Genus::BarkMound,

            Species::BacteriumNebulus
            | Species::BacteriumAcies
            | Species::BacteriumOmentum
            | Species::BacteriumScopulum
            | Species::BacteriumVerrata
            | Species::BacteriumBullaris
            | Species::BacteriumAlcyoneum
            | Species::BacteriumVesicula
            | Species::BacteriumCerbrus
            | Species::BacteriumAurasus
            | Species::BacteriumInformem
            | Species::BacteriumVolu
            | Species::BacteriumTela => Genus::Bacterium,

            Species::BrainTreeAureum
            | Species::BrainTreeOstrinum
            | Species::BrainTreePuniceum
            | Species::BrainTreeLindigoticum
            | Species::BrainTreeGypseeum
            | Species::BrainTreeLividum
            | Species::BrainTreeViride
            | Species::BrainTreeRoseum => Genus::BrainTree,

            Species::CactoidaLapis
            | Species::CactoidaPullulanta
            | Species::CactoidaCortexum
            | Species::CactoidaVermis => Genus::Cactoida,

            Species::ClypeusSpeculumi
            | Species::ClypeusLacrimam
            | Species::ClypeusMargaritus => Genus::Clypeus,

            Species::ConchaRenibus
            | Species::ConchaAureolas
            | Species::ConchaLabiata
            | Species::ConchaBiconcavis => Genus::Concha,

            Species::ElectricaePluma
            | Species::ElectricaeRadialem => Genus::Electricae,

            Species::FonticuluaCampestris
            | Species::FonticuluaSegmentatus
            | Species::FonticuluaDigitos
            | Species::FonticuluaUpupam
            | Species::FonticuluaLapida
            | Species::FonticuluaFluctus => Genus::Fonticulua,

            Species::FrutexaFlabellum
            | Species::FrutexaFlammasis
            | Species::FrutexaMetallicum
            | Species::FrutexaAcus
            | Species::FrutexaFera
            | Species::FrutexaSponsae
            | Species::FrutexaCollum => Genus::Fruxeta,

            Species::FumerolaAquatis
            | Species::FumerolaCarbosis
            | Species::FumerolaExtremus
            | Species::FumerolaNitris => Genus::Fumerola,

            Species::FungoidaSetisis
            | Species::FungoidaGelata
            | Species::FungoidaBullarum
            | Species::FungoidaStabitis => Genus::Fungoida,

            Species::OsseusFractus
            | Species::OsseusSpiralis
            | Species::OsseusCornibus
            | Species::OsseusPumice
            | Species::OsseusPellebantus
            | Species::OsseusDiscus => Genus::Osseus,

            Species::ReceptaUmbrux
            | Species::ReceptaDeltahedronix
            | Species::ReceptaConditivus => Genus::Recepta,

            Species::SinuousTubersAlbidum
            | Species::SinuousTubersBlatteum
            | Species::SinuousTubersCaeruleum
            | Species::SinuousTubersLindigoticum
            | Species::SinuousTubersPrasinum
            | Species::SinuousTubersRoseum
            | Species::SinuousTubersViolaceum
            | Species::SinuousTubersViride => Genus::SinuousTubers,

            Species::StratumTectonicas
            | Species::StratumPaleas
            | Species::StratumFrigus
            | Species::StratumLaminamus
            | Species::StratumExcutitus
            | Species::StratumLimaxus
            | Species::StratumCucumisis
            | Species::StratumAraneamus => Genus::Stratum,

            Species::TubusConifer
            | Species::TubusSororibus
            | Species::TubusRosarium
            | Species::TubusCavas
            | Species::TubusCompagibus => Genus::Tubus,

            Species::TussockPennata
            | Species::TussockVentusa
            | Species::TussockIgnis
            | Species::TussockCultro
            | Species::TussockSerrati
            | Species::TussockAlbata
            | Species::TussockDivisa
            | Species::TussockCaputus
            | Species::TussockTriticum
            | Species::TussockStigmasis
            | Species::TussockCapillum
            | Species::TussockCatena
            | Species::TussockPropagito
            | Species::TussockPennatis
            | Species::TussockVirgam => Genus::Tussock,
        }
    }
}

// impl FromStr for Genus {
//     type Err = String;
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s {
//             "$Codex_Ent_Stratum_Genus_Name;" => Ok(Genus::Stratum),
//             "$Codex_Ent_Ground_Struct_Ice_Name;" => Ok(Genus::CrystallineShards),
//             "$Codex_Ent_Bacterial_Genus_Name;" => Ok(Genus::Bacterium),
//             "$Codex_Ent_Aleoids_Genus_Name;" => Ok(Genus::Aleoida),
//             "$Codex_Ent_Conchas_Genus_Name;" => Ok(Genus::Concha),
//             "$Codex_Ent_Fungoids_Genus_Name;" => Ok(Genus::Fungoida),
//             "$Codex_Ent_Osseus_Genus_Name;" => Ok(Genus::Osseus),
//             "$Codex_Ent_Shrubs_Genus_Name;" => Ok(Genus::Frutexa),
//             "$Codex_Ent_Tussocks_Genus_Name;" => Ok(Genus::Tussock),
//             "$Codex_Ent_Cactoid_Genus_Name;" => Ok(Genus::Cactoida),
//             "$Codex_Ent_Clypeus_Genus_Name;" => Ok(Genus::Clypeus),
//             "$Codex_Ent_Recepta_Genus_Name;" => Ok(Genus::Recepta),
//
//             #[cfg(not(feature = "strict"))]
//             _ => Ok(Genus::Unknown(s.to_string())),
//
//             #[cfg(feature = "strict")]
//             _ => Err(s.to_string()),
//         }
//     }
// }
//
// from_str_deserialize_impl!(Genus);
