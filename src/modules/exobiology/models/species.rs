use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum::EnumIter;
use crate::exobiology::Genus;

use crate::modules::exobiology::models::spawn_condition::SpawnCondition;
use crate::modules::exobiology::r#static::species_spawn_conditions::SPECIES_SPAWN_CONDITIONS;

#[derive(Debug, Serialize, Deserialize, Clone, Hash, Eq, PartialEq, EnumIter)]
pub enum Species {
    // Aleoids
    #[serde(rename = "$Codex_Ent_Aleoids_01_Name;")]
    AleoidaArcus,

    #[serde(rename = "$Codex_Ent_Aleoids_02_Name;")]
    AleoidaCoronamus,

    #[serde(rename = "$Codex_Ent_Aleoids_03_Name;")]
    AleoidaSpica,

    #[serde(rename = "$Codex_Ent_Aleoids_04_Name;")]
    AleoidaLaminiae,

    #[serde(rename = "$Codex_Ent_Aleoids_05_Name;")]
    AleoidaGravis,

    // Amphora plant
    #[serde(rename = "$Codex_Ent_Vents_Name;")]
    AmphoraPlant,

    // Anemones
    // TODO needs to be verified
    #[serde(rename = "$Codex_Ent_Sphere_Name;")]
    AnemoneLuteolum,

    // TODO needs to be verified
    #[serde(rename = "$Codex_Ent_SphereABCD_01_Name;")]
    AnemoneCroceum,

    // TODO needs to be verified
    #[serde(rename = "$Codex_Ent_SphereABCD_02_Name;")]
    AnemonePuniceum,

    // TODO needs to be verified
    #[serde(rename = "$Codex_Ent_SphereABCD_03_Name;")]
    AnemoneRoseum,

    // TODO needs to be verified
    #[serde(rename = "$Codex_Ent_SphereEFGH_Name;")]
    AnemoneBlatteumBioluminescent,

    // TODO needs to be verified
    #[serde(rename = "$Codex_Ent_SphereEFGH_01_Name;")]
    AnemoneRubeumBioluminescent,

    // TODO needs to be verified
    #[serde(rename = "$Codex_Ent_SphereEFGH_02_Name;")]
    AnemonePrasinumBioluminescent,

    // TODO needs to be verified
    #[serde(rename = "$Codex_Ent_SphereEFGH_03_Name;")]
    AnemoneRoseumBioluminescent,

    // Bark mounds
    // TODO needs to be verified
    #[serde(rename = "$Codex_Ent_Cone_Name;")]
    BarkMound,

    // Bacterium
    #[serde(rename = "$Codex_Ent_Bacterial_01_Name;")]
    BacteriumAurasus,

    #[serde(rename = "$Codex_Ent_Bacterial_02_Name;")]
    BacteriumNebulus,

    #[serde(rename = "$Codex_Ent_Bacterial_03_Name;")]
    BacteriumScopulum,

    #[serde(rename = "$Codex_Ent_Bacterial_04_Name;")]
    BacteriumAcies,

    #[serde(rename = "$Codex_Ent_Bacterial_05_Name;")]
    BacteriumVesicula,

    #[serde(rename = "$Codex_Ent_Bacterial_06_Name;")]
    BacteriumAlcyoneum,

    #[serde(rename = "$Codex_Ent_Bacterial_07_Name;")]
    BacteriumTela,

    #[serde(rename = "$Codex_Ent_Bacterial_08_Name;")]
    BacteriumInformem,

    #[serde(rename = "$Codex_Ent_Bacterial_09_Name;")]
    BacteriumVolu,

    #[serde(rename = "$Codex_Ent_Bacterial_10_Name;")]
    BacteriumBullaris,

    #[serde(rename = "$Codex_Ent_Bacterial_11_Name;")]
    BacteriumOmentum,

    #[serde(rename = "$Codex_Ent_Bacterial_12_Name;")]
    BacteriumCerbrus,

    #[serde(rename = "$Codex_Ent_Bacterial_13_Name;")]
    BacteriumVerrata,

    // Brain tree
    // TODO needs to be verified
    #[serde(rename = "$Codex_Ent_Seed_Name;")]
    BrainTreeRoseum,

    // TODO needs to be verified
    #[serde(rename = "$Codex_Ent_SeedABCD_01_Name;")]
    BrainTreeGypseeum,

    // TODO needs to be verified
    #[serde(rename = "$Codex_Ent_SeedABCD_02_Name;")]
    BrainTreeOstrinum,

    // TODO needs to be verified
    #[serde(rename = "$Codex_Ent_SeedABCD_03_Name;")]
    BrainTreeViride,

    // TODO needs to be verified
    #[serde(rename = "$Codex_Ent_SeedEFGH_Name;")]
    BrainTreeLividum,

    // TODO needs to be verified
    #[serde(rename = "$Codex_Ent_SeedEFGH_01_Name;")]
    BrainTreeAureum,

