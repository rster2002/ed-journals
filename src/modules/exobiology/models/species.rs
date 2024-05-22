use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum::EnumIter;

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
    #[serde(rename = "$Codex_Ent_Sphere_Name;")]
    AnemoneLuteolum,

    #[serde(rename = "$Codex_Ent_SphereABCD_01_Name;")]
    AnemoneCroceum,

    #[serde(rename = "$Codex_Ent_SphereABCD_02_Name;")]
    AnemonePuniceum,

    #[serde(rename = "$Codex_Ent_SphereABCD_03_Name;")]
    AnemoneRoseum,

    #[serde(rename = "$Codex_Ent_SphereEFGH_Name;")]
    AnemoneBlatteumBioluminescent,

    #[serde(rename = "$Codex_Ent_SphereEFGH_01_Name;")]
    AnemoneRubeumBioluminescent,

    #[serde(rename = "$Codex_Ent_SphereEFGH_02_Name;")]
    AnemonePrasinumBioluminescent,

    #[serde(rename = "$Codex_Ent_SphereEFGH_03_Name;")]
    AnemoneRoseumBioluminescent,

    // Bark mounds
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
    BrainTreeAureum,
    BrainTreeOstrinum,
    BrainTreePuniceum,
    BrainTreeLindigoticum,
    BrainTreeGypseeum,
    BrainTreeLividum,
    BrainTreeViride,
    BrainTreeRoseum,

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
    SinuousTubersAlbidum,
    SinuousTubersBlatteum,
    SinuousTubersCaeruleum,
    SinuousTubersLindigoticum,
    SinuousTubersPrasinum,
    SinuousTubersRoseum,
    SinuousTubersViolaceum,
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

                Species::BrainTreeAureum => "Brain Tree Aureum",
                Species::BrainTreeGypseeum => "Brain Tree Gypseeum",
                Species::BrainTreeLindigoticum => "Brain Tree Lindigoticum",
                Species::BrainTreeLividum => "Brain Tree Lividum",
                Species::BrainTreeOstrinum => "Brain Tree Ostrinum",
                Species::BrainTreePuniceum => "Brain Tree Puniceum",
                Species::BrainTreeRoseum => "Brain Tree Roseum",
                Species::BrainTreeViride => "Brain Tree Viride",

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

                Species::SinuousTubersAlbidum => "Sinuous Tubers Albidum",
                Species::SinuousTubersBlatteum => "Sinuous Tubers Blatteum",
                Species::SinuousTubersCaeruleum => "Sinuous Tubers Caeruleum",
                Species::SinuousTubersLindigoticum => "Sinuous Tubers Lindigoticum",
                Species::SinuousTubersPrasinum => "Sinuous Tubers Prasinum",
                Species::SinuousTubersRoseum => "Sinuous Tubers Roseum",
                Species::SinuousTubersViolaceum => "Sinuous Tubers Violaceum",
                Species::SinuousTubersViride => "Sinuous Tubers Viride",

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
