use std::fmt::{Display, Formatter};
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::from_str_deserialize_impl;
use crate::modules::exobiology::Species;

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Hash)]
pub enum Genus {
    Aleoida,
    AmphoraPlant,
    Anemone, // TODO needs to be verified
    BarkMound, // TODO needs to be verified
    Bacterium,
    BrainTree,
    Cactoida,
    Clypeus,
    Concha,
    CrystallineShards,
    Electricae,
    Fonticulua,
    Frutexa,
    Fumerola,
    Fungoida,
    Osseus,
    Recepta,
    SinuousTubers, // TODO needs to be verified
    Stratum,
    Tubus,
    Tussock,
    ThargoidBarnacle, // TODO needs to be verified

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    #[serde(untagged)]
    Unknown(String),
}

#[derive(Debug, Error)]
pub enum GenusError {
    #[error("Failed to parse genus: '{0}'")]
    FailedToParse(String),
}

impl FromStr for Genus {
    type Err = GenusError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let string: &str = &s.to_ascii_lowercase();

        Ok(match string {
            "$codex_ent_aleoids_genus_name;" => Genus::Aleoida,
            "$codex_ent_vents_name;" => Genus::AmphoraPlant,
            "$codex_ent_sphere_name;" => Genus::Anemone,
            "$codex_ent_cone_name;" => Genus::BarkMound,
            "$codex_ent_bacterial_genus_name;" => Genus::Bacterium,
            "$codex_ent_brancae_name;" => Genus::BrainTree,
            "$codex_ent_cactoid_genus_name;" => Genus::Cactoida,
            "$codex_ent_clypeus_genus_name;" => Genus::Clypeus,
            "$codex_ent_conchas_genus_name;" => Genus::Concha,
            "$codex_ent_ground_struct_ice_name;" => Genus::CrystallineShards,
            "$codex_ent_electricae_genus_name;" => Genus::Electricae,
            "$codex_ent_fonticulus_genus_name;" => Genus::Fonticulua,
            "$codex_ent_shrubs_genus_name;" => Genus::Frutexa,
            "$codex_ent_fumerolas_genus_name;" => Genus::Fumerola,
            "$codex_ent_fungoids_genus_name;" => Genus::Fungoida,
            "$codex_ent_osseus_genus_name;" => Genus::Osseus,
            "$codex_ent_recepta_genus_name;" => Genus::Recepta,
            "$codex_ent_tube_name;" => Genus::SinuousTubers,
            "$codex_ent_stratum_genus_name;" => Genus::Stratum,
            "$codex_ent_tubus_genus_name;" => Genus::Tubus,
            "$codex_ent_tussocks_genus_name;" => Genus::Tussock,
            "$codex_ent_thargoid_barnacle_name;" => Genus::ThargoidBarnacle,

            #[cfg(feature = "allow-unknown")]
            _ => Genus::Unknown(string.to_string()),

            #[cfg(not(feature = "allow-unknown"))]
            _ => return Err(GenusError::FailedToParse(string.to_string())),
        })
    }
}

from_str_deserialize_impl!(Genus);

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
            | Species::FrutexaCollum => Genus::Frutexa,

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

            #[cfg(feature = "allow-unknown")]
            Species::Unknown(unknown) => Genus::Unknown(format!("Unknown species: {}", unknown))
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
                Genus::Frutexa => "Frutexa",
                Genus::Fumerola => "Fumerola",
                Genus::Fungoida => "Fungoida",
                Genus::Osseus => "Osseus",
                Genus::Recepta => "Recepta",
                Genus::SinuousTubers => "Sinuous Tubers",
                Genus::Stratum => "Stratum",
                Genus::Tubus => "Tubus",
                Genus::Tussock => "Tussock",
                Genus::ThargoidBarnacle => "Thargoid Barnacle",

                #[cfg(feature = "allow-unknown")]
                Genus::Unknown(unknown) => return write!(f, "Unknown genus: {}", unknown),
            }
        )
    }
}

impl Genus {
    /// The minimum distance in meters required between two samples.
    pub fn minimum_distance(&self) -> u32 {
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
            Genus::Frutexa => 150,
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

            #[cfg(feature = "allow-unknown")]
            Genus::Unknown(_) => 0,
        }
    }

    pub fn id(&self) -> u64 {
        match self {
            Genus::Aleoida => 23100,
            Genus::AmphoraPlant => 2101400,
            Genus::Anemone => 2100400,
            Genus::BarkMound => 2100300,
            Genus::Bacterium => 23200,
            Genus::BrainTree => 2100200,
            Genus::Cactoida => 23300,
            Genus::Clypeus => 23400,
            Genus::Concha => 23500,
            Genus::CrystallineShards => 2101500,
            Genus::Electricae => 23600,
            Genus::Fonticulua => 23700,
            Genus::Frutexa => 24400,
            Genus::Fumerola => 23800,
            Genus::Fungoida => 23900,
            Genus::Osseus => 24000,
            Genus::Recepta => 24100,
            Genus::SinuousTubers => 2100500,
            Genus::Stratum => 24200,
            Genus::Tubus => 24300,
            Genus::Tussock => 24500,
            Genus::ThargoidBarnacle => 21000,

            #[cfg(feature = "allow-unknown")]
            Genus::Unknown(_) => 0,
        }
    }
}