    // TODO needs to be verified
    #[serde(rename = "$Codex_Ent_SeedEFGH_02_Name;")]
    BrainTreePuniceum,

    // TODO needs to be verified
    #[serde(rename = "$Codex_Ent_SeedEFGH_03_Name;")]
    BrainTreeLindigoticum,

    // Cactoida
    #[serde(rename = "$Codex_Ent_Cactoid_01_Name;")]
    CactoidaCortexum,

    #[serde(rename = "$Codex_Ent_Cactoid_02_Name;")]
    CactoidaLapis,

    #[serde(rename = "$Codex_Ent_Cactoid_03_Name;")]
    CactoidaVermis,

    #[serde(rename = "$Codex_Ent_Cactoid_04_Name;")]
    CactoidaPullulanta,

    #[serde(rename = "$Codex_Ent_Cactoid_05_Name;")]
    CactoidaPeperatis,

    // Clypeus
    #[serde(rename = "$Codex_Ent_Clypeus_01_Name;")]
    ClypeusLacrimam,

    #[serde(rename = "$Codex_Ent_Clypeus_02_Name;")]
    ClypeusMargaritus,

    #[serde(rename = "$Codex_Ent_Clypeus_03_Name;")]
    ClypeusSpeculumi,

    // Conchas
    #[serde(rename = "$Codex_Ent_Conchas_01_Name;")]
    ConchaRenibus,

    #[serde(rename = "$Codex_Ent_Conchas_02_Name;")]
    ConchaAureolas,

    #[serde(rename = "$Codex_Ent_Conchas_03_Name;")]
    ConchaLabiata,

    #[serde(rename = "$Codex_Ent_Conchas_04_Name;")]
    ConchaBiconcavis,

    // Crystalline shards
    #[serde(rename = "$Codex_Ent_Ground_Struct_Ice_Name;")]
    CrystallineShards,

    // Electricae
    #[serde(rename = "$Codex_Ent_Electricae_01_Name;")]
    ElectricaePluma,

    #[serde(rename = "$Codex_Ent_Electricae_02_Name;")]
    ElectricaeRadialem,

    // Fonticulus
    #[serde(rename = "$Codex_Ent_Fonticulus_01_Name;")]
    FonticuluaSegmentatus,

    #[serde(rename = "$Codex_Ent_Fonticulus_02_Name;")]
    FonticuluaCampestris,

    #[serde(rename = "$Codex_Ent_Fonticulus_03_Name;")]
    FonticuluaUpupam,

    #[serde(rename = "$Codex_Ent_Fonticulus_04_Name;")]
    FonticuluaLapida,

    #[serde(rename = "$Codex_Ent_Fonticulus_05_Name;")]
    FonticuluaFluctus,

    #[serde(rename = "$Codex_Ent_Fonticulus_06_Name;")]
    FonticuluaDigitos,

    // Frutexa
    #[serde(rename = "$Codex_Ent_Shrubs_01_Name;")]
    FrutexaFlabellum,

    #[serde(rename = "$Codex_Ent_Shrubs_02_Name;")]
    FrutexaAcus,

    #[serde(rename = "$Codex_Ent_Shrubs_03_Name;")]
    FrutexaMetallicum,

    #[serde(rename = "$Codex_Ent_Shrubs_04_Name;")]
    FrutexaFlammasis,

    #[serde(rename = "$Codex_Ent_Shrubs_05_Name;")]
    FrutexaFera,

    #[serde(rename = "$Codex_Ent_Shrubs_06_Name;")]
    FrutexaSponsae,

    #[serde(rename = "$Codex_Ent_Shrubs_07_Name;")]
    FrutexaCollum,

    // Fumerola
    #[serde(rename = "$Codex_Ent_Fumerolas_01_Name;")]
    FumerolaCarbosis,

    #[serde(rename = "$Codex_Ent_Fumerolas_02_Name;")]
    FumerolaExtremus,

    #[serde(rename = "$Codex_Ent_Fumerolas_03_Name;")]
    FumerolaNitris,

    #[serde(rename = "$Codex_Ent_Fumerolas_04_Name;")]
    FumerolaAquatis,

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

    #[serde(rename = "$Codex_Ent_Osseus_02_Name;")]
    OsseusDiscus,

    #[serde(rename = "$Codex_Ent_Osseus_03_Name;")]
    OsseusSpiralis,

    #[serde(rename = "$Codex_Ent_Osseus_04_Name;")]
    OsseusPumice,

    #[serde(rename = "$Codex_Ent_Osseus_05_Name;")]
    OsseusCornibus,

    #[serde(rename = "$Codex_Ent_Osseus_06_Name;")]
    OsseusPellebantus,

    // Recepta
    #[serde(rename = "$Codex_Ent_Recepta_01_Name;")]
    ReceptaUmbrux,

    #[serde(rename = "$Codex_Ent_Recepta_02_Name;")]
    ReceptaDeltahedronix,

    #[serde(rename = "$Codex_Ent_Recepta_03_Name;")]
    ReceptaConditivus,

