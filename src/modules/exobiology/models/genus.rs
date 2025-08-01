use crate::exploration::shared::codex_regex::CODEX_REGEX;
use crate::from_str_deserialize_impl;
use crate::modules::exobiology::Species;
use serde::Serialize;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(not(feature = "allow-unknown"), non_exhaustive)]
pub enum Genus {
    Aleoida,
    AmphoraPlant,
    Anemone,   // TODO needs to be verified
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

impl Genus {
    /// Whether the current variant is unknown.
    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    pub fn is_unknown(&self) -> bool {
        matches!(self, Genus::Unknown(_))
    }

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

#[derive(Debug, Error)]
pub enum GenusError {
    #[error("Failed to parse genus: '{0}'")]
    FailedToParse(String),
}

impl FromStr for Genus {
    type Err = GenusError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(captures) = CODEX_REGEX.captures(s) else {
            return Err(GenusError::FailedToParse(s.to_string()));
        };

        let string: &str = &captures
            .get(1)
            .expect("Should have been captured already")
            .as_str()
            .to_ascii_lowercase();

        Ok(match string {
            "aleoids_genus" => Genus::Aleoida,
            "vents" => Genus::AmphoraPlant,
            "sphere" => Genus::Anemone,
            "cone" => Genus::BarkMound,
            "bacterial_genus" => Genus::Bacterium,
            "brancae" => Genus::BrainTree,
            "cactoid_genus" => Genus::Cactoida,
            "clypeus_genus" => Genus::Clypeus,
            "conchas_genus" => Genus::Concha,
            "ground_struct_ice" | "ice_structures" => Genus::CrystallineShards,
            "electricae_genus" => Genus::Electricae,
            "fonticulus_genus" => Genus::Fonticulua,
            "shrubs_genus" => Genus::Frutexa,
            "fumerolas_genus" => Genus::Fumerola,
            "fungoids_genus" => Genus::Fungoida,
            "osseus_genus" => Genus::Osseus,
            "recepta_genus" => Genus::Recepta,
            "tube" => Genus::SinuousTubers,
            "stratum_genus" => Genus::Stratum,
            "tubus_genus" => Genus::Tubus,
            "tussocks_genus" => Genus::Tussock,
            "thargoid_barnacle" => Genus::ThargoidBarnacle,

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
            Species::Unknown(unknown) => Genus::Unknown(format!("Unknown species: {unknown}")),
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
                Genus::Unknown(unknown) => return write!(f, "Unknown genus: {unknown}"),
            }
        )
    }
}
