use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::exobiology::Genus;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum::EnumIter;
use thiserror::Error;
use crate::from_str_deserialize_impl;
use crate::modules::exobiology::models::spawn_condition::SpawnCondition;
use crate::modules::exobiology::r#static::species_spawn_conditions::SPECIES_SPAWN_CONDITIONS;

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
    AnemoneLuteolum, // TODO needs to be verified
    AnemoneCroceum, // TODO needs to be verified
    AnemonePuniceum, // TODO needs to be verified
    AnemoneRoseum, // TODO needs to be verified
    AnemoneBlatteumBioluminescent, // TODO needs to be verified
    AnemoneRubeumBioluminescent, // TODO needs to be verified
    AnemonePrasinumBioluminescent, // TODO needs to be verified
    AnemoneRoseumBioluminescent, // TODO needs to be verified

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
    BrainTreeRoseum, // TODO needs to be verified
    BrainTreeGypseeum, // TODO needs to be verified
    BrainTreeOstrinum, // TODO needs to be verified
    BrainTreeViride, // TODO needs to be verified
    BrainTreeLividum, // TODO needs to be verified
    BrainTreeAureum, // TODO needs to be verified
    BrainTreePuniceum, // TODO needs to be verified
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
    SinuousTubersRoseum, // TODO needs to be verified
    SinuousTubersPrasinum, // TODO needs to be verified
    SinuousTubersAlbidum, // TODO needs to be verified
    SinuousTubersCaeruleum, // TODO needs to be verified
    SinuousTubersBlatteum, // TODO needs to be verified
    SinuousTubersLindigoticum, // TODO needs to be verified
    SinuousTubersViolaceum, // TODO needs to be verified
    SinuousTubersViride, // TODO needs to be verified

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
    ThargoidBarnacleCommon, // TODO needs to be verified
    ThargoidBarnacleLarge, // TODO needs to be verified
    ThargoidBarnacleBarbs, // TODO needs to be verified
    ThargoidBarnacleMatrixSubmerged, // TODO needs to be verified
    ThargoidBarnacleMatrix, // TODO needs to be verified

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    Unknown(String),
}

#[derive(Debug, Error)]
pub enum SpeciesError {
    #[error("Failed to parse species: '{0}'")]
    FailedToParse(String),
}

impl FromStr for Species {
    type Err = SpeciesError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let string: &str = &s.to_ascii_lowercase();

