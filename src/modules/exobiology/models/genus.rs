use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::modules::exobiology::Species;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Genus {
    #[serde(rename = "$Codex_Ent_Aleoids_Genus_Name;")]
    Aleoida,

    AmphoraPlant,

    // TODO needs to be verified
    #[serde(rename = "$Codex_Ent_Sphere_Name;")]
    Anemone,

    // TODO needs to be verified
    #[serde(rename = "$Codex_Ent_Cone_Name;")]
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

    #[serde(rename = "$Codex_Ent_Fumerolas_Genus_Name;")]
    Fumerola,

    #[serde(rename = "$Codex_Ent_Fungoids_Genus_Name;")]
    Fungoida,

    #[serde(rename = "$Codex_Ent_Osseus_Genus_Name;")]
    Osseus,

    #[serde(rename = "$Codex_Ent_Recepta_Genus_Name;")]
    Recepta,

    // TODO needs to be verified
    #[serde(rename = "$Codex_Ent_Tube_Name;")]
    SinuousTubers,

    #[serde(rename = "$Codex_Ent_Stratum_Genus_Name;")]
    Stratum,

    #[serde(rename = "$Codex_Ent_Tubus_Genus_Name;")]
    Tubus,

    #[serde(rename = "$Codex_Ent_Tussocks_Genus_Name;")]
    Tussock,

    // TODO needs to be verified
    #[serde(rename = "$Codex_Ent_Thargoid_Barnacle_Name;")]
    ThargoidBarnacle,

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

            Species::AnemonePrasinumBioluminescent
            | Species::AnemonePuniceum
            | Species::AnemoneRoseumBioluminescent
            | Species::AnemoneRoseum
            | Species::AnemoneBlatteumBioluminescent
            | Species::AnemoneLuteolum
            | Species::AnemoneRubeumBioluminescent
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
            | Species::CactoidaVermis
            | Species::CactoidaPeperatis => Genus::Cactoida,

            Species::CrystallineShards => Genus::CrystallineShards,

            Species::ClypeusSpeculumi | Species::ClypeusLacrimam | Species::ClypeusMargaritus => {
                Genus::Clypeus
            }

            Species::ConchaRenibus
            | Species::ConchaAureolas
            | Species::ConchaLabiata
            | Species::ConchaBiconcavis => Genus::Concha,

            Species::ElectricaePluma | Species::ElectricaeRadialem => Genus::Electricae,

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

            Species::ReceptaUmbrux | Species::ReceptaDeltahedronix | Species::ReceptaConditivus => {
                Genus::Recepta
            }

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

            Species::ThargoidBarnacleCommon
            | Species::ThargoidBarnacleLarge
            | Species::ThargoidBarnacleBarbs
            | Species::ThargoidBarnacleMatrixSubmerged
            | Species::ThargoidBarnacleMatrix => Genus::ThargoidBarnacle,
        }
    }
}

impl From<Species> for Genus {
    fn from(value: Species) -> Self {
        (&value).into()
    }
}

impl Display for Genus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Genus::Aleoida => "Aleoida",
                Genus::AmphoraPlant => "Amphora Plant",
                Genus::Anemone => "Anemone",
                Genus::BarkMound => "Bark Mound",
                Genus::Bacterium => "Bacterium",
                Genus::BrainTree => "Brain Tree",
                Genus::Cactoida => "Cactoida",
                Genus::Clypeus => "Clypeus",
                Genus::Concha => "Concha",
                Genus::CrystallineShards => "Crystalline Shards",
                Genus::Electricae => "Electricae",
                Genus::Fonticulua => "Fonticulua",
                Genus::Fruxeta => "Fruxeta",
                Genus::Fumerola => "Fumerola",
                Genus::Fungoida => "Fungoida",
                Genus::Osseus => "Osseus",
                Genus::Recepta => "Recepta",
                Genus::SinuousTubers => "Sinuous Tubers",
                Genus::Stratum => "Stratum",
                Genus::Tubus => "Tubus",
                Genus::Tussock => "Tussock",
                Genus::ThargoidBarnacle => "Thargoid Barnacle",

                #[cfg(not(feature = "strict"))]
                Genus::Unknown(unknown) => return write!(f, "Unknown genus: {}", unknown),
            }
        )
    }
}

impl Genus {
    /// The minimum distance in meters required between two samples.
    pub fn minimum_distance(&self) -> u16 {
        match self {
            Genus::Aleoida => 150,
            Genus::AmphoraPlant => 100,
            Genus::Anemone => 100,
            Genus::BarkMound => 100,
            Genus::Bacterium => 500,
            Genus::BrainTree => 100,
            Genus::Cactoida => 300,
            Genus::Clypeus => 150,
            Genus::Concha => 150,
            Genus::CrystallineShards => 100,
            Genus::Electricae => 1000,
            Genus::Fonticulua => 500,
            Genus::Fruxeta => 150,
            Genus::Fumerola => 100,
            Genus::Fungoida => 300,
            Genus::Osseus => 800,
            Genus::Recepta => 150,
            Genus::SinuousTubers => 100,
            Genus::Stratum => 500,
            Genus::Tubus => 800,
            Genus::Tussock => 200,

            // TODO check what this should be
            Genus::ThargoidBarnacle => 0,

            #[cfg(not(feature = "strict"))]
            Genus::Unknown(_) => 0,
        }
    }
}
