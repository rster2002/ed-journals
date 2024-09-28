use crate::exobiology::Genus;
use crate::exploration::shared::codex_regex::CODEX_REGEX;
use crate::from_str_deserialize_impl;
use crate::modules::exobiology::models::spawn_condition::SpawnCondition;
use crate::modules::exobiology::r#static::species_spawn_conditions::SPECIES_SPAWN_CONDITIONS;
use serde::Serialize;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use strum::EnumIter;
use thiserror::Error;

#[derive(Debug, Serialize, Clone, Hash, Eq, PartialEq, EnumIter)]
pub enum Species {
    // Aleoids
    AleoidaArcus,
    AleoidaCoronamus,
    AleoidaSpica,
    AleoidaLaminiae,
    AleoidaGravis,

    // Amphora plant
    AmphoraPlant,

    // Anemones
    AnemoneLuteolum,               // TODO needs to be verified
    AnemoneCroceum,                // TODO needs to be verified
    AnemonePuniceum,               // TODO needs to be verified
    AnemoneRoseum,                 // TODO needs to be verified
    AnemoneBlatteumBioluminescent, // TODO needs to be verified
    AnemoneRubeumBioluminescent,   // TODO needs to be verified
    AnemonePrasinumBioluminescent, // TODO needs to be verified
    AnemoneRoseumBioluminescent,   // TODO needs to be verified

    // Bark mounds
    BarkMound,

    // Bacterium
    BacteriumAurasus,
    BacteriumNebulus,
    BacteriumScopulum,
    BacteriumAcies,
    BacteriumVesicula,
    BacteriumAlcyoneum,
    BacteriumTela,
    BacteriumInformem,
    BacteriumVolu,
    BacteriumBullaris,
    BacteriumOmentum,
    BacteriumCerbrus,
    BacteriumVerrata,

    // Brain tree
    BrainTreeRoseum,       // TODO needs to be verified
    BrainTreeGypseeum,     // TODO needs to be verified
    BrainTreeOstrinum,     // TODO needs to be verified
    BrainTreeViride,       // TODO needs to be verified
    BrainTreeLividum,      // TODO needs to be verified
    BrainTreeAureum,       // TODO needs to be verified
    BrainTreePuniceum,     // TODO needs to be verified
    BrainTreeLindigoticum, // TODO needs to be verified

    // Cactoida
    CactoidaCortexum,
    CactoidaLapis,
    CactoidaVermis,
    CactoidaPullulanta,
    CactoidaPeperatis,

    // Clypeus
    ClypeusLacrimam,
    ClypeusMargaritus,
    ClypeusSpeculumi,

    // Conchas
    ConchaRenibus,
    ConchaAureolas,
    ConchaLabiata,
    ConchaBiconcavis,

    // Crystalline shards
    CrystallineShards,

    // Electricae
    ElectricaePluma,
    ElectricaeRadialem,

    // Fonticulus
    FonticuluaSegmentatus,
    FonticuluaCampestris,
    FonticuluaUpupam,
    FonticuluaLapida,
    FonticuluaFluctus,
    FonticuluaDigitos,

    // Frutexa
    FrutexaFlabellum,
    FrutexaAcus,
    FrutexaMetallicum,
    FrutexaFlammasis,
    FrutexaFera,
    FrutexaSponsae,
    FrutexaCollum,

    // Fumerola
    FumerolaCarbosis,
    FumerolaExtremus,
    FumerolaNitris,
    FumerolaAquatis,

    // Fungoida
    FungoidaSetisis,
    FungoidaStabitis,
    FungoidaBullarum,
    FungoidaGelata,

    // Osseus
    OsseusFractus,
    OsseusDiscus,
    OsseusSpiralis,
    OsseusPumice,
    OsseusCornibus,
    OsseusPellebantus,

    // Recepta
    ReceptaUmbrux,
    ReceptaDeltahedronix,
    ReceptaConditivus,