        Ok(match string {
            "$codex_ent_aleoids_01_name;" => Species::AleoidaArcus,
            "$codex_ent_aleoids_02_name;" => Species::AleoidaCoronamus,
            "$codex_ent_aleoids_03_name;" => Species::AleoidaSpica,
            "$codex_ent_aleoids_04_name;" => Species::AleoidaLaminiae,
            "$codex_ent_aleoids_05_name;" => Species::AleoidaGravis,

            "$codex_ent_vents_name;" => Species::AmphoraPlant,

            "$codex_ent_sphere_name;" => Species::AnemoneLuteolum,
            "$codex_ent_sphereabcd_01_name;" => Species::AnemoneCroceum,
            "$codex_ent_sphereabcd_02_name;" => Species::AnemonePuniceum,
            "$codex_ent_sphereabcd_03_name;" => Species::AnemoneRoseum,
            "$codex_ent_sphereefgh_name;" => Species::AnemoneBlatteumBioluminescent,
            "$codex_ent_sphereefgh_01_name;" => Species::AnemoneRubeumBioluminescent,
            "$codex_ent_sphereefgh_02_name;" => Species::AnemonePrasinumBioluminescent,
            "$codex_ent_sphereefgh_03_name;" => Species::AnemoneRoseumBioluminescent,

            "$codex_ent_cone_name;" => Species::BarkMound,

            "$codex_ent_bacterial_01_name;" => Species::BacteriumAurasus,
            "$codex_ent_bacterial_02_name;" => Species::BacteriumNebulus,
            "$codex_ent_bacterial_03_name;" => Species::BacteriumScopulum,
            "$codex_ent_bacterial_04_name;" => Species::BacteriumAcies,
            "$codex_ent_bacterial_05_name;" => Species::BacteriumVesicula,
            "$codex_ent_bacterial_06_name;" => Species::BacteriumAlcyoneum,
            "$codex_ent_bacterial_07_name;" => Species::BacteriumTela,
            "$codex_ent_bacterial_08_name;" => Species::BacteriumInformem,
            "$codex_ent_bacterial_09_name;" => Species::BacteriumVolu,
            "$codex_ent_bacterial_10_name;" => Species::BacteriumBullaris,
            "$codex_ent_bacterial_11_name;" => Species::BacteriumOmentum,
            "$codex_ent_bacterial_12_name;" => Species::BacteriumCerbrus,
            "$codex_ent_bacterial_13_name;" => Species::BacteriumVerrata,

            "$codex_ent_seed_name;" => Species::BrainTreeRoseum,
            "$codex_ent_seedabcd_01_name;" => Species::BrainTreeGypseeum,
            "$codex_ent_seedabcd_02_name;" => Species::BrainTreeOstrinum,
            "$codex_ent_seedabcd_03_name;" => Species::BrainTreeViride,
            "$codex_ent_seedefgh_name;" => Species::BrainTreeLividum,
            "$codex_ent_seedefgh_01_name;" => Species::BrainTreeAureum,
            "$codex_ent_seedefgh_02_name;" => Species::BrainTreePuniceum,
            "$codex_ent_seedefgh_03_name;" => Species::BrainTreeLindigoticum,

            "$codex_ent_cactoid_01_name;" => Species::CactoidaCortexum,
            "$codex_ent_cactoid_02_name;" => Species::CactoidaLapis,
            "$codex_ent_cactoid_03_name;" => Species::CactoidaVermis,
            "$codex_ent_cactoid_04_name;" => Species::CactoidaPullulanta,
            "$codex_ent_cactoid_05_name;" => Species::CactoidaPeperatis,

            "$codex_ent_clypeus_01_name;" => Species::ClypeusLacrimam,
            "$codex_ent_clypeus_02_name;" => Species::ClypeusMargaritus,
            "$codex_ent_clypeus_03_name;" => Species::ClypeusSpeculumi,

            "$codex_ent_conchas_01_name;" => Species::ConchaRenibus,
            "$codex_ent_conchas_02_name;" => Species::ConchaAureolas,
            "$codex_ent_conchas_03_name;" => Species::ConchaLabiata,
            "$codex_ent_conchas_04_name;" => Species::ConchaBiconcavis,

            "$codex_ent_ground_struct_ice_name;" => Species::CrystallineShards,

            "$codex_ent_electricae_01_name;" => Species::ElectricaePluma,
            "$codex_ent_electricae_02_name;" => Species::ElectricaeRadialem,

            "$codex_ent_fonticulus_01_name;" => Species::FonticuluaSegmentatus,
            "$codex_ent_fonticulus_02_name;" => Species::FonticuluaCampestris,
            "$codex_ent_fonticulus_03_name;" => Species::FonticuluaUpupam,
            "$codex_ent_fonticulus_04_name;" => Species::FonticuluaLapida,
            "$codex_ent_fonticulus_05_name;" => Species::FonticuluaFluctus,
            "$codex_ent_fonticulus_06_name;" => Species::FonticuluaDigitos,

            "$codex_ent_shrubs_01_name;" => Species::FrutexaFlabellum,
            "$codex_ent_shrubs_02_name;" => Species::FrutexaAcus,
            "$codex_ent_shrubs_03_name;" => Species::FrutexaMetallicum,
            "$codex_ent_shrubs_04_name;" => Species::FrutexaFlammasis,
            "$codex_ent_shrubs_05_name;" => Species::FrutexaFera,
            "$codex_ent_shrubs_06_name;" => Species::FrutexaSponsae,
            "$codex_ent_shrubs_07_name;" => Species::FrutexaCollum,

            "$codex_ent_fumerolas_01_name;" => Species::FumerolaCarbosis,
            "$codex_ent_fumerolas_02_name;" => Species::FumerolaExtremus,
            "$codex_ent_fumerolas_03_name;" => Species::FumerolaNitris,
            "$codex_ent_fumerolas_04_name;" => Species::FumerolaAquatis,

            "$codex_ent_fungoids_01_name;" => Species::FungoidaSetisis,
            "$codex_ent_fungoids_02_name;" => Species::FungoidaStabitis,
            "$codex_ent_fungoids_03_name;" => Species::FungoidaBullarum,
            "$codex_ent_fungoids_04_name;" => Species::FungoidaGelata,

            "$codex_ent_osseus_01_name;" => Species::OsseusFractus,
            "$codex_ent_osseus_02_name;" => Species::OsseusDiscus,
            "$codex_ent_osseus_03_name;" => Species::OsseusSpiralis,
            "$codex_ent_osseus_04_name;" => Species::OsseusPumice,
            "$codex_ent_osseus_05_name;" => Species::OsseusCornibus,
            "$codex_ent_osseus_06_name;" => Species::OsseusPellebantus,

            "$codex_ent_recepta_01_name;" => Species::ReceptaUmbrux,
            "$codex_ent_recepta_02_name;" => Species::ReceptaDeltahedronix,
            "$codex_ent_recepta_03_name;" => Species::ReceptaConditivus,

            "$codex_ent_tube_name;" => Species::SinuousTubersRoseum,
            "$codex_ent_tubeabcd_01_name;" => Species::SinuousTubersPrasinum,
            "$codex_ent_tubeabcd_02_name;" => Species::SinuousTubersAlbidum,
            "$codex_ent_tubeabcd_03_name;" => Species::SinuousTubersCaeruleum,
            "$codex_ent_tubeefgh_name;" => Species::SinuousTubersBlatteum,
            "$codex_ent_tubeefgh_01_name;" => Species::SinuousTubersLindigoticum,
            "$codex_ent_tubeefgh_02_name;" => Species::SinuousTubersViolaceum,
            "$codex_ent_tubeefgh_03_name;" => Species::SinuousTubersViride,

            "$codex_ent_stratum_01_name;" => Species::StratumExcutitus,
            "$codex_ent_stratum_02_name;" => Species::StratumPaleas,
            "$codex_ent_stratum_03_name;" => Species::StratumLaminamus,
            "$codex_ent_stratum_04_name;" => Species::StratumAraneamus,
            "$codex_ent_stratum_05_name;" => Species::StratumLimaxus,
            "$codex_ent_stratum_06_name;" => Species::StratumCucumisis,
            "$codex_ent_stratum_07_name;" => Species::StratumTectonicas,
            "$codex_ent_stratum_08_name;" => Species::StratumFrigus,

            "$codex_ent_tubus_01_name;" => Species::TubusConifer,
            "$codex_ent_tubus_02_name;" => Species::TubusSororibus,
            "$codex_ent_tubus_03_name;" => Species::TubusCavas,
            "$codex_ent_tubus_04_name;" => Species::TubusRosarium,
            "$codex_ent_tubus_05_name;" => Species::TubusCompagibus,

            "$codex_ent_tussocks_01_name;" => Species::TussockPennata,
            "$codex_ent_tussocks_02_name;" => Species::TussockVentusa,
            "$codex_ent_tussocks_03_name;" => Species::TussockIgnis,
            "$codex_ent_tussocks_04_name;" => Species::TussockCultro,
            "$codex_ent_tussocks_05_name;" => Species::TussockCatena,
            "$codex_ent_tussocks_06_name;" => Species::TussockPennatis,
            "$codex_ent_tussocks_07_name;" => Species::TussockSerrati,
            "$codex_ent_tussocks_08_name;" => Species::TussockAlbata,
            "$codex_ent_tussocks_09_name;" => Species::TussockPropagito,
            "$codex_ent_tussocks_10_name;" => Species::TussockDivisa,
            "$codex_ent_tussocks_11_name;" => Species::TussockCaputus,
            "$codex_ent_tussocks_12_name;" => Species::TussockTriticum,
            "$codex_ent_tussocks_13_name;" => Species::TussockStigmasis,
            "$codex_ent_tussocks_14_name;" => Species::TussockVirgam,
            "$codex_ent_tussocks_15_name;" => Species::TussockCapillum,

            "$codex_ent_thargoid_barnacle_01_name;" => Species::ThargoidBarnacleCommon,
            "$codex_ent_thargoid_barnacle_02_name;" => Species::ThargoidBarnacleLarge,
            "$codex_ent_thargoid_barnacle_spikes_name;" => Species::ThargoidBarnacleBarbs,
            "$codex_ent_thargoid_barnacle_matrix_submerged_name;" => Species::ThargoidBarnacleMatrixSubmerged,
            "$codex_ent_thargoid_barnacle_matrix_name;" => Species::ThargoidBarnacleMatrix,

            #[cfg(feature = "allow-unknown")]
            _ => Species::Unknown(s.to_string()),

            #[cfg(not(feature = "allow-unknown"))]
            _ => return Err(SpeciesError::FailedToParse(s.to_string())),
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

impl Species {
    pub fn spawn_conditions(&self) -> &Vec<SpawnCondition> {
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

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use crate::modules::exobiology::Species;

    #[test]
    fn all_species_have_matching_spawn_conditions() {
        for species in Species::iter() {
            assert!(!species.spawn_conditions().is_empty());
        }
    }
}