    // Sinuous tubers
    // TODO needs to be verified
    #[serde(rename = "$Codex_Ent_Tube_Name;")]
    SinuousTubersRoseum,

    // TODO needs to be verified
    #[serde(rename = "$Codex_Ent_TubeABCD_01_Name;")]
    SinuousTubersPrasinum,

    // TODO needs to be verified
    #[serde(rename = "$Codex_Ent_TubeABCD_02_Name;")]
    SinuousTubersAlbidum,

    // TODO needs to be verified
    #[serde(rename = "$Codex_Ent_TubeABCD_03_Name;")]
    SinuousTubersCaeruleum,

    // TODO needs to be verified
    #[serde(rename = "$Codex_Ent_TubeEFGH_Name;")]
    SinuousTubersBlatteum,

    // TODO needs to be verified
    #[serde(rename = "$Codex_Ent_TubeEFGH_01_Name;")]
    SinuousTubersLindigoticum,

    // TODO needs to be verified
    #[serde(rename = "$Codex_Ent_TubeEFGH_02_Name;")]
    SinuousTubersViolaceum,

    // TODO needs to be verified
    #[serde(rename = "$Codex_Ent_TubeEFGH_03_Name;")]
    SinuousTubersViride,

    // Stratum
    #[serde(rename = "$Codex_Ent_Stratum_01_Name;")]
    StratumExcutitus,

    #[serde(rename = "$Codex_Ent_Stratum_02_Name;")]
    StratumPaleas,

    #[serde(rename = "$Codex_Ent_Stratum_03_Name;")]
    StratumLaminamus,

    #[serde(rename = "$Codex_Ent_Stratum_04_Name;")]
    StratumAraneamus,

    #[serde(rename = "$Codex_Ent_Stratum_05_Name;")]
    StratumLimaxus,

    #[serde(rename = "$Codex_Ent_Stratum_06_Name;")]
    StratumCucumisis,

    #[serde(rename = "$Codex_Ent_Stratum_07_Name;")]
    StratumTectonicas,

    #[serde(rename = "$Codex_Ent_Stratum_08_Name;")]
    StratumFrigus,

    // Tubus
    #[serde(rename = "$Codex_Ent_Tubus_01_Name;")]
    TubusConifer,

    #[serde(rename = "$Codex_Ent_Tubus_02_Name;")]
    TubusSororibus,

    #[serde(rename = "$Codex_Ent_Tubus_03_Name;")]
    TubusCavas,

    #[serde(rename = "$Codex_Ent_Tubus_04_Name;")]
    TubusRosarium,

    #[serde(rename = "$Codex_Ent_Tubus_05_Name;")]
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

    #[serde(rename = "$Codex_Ent_Tussocks_05_Name;")]
    TussockCatena,

    #[serde(rename = "$Codex_Ent_Tussocks_06_Name;")]
    TussockPennatis,

    #[serde(rename = "$Codex_Ent_Tussocks_07_Name;")]
    TussockSerrati,

    #[serde(rename = "$Codex_Ent_Tussocks_08_Name;")]
    TussockAlbata,

    #[serde(rename = "$Codex_Ent_Tussocks_09_Name;")]
    TussockPropagito,

    #[serde(rename = "$Codex_Ent_Tussocks_10_Name;")]
    TussockDivisa,

    #[serde(rename = "$Codex_Ent_Tussocks_11_Name;")]
    TussockCaputus,

    #[serde(rename = "$Codex_Ent_Tussocks_12_Name;")]
    TussockTriticum,

    #[serde(rename = "$Codex_Ent_Tussocks_13_Name;")]
    TussockStigmasis,

    #[serde(rename = "$Codex_Ent_Tussocks_14_Name;")]
    TussockVirgam,

    #[serde(rename = "$Codex_Ent_Tussocks_15_Name;")]
    TussockCapillum,

    // Thargoid barnacles
    // TODO needs to be verified
    #[serde(rename = "$Codex_Ent_Thargoid_Barnacle_01_Name;")]
    ThargoidBarnacleCommon,

    // TODO needs to be verified
    #[serde(rename = "$Codex_Ent_Thargoid_Barnacle_02_Name;")]
    ThargoidBarnacleLarge,

    // TODO needs to be verified
    #[serde(rename = "$Codex_Ent_Thargoid_Barnacle_Spikes_Name;")]
    ThargoidBarnacleBarbs,

    // TODO needs to be verified
    #[serde(rename = "$Codex_Ent_Thargoid_Barnacle_Matrix_Submerged_Name;")]
    ThargoidBarnacleMatrixSubmerged,

    // TODO needs to be verified
    #[serde(rename = "$Codex_Ent_Thargoid_Barnacle_Matrix_Name;")]
    ThargoidBarnacleMatrix,
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
            Species::BacteriumInformem => 8_418_00,
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

            Species::CrystallineShards => 0,

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
            Species::ReceptaConditivus => 14_3133_700,

            Species::StratumExcutitus => 2_448_900,
            Species::StratumPaleas => 1_362_00,
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