    // Sinuous tubers
    SinuousTubersRoseum,       // TODO needs to be verified
    SinuousTubersPrasinum,     // TODO needs to be verified
    SinuousTubersAlbidum,      // TODO needs to be verified
    SinuousTubersCaeruleum,    // TODO needs to be verified
    SinuousTubersBlatteum,     // TODO needs to be verified
    SinuousTubersLindigoticum, // TODO needs to be verified
    SinuousTubersViolaceum,    // TODO needs to be verified
    SinuousTubersViride,       // TODO needs to be verified

    // Stratum
    StratumExcutitus,
    StratumPaleas,
    StratumLaminamus,
    StratumAraneamus,
    StratumLimaxus,
    StratumCucumisis,
    StratumTectonicas,
    StratumFrigus,

    // Tubus
    TubusConifer,
    TubusSororibus,
    TubusCavas,
    TubusRosarium,
    TubusCompagibus,

    // Tussock
    TussockPennata,
    TussockVentusa,
    TussockIgnis,
    TussockCultro,
    TussockCatena,
    TussockPennatis,
    TussockSerrati,
    TussockAlbata,
    TussockPropagito,
    TussockDivisa,
    TussockCaputus,
    TussockTriticum,
    TussockStigmasis,
    TussockVirgam,
    TussockCapillum,

    // Thargoid barnacles
    ThargoidBarnacleCommon,          // TODO needs to be verified
    ThargoidBarnacleLarge,           // TODO needs to be verified
    ThargoidBarnacleBarbs,           // TODO needs to be verified
    ThargoidBarnacleMatrixSubmerged, // TODO needs to be verified
    ThargoidBarnacleMatrix,          // TODO needs to be verified

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    Unknown(String),
}

#[cfg(feature = "allow-unknown")]
lazy_static! {
    static ref UNKNOWN_SPAWN_CONDITIONS: Vec<SpawnCondition> = vec![SpawnCondition::Special,];
}

impl Species {
    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    pub fn is_unknown(&self) -> bool {
        matches!(self, Species::Unknown(_))
    }

    pub fn spawn_conditions(&self) -> &Vec<SpawnCondition> {
        #[cfg(feature = "allow-unknown")]
        if let Species::Unknown(_) = self {
            return &UNKNOWN_SPAWN_CONDITIONS;
        }

        &SPECIES_SPAWN_CONDITIONS
            .iter()
            .find(|(species, _)| species == self)
            .expect("Species should always have a matching spawning condition")
            .1
    }

    pub fn genus(&self) -> Genus {
        self.into()
    }

    pub fn base_value(&self) -> u64 {
        match self {
            Species::AleoidaArcus => 7_252_500,
            Species::AleoidaCoronamus => 6_284_600,
            Species::AleoidaSpica => 3_385_200,
            Species::AleoidaLaminiae => 3_385_200,
            Species::AleoidaGravis => 12_934_900,

            // TODO this needs a value
            Species::AmphoraPlant => 0,

            // TODO these need values
            Species::AnemoneLuteolum => 0,
            Species::AnemoneCroceum => 0,
            Species::AnemonePuniceum => 0,
            Species::AnemoneRoseum => 0,
            Species::AnemoneBlatteumBioluminescent => 0,
            Species::AnemoneRubeumBioluminescent => 0,
            Species::AnemonePrasinumBioluminescent => 0,
            Species::AnemoneRoseumBioluminescent => 0,

            // TODO this needs a value
            Species::BarkMound => 0,

            Species::BacteriumAurasus => 1_000_000,
            Species::BacteriumNebulus => 5_289_900,
            Species::BacteriumScopulum => 4_934_500,
            Species::BacteriumAcies => 1_000_000,
            Species::BacteriumVesicula => 1_000_000,
            Species::BacteriumAlcyoneum => 1_658_500,
            Species::BacteriumTela => 1_949_000,
            Species::BacteriumInformem => 8_418_000,
            Species::BacteriumVolu => 7_774_700,
            Species::BacteriumBullaris => 1_152_500,
            Species::BacteriumOmentum => 4_689_800,
            Species::BacteriumCerbrus => 1_689_800,
            Species::BacteriumVerrata => 3_897_000,

            // TODO these need values
            Species::BrainTreeRoseum => 0,
            Species::BrainTreeGypseeum => 0,
            Species::BrainTreeOstrinum => 0,
            Species::BrainTreeViride => 0,
            Species::BrainTreeLividum => 0,
            Species::BrainTreeAureum => 0,
            Species::BrainTreePuniceum => 0,
            Species::BrainTreeLindigoticum => 0,

            Species::CactoidaCortexum => 3_667_600,
            Species::CactoidaLapis => 2_483_600,
            Species::CactoidaVermis => 16_202_800,
            Species::CactoidaPullulanta => 3_667_600,
            Species::CactoidaPeperatis => 2_483_600,

            Species::ClypeusLacrimam => 8_418_000,
            Species::ClypeusMargaritus => 11_873_200,
            Species::ClypeusSpeculumi => 16_202_800,

            Species::ConchaRenibus => 4_572_400,
            Species::ConchaAureolas => 7_774_700,
            Species::ConchaLabiata => 2_352_400,
            Species::ConchaBiconcavis => 19_010_800,

            Species::CrystallineShards => 1_628_800,

            Species::ElectricaePluma => 6_284_600,
            Species::ElectricaeRadialem => 6_284_600,

            Species::FonticuluaSegmentatus => 19_010_800,
            Species::FonticuluaCampestris => 1_000_000,
            Species::FonticuluaUpupam => 5_727_600,
            Species::FonticuluaLapida => 3_111_000,
            Species::FonticuluaFluctus => 20_000_000,
            Species::FonticuluaDigitos => 1_804_100,

            Species::FumerolaCarbosis => 6_284_600,
            Species::FumerolaExtremus => 16_202_800,
            Species::FumerolaNitris => 7_500_900,
            Species::FumerolaAquatis => 6_284_600,

            Species::FungoidaSetisis => 1_670_100,
            Species::FungoidaStabitis => 2_680_300,
            Species::FungoidaBullarum => 3_703_200,
            Species::FungoidaGelata => 3_330_300,

            Species::OsseusFractus => 4_027_800,
            Species::OsseusDiscus => 12_934_900,
            Species::OsseusSpiralis => 2_404_700,
            Species::OsseusPumice => 3_156_300,
            Species::OsseusCornibus => 1_483_000,
            Species::OsseusPellebantus => 9_739_000,

            Species::ReceptaUmbrux => 12_934_900,
            Species::ReceptaDeltahedronix => 16_202_800,
            Species::ReceptaConditivus => 14_313_700,

            Species::StratumExcutitus => 2_448_900,
            Species::StratumPaleas => 1_362_000,
            Species::StratumLaminamus => 2_788_300,
            Species::StratumAraneamus => 2_448_900,
            Species::StratumLimaxus => 1_362_000,
            Species::StratumCucumisis => 16_202_800,
            Species::StratumTectonicas => 19_010_800,
            Species::StratumFrigus => 2_637_500,

            Species::TubusConifer => 2_415_500,
            Species::TubusSororibus => 5_727_600,
            Species::TubusCavas => 11_873_200,
            Species::TubusRosarium => 2_637_500,
            Species::TubusCompagibus => 7_774_700,

            Species::FrutexaFlabellum => 1_808_900,
            Species::FrutexaAcus => 7_774_700,
            Species::FrutexaMetallicum => 1_632_500,
            Species::FrutexaFlammasis => 10_326_000,
            Species::FrutexaFera => 1_632_500,
            Species::FrutexaSponsae => 5_988_000,
            Species::FrutexaCollum => 1_639_800,

            Species::TussockPennata => 5_853_800,
            Species::TussockVentusa => 3_227_700,
            Species::TussockIgnis => 1_849_000,
            Species::TussockCultro => 1_766_600,
            Species::TussockCatena => 1_766_600,
            Species::TussockPennatis => 1_000_000,
            Species::TussockSerrati => 4_447_100,
            Species::TussockAlbata => 3_252_500,
            Species::TussockPropagito => 1_000_000,
            Species::TussockDivisa => 1_766_600,
            Species::TussockCaputus => 3_472_400,
            Species::TussockTriticum => 7_774_700,
            Species::TussockStigmasis => 19_010_800,
            Species::TussockVirgam => 14_313_700,
            Species::TussockCapillum => 7_025_800,

            // TODO these need values
            Species::ThargoidBarnacleCommon => 0,
            Species::ThargoidBarnacleLarge => 0,
            Species::ThargoidBarnacleBarbs => 0,
            Species::ThargoidBarnacleMatrixSubmerged => 0,
            Species::ThargoidBarnacleMatrix => 2_313_500,

            // TODO these need values
            Species::SinuousTubersRoseum => 0,
            Species::SinuousTubersPrasinum => 0,
            Species::SinuousTubersAlbidum => 0,
            Species::SinuousTubersCaeruleum => 0,
            Species::SinuousTubersBlatteum => 0,
            Species::SinuousTubersLindigoticum => 0,
            Species::SinuousTubersViolaceum => 0,
            Species::SinuousTubersViride => 0,

            #[cfg(feature = "allow-unknown")]
            Species::Unknown(_) => 0,
        }
    }
}

#[derive(Debug, Error)]
pub enum SpeciesError {
    #[error("Failed to parse species: '{0}'")]
    FailedToParse(String),

    #[error("Unknown species: '{0}'")]
    UnknownSpecies(String),
}

impl FromStr for Species {
    type Err = SpeciesError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(captures) = CODEX_REGEX.captures(s) else {
            return Err(SpeciesError::FailedToParse(s.to_string()));
        };

        let string: &str = &captures
            .get(1)
            .expect("Should have been captured already")
            .as_str()
            .to_ascii_lowercase();

        Ok(match string {
            "aleoids_01" => Species::AleoidaArcus,
            "aleoids_02" => Species::AleoidaCoronamus,
            "aleoids_03" => Species::AleoidaSpica,
            "aleoids_04" => Species::AleoidaLaminiae,
            "aleoids_05" => Species::AleoidaGravis,

            "vents" => Species::AmphoraPlant,

            "sphere" => Species::AnemoneLuteolum,
            "sphereabcd_01" => Species::AnemoneCroceum,
            "sphereabcd_02" => Species::AnemonePuniceum,
            "sphereabcd_03" => Species::AnemoneRoseum,
            "sphereefgh" => Species::AnemoneBlatteumBioluminescent,
            "sphereefgh_01" => Species::AnemoneRubeumBioluminescent,
            "sphereefgh_02" => Species::AnemonePrasinumBioluminescent,
            "sphereefgh_03" => Species::AnemoneRoseumBioluminescent,

            "cone" => Species::BarkMound,

            "bacterial_01" => Species::BacteriumAurasus,
            "bacterial_02" => Species::BacteriumNebulus,
            "bacterial_03" => Species::BacteriumScopulum,
            "bacterial_04" => Species::BacteriumAcies,
            "bacterial_05" => Species::BacteriumVesicula,
            "bacterial_06" => Species::BacteriumAlcyoneum,
            "bacterial_07" => Species::BacteriumTela,
            "bacterial_08" => Species::BacteriumInformem,
            "bacterial_09" => Species::BacteriumVolu,
            "bacterial_10" => Species::BacteriumBullaris,
            "bacterial_11" => Species::BacteriumOmentum,
            "bacterial_12" => Species::BacteriumCerbrus,
            "bacterial_13" => Species::BacteriumVerrata,

            "seed" => Species::BrainTreeRoseum,
            "seedabcd_01" => Species::BrainTreeGypseeum,
            "seedabcd_02" => Species::BrainTreeOstrinum,
            "seedabcd_03" => Species::BrainTreeViride,
            "seedefgh" => Species::BrainTreeLividum,
            "seedefgh_01" => Species::BrainTreeAureum,
            "seedefgh_02" => Species::BrainTreePuniceum,
            "seedefgh_03" => Species::BrainTreeLindigoticum,

            "cactoid_01" => Species::CactoidaCortexum,
            "cactoid_02" => Species::CactoidaLapis,
            "cactoid_03" => Species::CactoidaVermis,
            "cactoid_04" => Species::CactoidaPullulanta,
            "cactoid_05" => Species::CactoidaPeperatis,

            "clypeus_01" => Species::ClypeusLacrimam,
            "clypeus_02" => Species::ClypeusMargaritus,
            "clypeus_03" => Species::ClypeusSpeculumi,

            "conchas_01" => Species::ConchaRenibus,
            "conchas_02" => Species::ConchaAureolas,
            "conchas_03" => Species::ConchaLabiata,
            "conchas_04" => Species::ConchaBiconcavis,

            "ground_struct_ice" => Species::CrystallineShards,

            "electricae_01" => Species::ElectricaePluma,
            "electricae_02" => Species::ElectricaeRadialem,

            "fonticulus_01" => Species::FonticuluaSegmentatus,
            "fonticulus_02" => Species::FonticuluaCampestris,
            "fonticulus_03" => Species::FonticuluaUpupam,
            "fonticulus_04" => Species::FonticuluaLapida,
            "fonticulus_05" => Species::FonticuluaFluctus,
            "fonticulus_06" => Species::FonticuluaDigitos,

            "shrubs_01" => Species::FrutexaFlabellum,
            "shrubs_02" => Species::FrutexaAcus,
            "shrubs_03" => Species::FrutexaMetallicum,
            "shrubs_04" => Species::FrutexaFlammasis,
            "shrubs_05" => Species::FrutexaFera,
            "shrubs_06" => Species::FrutexaSponsae,
            "shrubs_07" => Species::FrutexaCollum,

            "fumerolas_01" => Species::FumerolaCarbosis,
            "fumerolas_02" => Species::FumerolaExtremus,
            "fumerolas_03" => Species::FumerolaNitris,
            "fumerolas_04" => Species::FumerolaAquatis,

            "fungoids_01" => Species::FungoidaSetisis,
            "fungoids_02" => Species::FungoidaStabitis,
            "fungoids_03" => Species::FungoidaBullarum,
            "fungoids_04" => Species::FungoidaGelata,

            "osseus_01" => Species::OsseusFractus,
            "osseus_02" => Species::OsseusDiscus,
            "osseus_03" => Species::OsseusSpiralis,
            "osseus_04" => Species::OsseusPumice,
            "osseus_05" => Species::OsseusCornibus,
            "osseus_06" => Species::OsseusPellebantus,

            "recepta_01" => Species::ReceptaUmbrux,
            "recepta_02" => Species::ReceptaDeltahedronix,
            "recepta_03" => Species::ReceptaConditivus,

            "tube" => Species::SinuousTubersRoseum,
            "tubeabcd_01" => Species::SinuousTubersPrasinum,
            "tubeabcd_02" => Species::SinuousTubersAlbidum,
            "tubeabcd_03" => Species::SinuousTubersCaeruleum,
            "tubeefgh" => Species::SinuousTubersBlatteum,
            "tubeefgh_01" => Species::SinuousTubersLindigoticum,
            "tubeefgh_02" => Species::SinuousTubersViolaceum,
            "tubeefgh_03" => Species::SinuousTubersViride,

            "stratum_01" => Species::StratumExcutitus,
            "stratum_02" => Species::StratumPaleas,
            "stratum_03" => Species::StratumLaminamus,
            "stratum_04" => Species::StratumAraneamus,
            "stratum_05" => Species::StratumLimaxus,
            "stratum_06" => Species::StratumCucumisis,
            "stratum_07" => Species::StratumTectonicas,
            "stratum_08" => Species::StratumFrigus,

            "tubus_01" => Species::TubusConifer,
            "tubus_02" => Species::TubusSororibus,
            "tubus_03" => Species::TubusCavas,
            "tubus_04" => Species::TubusRosarium,
            "tubus_05" => Species::TubusCompagibus,

            "tussocks_01" => Species::TussockPennata,
            "tussocks_02" => Species::TussockVentusa,
            "tussocks_03" => Species::TussockIgnis,
            "tussocks_04" => Species::TussockCultro,
            "tussocks_05" => Species::TussockCatena,
            "tussocks_06" => Species::TussockPennatis,
            "tussocks_07" => Species::TussockSerrati,
            "tussocks_08" => Species::TussockAlbata,
            "tussocks_09" => Species::TussockPropagito,
            "tussocks_10" => Species::TussockDivisa,
            "tussocks_11" => Species::TussockCaputus,
            "tussocks_12" => Species::TussockTriticum,
            "tussocks_13" => Species::TussockStigmasis,
            "tussocks_14" => Species::TussockVirgam,
            "tussocks_15" => Species::TussockCapillum,

            "thargoid_barnacle_01" => Species::ThargoidBarnacleCommon,
            "thargoid_barnacle_02" => Species::ThargoidBarnacleLarge,
            "thargoid_barnacle_spikes" => Species::ThargoidBarnacleBarbs,
            "thargoid_barnacle_matrix_submerged" => Species::ThargoidBarnacleMatrixSubmerged,
            "thargoid_barnacle_matrix" => Species::ThargoidBarnacleMatrix,

            #[cfg(feature = "allow-unknown")]
            _ => Species::Unknown(s.to_string()),

            #[cfg(not(feature = "allow-unknown"))]
            _ => return Err(SpeciesError::UnknownSpecies(s.to_string())),
        })
    }
}

from_str_deserialize_impl!(Species);

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

                Species::AnemoneLuteolum => "Luteolum Anemone",
                Species::AnemoneBlatteumBioluminescent => "Blatteum Bioluminescent Anemone",
                Species::AnemoneCroceum => "Croceum Anemone",
                Species::AnemonePrasinumBioluminescent => "Prasinum Bioluminescent Anemone",
                Species::AnemonePuniceum => "Puniceum Anemone",
                Species::AnemoneRoseum => "Roseum Anemone",
                Species::AnemoneRoseumBioluminescent => "Roseum Bioluminescent Anemone",
                Species::AnemoneRubeumBioluminescent => "Rubeum Bioluminescent Anemone",

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

                Species::BrainTreeAureum => "Aureum Brain Tree ",
                Species::BrainTreeGypseeum => "Gypseeum Brain Tree ",
                Species::BrainTreeLindigoticum => "Lindigoticum Brain Tree ",
                Species::BrainTreeLividum => "Lividum Brain Tree ",
                Species::BrainTreeOstrinum => "Ostrinum Brain Tree ",
                Species::BrainTreePuniceum => "Puniceum Brain Tree ",
                Species::BrainTreeRoseum => "Roseum Brain Tree ",
                Species::BrainTreeViride => "Viride Brain Tree ",

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

                Species::SinuousTubersAlbidum => "Albidum Sinuous Tubers",
                Species::SinuousTubersBlatteum => "Blatteum Sinuous Tubers",
                Species::SinuousTubersCaeruleum => "Caeruleum Sinuous Tubers",
                Species::SinuousTubersLindigoticum => "Lindigoticum Sinuous Tubers",
                Species::SinuousTubersPrasinum => "Prasinum Sinuous Tubers",
                Species::SinuousTubersRoseum => "Roseum Sinuous Tubers",
                Species::SinuousTubersViolaceum => "Violaceum Sinuous Tubers",
                Species::SinuousTubersViride => "Viride Sinuous Tubers",

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

                Species::ThargoidBarnacleCommon => "Common Thargoid Barnacle",
                Species::ThargoidBarnacleLarge => "Large Thargoid Barnacle",
                Species::ThargoidBarnacleBarbs => "Thargoid Barnacle Barbs",
                Species::ThargoidBarnacleMatrixSubmerged => "Thargoid Barnacle Matrix Submerged",
                Species::ThargoidBarnacleMatrix => "Thargoid Barnacle Matrix",

                #[cfg(feature = "allow-unknown")]
                Species::Unknown(unknown) => return write!(f, "Unknown species: {}", unknown),
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use crate::modules::exobiology::Species;

    #[test]
    fn all_species_have_matching_spawn_conditions() {
        for species in Species::iter() {
            dbg!(&species);
            assert!(!species.spawn_conditions().is_empty());
        }
    }
}
