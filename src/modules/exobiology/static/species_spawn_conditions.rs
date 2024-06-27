use std::any;

use lazy_static::lazy_static;

use crate::exobiology::{SpawnCondition, Species};
use crate::galaxy::{
    AtmosphereType, BodyType, PlanetClass, Region, StarClass, StarLuminosity, VolcanismType,
};
use crate::materials::Material;
use AtmosphereType::*;
use PlanetClass::*;
use SpawnCondition::*;
use Species::*;

macro_rules! any {
    ($($x:expr),*$(,)?) => {
        SpawnCondition::Any(vec![$($x),*])
    };
}

macro_rules! all {
    ($($x:expr),*$(,)?) => {
        SpawnCondition::All(vec![$($x),*])
    };
}

lazy_static! {
    #[rustfmt::skip]
    pub static ref SPECIES_SPAWN_CONDITIONS: [(Species, SpawnCondition); 122] = [
        (
            AleoidaArcus,
            all![
                ThinAtmosphere(CarbonDioxide),
                MinGravity(0.04),
                MaxGravity(0.27),
                MinMeanTemperature(175.0),
                MaxMeanTemperature(180.0),
                MinPressure(0.0164),
                any![
                    PlanetClass(RockyBody),
                    PlanetClass(HighMetalContentBody),
                ],
                VolcanismType(VolcanismType::None),
            ]
        ),
        (
            AleoidaCoronamus,
            all![
                ThinAtmosphere(CarbonDioxide),
                MinGravity(0.04),
                MaxGravity(0.27),
                MinMeanTemperature(180.0),
                MaxMeanTemperature(190.0),
                MinPressure(0.025),
                any![
                    PlanetClass(RockyBody),
                    PlanetClass(HighMetalContentBody),
                ],
                VolcanismType(VolcanismType::None),
            ]
        ),
        (
            AleoidaGravis,
            all![
                ThinAtmosphere(CarbonDioxide),
                MinGravity(0.04),
                MaxGravity(0.27),
                MinMeanTemperature(190.0),
                MaxMeanTemperature(196.0),
                MinPressure(0.054),
                any![
                    PlanetClass(RockyBody),
                    PlanetClass(HighMetalContentBody),
                ],
                VolcanismType(VolcanismType::None)
            ]
        ),
        (
            AleoidaLaminiae,
            all![
                ThinAtmosphere(Ammonia),
                MinGravity(0.04),
                MaxGravity(0.27),
                MinMeanTemperature(152.0),
                MaxMeanTemperature(177.0),
                MaxPressure(0.0135),
                any![
                    PlanetClass(RockyBody),
                    PlanetClass(HighMetalContentBody),
                ],
                any![
                    Region(Region::OuterArm),
                    Region(Region::OuterOrionSpur),
                    Region(Region::OuterScutumCentaurusArm),
                    Region(Region::OuterOrionPerseusConflux),
                    Region(Region::PerseusArm),
                    Region(Region::InnerOrionPerseusConflux),
                    Region(Region::InnerScutumCentaurusArm),
                    Region(Region::OuterScutumCentaurusArm),
                ],
            ],
        ),
        (
            AleoidaSpica,
            all![
                ThinAtmosphere(Ammonia),
                MinGravity(0.04),
                MaxGravity(0.276),
                MinMeanTemperature(170.0),
                MaxMeanTemperature(177.0),
                MaxPressure(0.0135),
                any![
                    PlanetClass(RockyBody),
                    PlanetClass(HighMetalContentBody),
                ],
                any![
                    Region(Region::OuterArm),
                    Region(Region::OuterOrionSpur),
                    Region(Region::OuterScutumCentaurusArm),
                    Region(Region::OuterOrionPerseusConflux),
                    Region(Region::PerseusArm),
                    Region(Region::InnerOrionPerseusConflux),
                    Region(Region::InnerScutumCentaurusArm),
                    Region(Region::OuterScutumCentaurusArm),
                ],
            ],
        ),
        (
            AmphoraPlant,
            all![
                ParentStarClass(StarClass::A),
                NoAtmosphere,
                PlanetClass(MetalRichBody),
                any![
                    SystemContainsPlanetClass(EarthlikeBody),
                    SystemContainsPlanetClass(AmmoniaWorld),
                    SystemContainsPlanetClass(GasGiantWithWaterBasedLife,),
                    SystemContainsPlanetClass(GasGiantWithAmmoniaBasedLife,),
                    SystemContainsPlanetClass(WaterGiant),
                ],
            ],
        ),
        (
            AnemonePrasinumBioluminescent,
            all![
                NoAtmosphere,
                MinGravity(0.036),
                MinMeanTemperature(110.0),
                MaxMeanTemperature(3050.0),
                any![
                    VolcanismType(VolcanismType::CarbonDioxideGeysers),
                    VolcanismType(VolcanismType::None),
                ],
                any![
                    PlanetClass(MetalRichBody),
                    PlanetClass(RockyBody),
                    PlanetClass(HighMetalContentBody),
                ],
                any![
                    MainStarClass(StarClass::O),
                    MainStarClass(StarClass::AeBe),
                ],
            ],
        ),
        (
            AnemonePuniceum,
            all![
                NoAtmosphere,
                MinGravity(0.17),
                MaxGravity(2.52),
                MinMeanTemperature(65.0),
                MaxMeanTemperature(800.0),
                any![
                    VolcanismType(VolcanismType::None),
                    VolcanismType(VolcanismType::CarbonDioxideGeysers),
                ],
                any![
                    PlanetClass(IcyBody),
                    PlanetClass(RockyIceBody),
                ],
                ParentStarClass(StarClass::O),
                Region(Region::ArcadianStream),
                // TODO: 'regions': ['anemone-a'] ?? Which region is this?
            ],
        ),
        (
            AnemoneRoseumBioluminescent,
            all![
                NoAtmosphere,
                MinGravity(0.036),
                MaxGravity(4.61),
                MinMeanTemperature(400.0),
                AnyVolcanism,
                any![
                    PlanetClass(MetalRichBody),
                    PlanetClass(HighMetalContentBody),
                ],
                ParentStarClass(StarClass::B),
                any![
                    ParentStarLuminosity(StarLuminosity::I),
                    ParentStarLuminosity(StarLuminosity::II),
                    ParentStarLuminosity(StarLuminosity::III),
                ],
            ],
        ),
        (
            AnemoneRoseum,
            all![
                NoAtmosphere,
                MinGravity(0.045),
                MaxGravity(0.37),
                MinMeanTemperature(200.0),
                MaxMeanTemperature(440.0),
                any![
                    VolcanismType(VolcanismType::SilicateMagma),
                    VolcanismType(VolcanismType::RockyMagma),
                    VolcanismType(VolcanismType::MetallicMagma),
                ],
                PlanetClass(RockyBody),
                all![
                    ParentStarClass(StarClass::B),
                    any![
                        ParentStarLuminosity(StarLuminosity::I),
                        ParentStarLuminosity(StarLuminosity::II),
                        ParentStarLuminosity(StarLuminosity::III),
                        ParentStarLuminosity(StarLuminosity::IV),
                    ],
                ],
                // TODO: 'regions': ['anemone-a'] ?? Which region is this?
            ]
        ),
        (
            AnemoneBlatteumBioluminescent,
            all![
                NoAtmosphere,
                MinMeanTemperature(220.0),
                AnyVolcanism,
                any![
                    PlanetClass(MetalRichBody),
                    PlanetClass(HighMetalContentBody),
                ],
                all![
                    ParentStarClass(StarClass::B),
                    any![
                        ParentStarLuminosity(StarLuminosity::IV),
                        ParentStarLuminosity(StarLuminosity::V),
                    ],
                ],
                // TODO: 'regions': ['anemone-a'] ?? Which region is this?
            ],
        ),
        (
            AnemoneLuteolum,
            all![
                NoAtmosphere,
                MinGravity(0.044),
                MaxGravity(1.28),
                MinMeanTemperature(200.0),
                MaxMeanTemperature(400.0),
                any![
                    VolcanismType(VolcanismType::MetallicMagma),
                    VolcanismType(VolcanismType::SilicateMagma),
                    VolcanismType(VolcanismType::RockyMagma),
                    VolcanismType(VolcanismType::WaterMagma),
                ],
                PlanetClass(RockyBody),
                ParentStarClass(StarClass::B),
                any![
                    ParentStarLuminosity(StarLuminosity::IV),
                    ParentStarLuminosity(StarLuminosity::V),
                ],
                // TODO: 'regions': ['anemone-a'] ?? Which region is this?
            ],
        ),
        (
            AnemoneRubeumBioluminescent,
            all![
                NoAtmosphere,
                MinGravity(0.036),
                MaxGravity(4.61),
                MinMeanTemperature(160.0),
                MaxMeanTemperature(1800.0),
                AnyVolcanism,
                any![
                    PlanetClass(MetalRichBody),
                    PlanetClass(HighMetalContentBody),
                ],
                any![
                    all![
                        ParentStarClass(StarClass::B),
                        ParentStarLuminosity(StarLuminosity::VI),
                    ],
                    all![
                        ParentStarClass(StarClass::A),
                        any![
                            ParentStarLuminosity(StarLuminosity::I),
                            ParentStarLuminosity(StarLuminosity::II),
                            ParentStarLuminosity(StarLuminosity::III),
                        ],
                    ],
                    ParentStarClass(StarClass::N),
                ],
            ],
        ),
        (
            AnemoneCroceum,
            all![
                NoAtmosphere,
                MinGravity(0.047),
                MaxGravity(0.37),
                MinMeanTemperature(200.0),
                MaxMeanTemperature(440.0),
                any![
                    VolcanismType(VolcanismType::SilicateMagma),
                    VolcanismType(VolcanismType::RockyMagma),
                    VolcanismType(VolcanismType::MetallicMagma)
                ],
                PlanetClass(RockyBody),
                any![
                    all![
                        ParentStarClass(StarClass::B),
                        any![
                            ParentStarLuminosity(StarLuminosity::V),
                            ParentStarLuminosity(StarLuminosity::IV),
                        ],
                    ],
                    all![
                        ParentStarClass(StarClass::A),
                        ParentStarLuminosity(StarLuminosity::III),
                    ],
                ],
                // TODO: regions': ['anemone-a'] ?? Which region is this?
            ]
        ),
        (
            BarkMound,
            all![
                NoAtmosphere,
                WithinNebulaRange(150.0),
            ],
        ),
        (
            BacteriumNebulus,
            ThinAtmosphere(Helium),
        ),
        (
            BacteriumAcies,
            any![
                ThinAtmosphere(Neon),
                ThinAtmosphere(NeonRich),
            ],
        ),
        (
            BacteriumOmentum,
            all![
                any![
                    ThinAtmosphere(Neon),
                    ThinAtmosphere(NeonRich),
                ],
                any![
                    VolcanismType(VolcanismType::NitrogenMagma),
                    VolcanismType(VolcanismType::NitrogenGeysers),
                    VolcanismType(VolcanismType::AmmoniaMagma),
                    VolcanismType(VolcanismType::AmmoniaGeysers),
                ],
            ],
        ),
        (
            BacteriumScopulum,
            all![
                any![
                    ThinAtmosphere(Neon),
                    ThinAtmosphere(NeonRich),
                ],
                any![
                    VolcanismType(VolcanismType::CarbonDioxideGeysers),
                    VolcanismType(VolcanismType::MethaneGeysers),
                    VolcanismType(VolcanismType::MethaneMagma),
                ],
            ],
        ),
        (
            BacteriumVerrata,
            all![
                any![
                    ThinAtmosphere(Neon),
                    ThinAtmosphere(NeonRich),
                ],
                any![
                    VolcanismType(VolcanismType::WaterMagma),
                    VolcanismType(VolcanismType::WaterGeysers),
                ],
            ],
        ),
        (
            BacteriumBullaris,
            any![
                ThinAtmosphere(Methane),
                ThinAtmosphere(MethaneRich),
            ],
        ),
        (
            BacteriumAlcyoneum,
            ThinAtmosphere(Ammonia),
        ),
        (
            BacteriumVesicula,
            any![
                ThinAtmosphere(Argon),
                ThinAtmosphere(ArgonRich),
            ],
        ),
        (
            BacteriumCerbrus,
            all![
                MinMeanTemperature(132.0),
                MaxMeanTemperature(500.0),
                any![
                    ThinAtmosphere(SulfurDioxide),
                    ThinAtmosphere(Water),
                ],
                any![
                    all![
                        MinGravity(0.04),
                        MaxGravity(0.06),
                    ],
                    all![
                        MinGravity(0.23),
                        MaxGravity(0.6),
                    ],
                ],
            ],
        ),
        (
            BacteriumAurasus,
            any![
                ThinAtmosphere(CarbonDioxide),
                ThinAtmosphere(CarbonDioxideRich),
            ],
        ),
        (
            BacteriumInformem,
            ThinAtmosphere(Nitrogen),
        ),
        (
            BacteriumVolu,
            ThinAtmosphere(Oxygen),
        ),
        (
            BacteriumTela,
            any![
                all![
                    ThinAtmosphere(Argon),
                    any![
                        PlanetClass(IcyBody),
                        PlanetClass(RockyIceBody),
                    ],
                    MinGravity(0.045),
                    MaxGravity(0.28),
                    MinMeanTemperature(50.0),
                    MaxMeanTemperature(150.0),
                    AnyVolcanism,
                ],
                all![
                    ThinAtmosphere(ArgonRich),
                    MinGravity(0.24),
                    MaxGravity(0.45),
                    MinMeanTemperature(50.0),
                    MaxMeanTemperature(150.0),
                    MaxPressure(0.05),
                    AnyVolcanism,
                ],
                all![
                    ThinAtmosphere(Ammonia),
                    MinGravity(0.025),
                    MaxGravity(0.23),
                    MinMeanTemperature(165.0),
                    MaxMeanTemperature(177.0),
                    MinPressure(0.0025),
                    MaxPressure(0.02),
                    AnyVolcanism,
                ],
                all![
                    ThinAtmosphere(CarbonDioxide),
                    MinGravity(0.45),
                    MaxGravity(0.61),
                    MinMeanTemperature(300.0),
                    MaxMeanTemperature(500.0),
                    MinPressure(0.006),
                    VolcanismType(VolcanismType::None),
                ],
                all![
                    any![
                        ThinAtmosphere(CarbonDioxide),
                        ThinAtmosphere(CarbonDioxideRich),
                    ],
                    MinGravity(0.26),
                    MaxGravity(0.57),
                    MinMeanTemperature(167.0),
                    MaxMeanTemperature(300.0),
                    MinPressure(0.006),
                    AnyVolcanism,
                ],
                all![
                    ThinAtmosphere(Helium),
                    PlanetClass(IcyBody),
                    MinGravity(0.025),
                    MaxGravity(0.61),
                    MinMeanTemperature(20.0),
                    MaxMeanTemperature(21.0),
                    MinPressure(0.067),
                    AnyVolcanism,
                ],
                all![
                    ThinAtmosphere(Methane),
                    any![
                        PlanetClass(IcyBody),
                        PlanetClass(RockyBody),
                        PlanetClass(HighMetalContentBody)
                    ],
                    MinGravity(0.026),
                    MaxGravity(0.126),
                    MinMeanTemperature(80.0),
                    MaxMeanTemperature(109.0),
                    MinPressure(0.012),
                    AnyVolcanism,
                ],
                all![
                    ThinAtmosphere(Neon),
                    any![
                        PlanetClass(IcyBody),
                        PlanetClass(RockyIceBody),
                    ],
                    MinGravity(0.27),
                    MaxGravity(0.61),
                    MinMeanTemperature(20.0),
                    MaxMeanTemperature(95.0),
                    MaxPressure(0.008),
                    AnyVolcanism,
                ],
                all![
                    ThinAtmosphere(NeonRich),
                    any![
                        PlanetClass(IcyBody),
                        PlanetClass(RockyIceBody),
                    ],
                    MinGravity(0.27),
                    MaxGravity(0.61),
                    MinMeanTemperature(20.0),
                    MaxMeanTemperature(95.0),
                    MinPressure(0.003),
                    AnyVolcanism,
                ],
                all![
                    ThinAtmosphere(Nitrogen),
                    MinGravity(0.21),
                    MaxGravity(0.35),
                    MinMeanTemperature(55.0),
                    MaxMeanTemperature(80.0),
                    AnyVolcanism,
                ],
                all![
                    ThinAtmosphere(Oxygen),
                    MinGravity(0.23),
                    MaxGravity(0.5),
                    MinMeanTemperature(150.0),
                    MaxMeanTemperature(240.0),
                    MinPressure(0.01),
                    AnyVolcanism,
                ],
                all![
                    ThinAtmosphere(SulfurDioxide),
                    MinGravity(0.18),
                    MaxGravity(0.61),
                    MinMeanTemperature(148.0),
                    MaxMeanTemperature(500.0),
                    AnyVolcanism,
                ],
                all![
                    ThinAtmosphere(SulfurDioxide),
                    MinGravity(0.18),
                    MaxGravity(0.61),
                    MinMeanTemperature(300.0),
                    MaxMeanTemperature(500.0),
                    VolcanismType(VolcanismType::None),
                ],
                all![
                    ThinAtmosphere(Water),
                    any![
                        PlanetClass(RockyBody),
                        PlanetClass(HighMetalContentBody),
                    ],
                    MinGravity(0.04),
                    MaxGravity(0.063),
                    VolcanismType(VolcanismType::None),
                ],
                all![
                    ThinAtmosphere(WaterRich),
                    any![
                        PlanetClass(IcyBody),
                        PlanetClass(RockyIceBody),
                    ],
                    MinGravity(0.32),
                    MaxGravity(0.44),
                    MinMeanTemperature(240.0),
                    MaxMeanTemperature(330.0),
                    MinPressure(0.01),
                    AnyVolcanism,
                ]
            ]
        ),
        (
            BrainTreeAureum,
            all![
                NoAtmosphere,
                AnyVolcanism,
            ],
        ),
        (
            BrainTreeOstrinum,
            all![
                NoAtmosphere,
                AnyVolcanism,
            ],
        ),
        (
            BrainTreePuniceum,
            all![
                NoAtmosphere,
                AnyVolcanism,
            ],
        ),
        (
            BrainTreeLindigoticum,
            all![
                NoAtmosphere,
                AnyVolcanism,
            ],
        ),
        (
            BrainTreeGypseeum,
            all![
                NoAtmosphere,
                AnyVolcanism,
            ],
        ),
        (
            BrainTreeLividum,
            all![
                NoAtmosphere,
                AnyVolcanism,
            ],
        ),
        (
            BrainTreeViride,
            all![
                NoAtmosphere,
                AnyVolcanism,
            ],
        ),
        (
            BrainTreeRoseum,
            all![
                NoAtmosphere,
                AnyVolcanism,
            ],
        ),
        (
            CactoidaLapis,
            all![
                MaxGravity(0.27),
                any![
                    PlanetClass(RockyBody),
                    PlanetClass(HighMetalContentBody),
                ],
                ThinAtmosphere(Ammonia),
            ],
        ),
        (
            CactoidaPullulanta,
            all![
                MaxGravity(0.27),
                any![
                    PlanetClass(RockyBody),
                    PlanetClass(HighMetalContentBody),
                ],
                any![
                    ThinAtmosphere(CarbonDioxide),
                    ThinAtmosphere(CarbonDioxideRich),
                ],
                MinMeanTemperature(180.0),
                MaxMeanTemperature(195.0),
            ],
        ),
        (
            CactoidaCortexum,
            all![
                MaxGravity(0.27),
                any![
                    PlanetClass(RockyBody),
                    PlanetClass(HighMetalContentBody),
                ],
                any![
                    ThinAtmosphere(CarbonDioxide),
                    ThinAtmosphere(CarbonDioxideRich),
                ],
                MinMeanTemperature(180.0),
                MaxMeanTemperature(195.0),
            ],
        ),
        (
            CactoidaVermis,
            all![
                MaxGravity(0.27),
                any![
                    PlanetClass(RockyBody),
                    PlanetClass(HighMetalContentBody),
                ],
                ThinAtmosphere(Water),
            ],
        ),
        (
            CactoidaPeperatis,
            all![
                MaxGravity(0.27),
                any![
                    PlanetClass(RockyBody),
                    PlanetClass(HighMetalContentBody),
                ],
                ThinAtmosphere(Ammonia),
            ],
        ),
        (
            ClypeusSpeculumi,
            all![
                MaxGravity(0.27),
                MinMeanTemperature(190.0),
                any![
                    ThinAtmosphere(CarbonDioxide),
                    ThinAtmosphere(Water),
                ],
                MinDistanceFromParentSun(5.0),
            ],
        ),
        (
            ClypeusLacrimam,
            all![
                MaxGravity(0.27),
                MinMeanTemperature(190.0),
                any![
                    ThinAtmosphere(CarbonDioxide),
                    ThinAtmosphere(Water),
                ],
            ],
        ),
        (
            ClypeusMargaritus,
            all![
                MaxGravity(0.27),
                MinMeanTemperature(190.0),
                any![
                    ThinAtmosphere(CarbonDioxide),
                    ThinAtmosphere(Water),
                ],
            ],
        ),
        (
            ConchaRenibus,
            any![
                all![
                    MaxGravity(0.27),
                    any![
                        ThinAtmosphere(CarbonDioxide),
                        ThinAtmosphere(CarbonDioxideRich)
                    ],
                    MinMeanTemperature(180.0),
                    MaxMeanTemperature(195.0),
                ],
                all![
                    MaxGravity(0.27),
                    any![
                        ThinAtmosphere(Water),
                        ThinAtmosphere(WaterRich),
                    ],
                ],
            ],
        ),
        (
            ConchaAureolas,
            all![MaxGravity(0.27), ThinAtmosphere(Ammonia)]
        ),
        (
            ConchaLabiata,
            all![
                MaxGravity(0.27),
                any![
                    ThinAtmosphere(CarbonDioxide),
                    ThinAtmosphere(CarbonDioxideRich),
                ],
                MaxMeanTemperature(190.0),
            ],
        ),
        (
            ConchaBiconcavis,
            all![
                MaxGravity(0.27),
                ThinAtmosphere(Nitrogen),
            ],
        ),
        (
            CrystallineShards,
            all![
                NoAtmosphere,
                any![
                    ParentStarClass(StarClass::A),
                    ParentStarClass(StarClass::F),
                    ParentStarClass(StarClass::G),
                    ParentStarClass(StarClass::K),
                    ParentStarClass(StarClass::M),
                    ParentStarClass(StarClass::S),
                ],
                MinMeanTemperature(0.0),
                MaxMeanTemperature(273.0),
                any![
                    SystemContainsPlanetClass(EarthlikeBody),
                    SystemContainsPlanetClass(AmmoniaWorld),
                    SystemContainsPlanetClass(GasGiantWithWaterBasedLife,),
                    SystemContainsPlanetClass(GasGiantWithAmmoniaBasedLife,),
                    SystemContainsPlanetClass(WaterGiant),
                ],
                MinDistanceFromParentSun(24.0),
            ],
        ),
        (
            ElectricaePluma,
            all![
                any![
                    ThinAtmosphere(Helium),
                    ThinAtmosphere(Neon),
                    ThinAtmosphere(Argon),
                ],
                PlanetClass(IcyBody),
                ParentStarClass(StarClass::A),
                any![
                    MinOrEqualParentStarLuminosity(StarLuminosity::V),
                    ParentStarClass(StarClass::N),
                ],
                MaxGravity(0.27),
            ],
        ),
        (
            ElectricaeRadialem,
            all![
                any![
                    ThinAtmosphere(Helium),
                    ThinAtmosphere(Neon),
                    ThinAtmosphere(Argon),
                ],
                PlanetClass(IcyBody),
                WithinNebulaRange(150.0),
                MaxGravity(0.27),
            ],
        ),
        (
            FonticuluaCampestris,
            all![
                ThinAtmosphere(Argon),
                any![
                    PlanetClass(IcyBody),
                    PlanetClass(RockyIceBody),
                ],
                MaxGravity(0.27),
                MinMeanTemperature(50.0),
                MaxMeanTemperature(150.0),
            ],
        ),
        (
            FonticuluaSegmentatus,
            all![
                any![
                    ThinAtmosphere(Neon),
                    ThinAtmosphere(NeonRich)
                ],
                any![
                    PlanetClass(IcyBody),
                    PlanetClass(RockyIceBody),
                ],
                MaxGravity(0.27),
            ],
        ),
        (
            FonticuluaDigitos,
            all![
                any![
                    ThinAtmosphere(Methane),
                    ThinAtmosphere(MethaneRich),
                ],
                any![
                    PlanetClass(IcyBody),
                    PlanetClass(RockyIceBody),
                ],
                MaxGravity(0.27),
            ],
        ),
        (
            FonticuluaUpupam,
            all![
                ThinAtmosphere(ArgonRich),
                any![
                    PlanetClass(IcyBody),
                    PlanetClass(RockyIceBody),
                ],
                MaxGravity(0.27),
            ],
        ),
        (
            FonticuluaLapida,
            all![
                ThinAtmosphere(Nitrogen),
                any![
                    PlanetClass(IcyBody),
                    PlanetClass(RockyIceBody),
                ],
                MaxGravity(0.27),
            ],
        ),
        (
            FonticuluaFluctus,
            all![
                ThinAtmosphere(Oxygen),
                any![
                    PlanetClass(IcyBody),
                    PlanetClass(RockyIceBody),
                ],
                MaxGravity(0.27),
            ],
        ),
        (
            FrutexaAcus,
            all![
                ThinAtmosphere(CarbonDioxide),
                PlanetClass(RockyBody),
                MinGravity(0.04),
                MaxGravity(0.27),
                MinMeanTemperature(147.0),
                MaxMeanTemperature(195.0),
                MinPressure(0.003),
                MaxPressure(0.1),
            ],
        ),
        (
            FrutexaCollum,
            all![
                ThinAtmosphere(SulfurDioxide),
                PlanetClass(RockyBody),
                MaxGravity(0.27),
            ],
        ),
        (
            FrutexaFera,
            all![
                ThinAtmosphere(CarbonDioxide),
                PlanetClass(RockyBody),
                MinGravity(0.04),
                MaxGravity(0.25),
                MinMeanTemperature(145.0),
                MaxMeanTemperature(200.0),
                MinPressure(0.003),
                any![
                    Region(Region::GalacticCenter),
                    Region(Region::EmpyreonStraits),
                    Region(Region::OuterArm),
                    Region(Region::FormidineRift),
                    Region(Region::ArcadianStream),
                    Region(Region::TheConduit),
                    Region(Region::NewtonsVault),
                    Region(Region::NormaArm),
                    Region(Region::KeplersCrest),
                    Region(Region::Xibalba),
                ],
            ],
        ),
        (
            FrutexaFlabellum,
            all![
                ThinAtmosphere(Ammonia),
                PlanetClass(RockyBody),
                MaxGravity(0.27),
            ],
        ),
        (
            FrutexaFlammasis,
            all![
                ThinAtmosphere(Ammonia),
                PlanetClass(RockyBody),
                MaxGravity(0.27),
            ],
        ),
        (
            FrutexaMetallicum,
            all![
                any![
                    all![
                        ThinAtmosphere(CarbonDioxide),
                        MaxMeanTemperature(195.0),
                    ],
                    ThinAtmosphere(Ammonia)
                ],
                PlanetClass(HighMetalContentBody),
                MaxGravity(0.27),
            ],
        ),
        (
            FrutexaSponsae,
            all![
                ThinAtmosphere(Water),
                PlanetClass(RockyBody),
                MaxGravity(0.27),
            ],
        ),
        (
            FumerolaAquatis,
            all![
                AnyThinAtmosphere,
                GeologicalSignalsPresent,
                any![
                    VolcanismType(VolcanismType::WaterMagma),
                    VolcanismType(VolcanismType::WaterGeysers),
                ],
                MaxGravity(0.27),
            ],
        ),
        (
            FumerolaCarbosis,
            all![
                AnyThinAtmosphere,
                GeologicalSignalsPresent,
                any![
                    VolcanismType(VolcanismType::CarbonDioxideGeysers),
                    VolcanismType(VolcanismType::MethaneMagma),
                    VolcanismType(VolcanismType::MethaneGeysers),
                ],
                MaxGravity(0.27),
            ],
        ),
        (
            FumerolaExtremus,
            all![
                AnyThinAtmosphere,
                GeologicalSignalsPresent,
                any![
                    MaterialPresence(Material::Iron),
                    VolcanismType(VolcanismType::CarbonDioxideGeysers),
                    VolcanismType(VolcanismType::MethaneMagma),
                    VolcanismType(VolcanismType::MethaneGeysers),
                ],
                any![
                    PlanetClass(RockyBody),
                    PlanetClass(HighMetalContentBody),
                ],
                MaxGravity(0.27),
            ],
        ),
        (
            FumerolaNitris,
            all![
                AnyThinAtmosphere,
                GeologicalSignalsPresent,
                any![
                    VolcanismType(VolcanismType::NitrogenMagma),
                    VolcanismType(VolcanismType::NitrogenGeysers),
                    VolcanismType(VolcanismType::AmmoniaMagma),
                    VolcanismType(VolcanismType::AmmoniaGeysers),
                ],
                MaxGravity(0.27),
            ],
        ),
        (
            FungoidaBullarum,
            all![
                any![
                    ThinAtmosphere(Argon),
                    ThinAtmosphere(ArgonRich),
                    ThinAtmosphere(Nitrogen),
                ],
                any![
                    PlanetClass(RockyIceBody),
                    PlanetClass(RockyBody),
                    PlanetClass(HighMetalContentBody),
                ],
                MinMeanTemperature(50.0),
                MaxMeanTemperature(135.0),
                MaxPressure(0.1),
                MaxGravity(0.27),
            ],
        ),
        (
            FungoidaGelata,
            all![
                any![
                    ThinAtmosphere(Water),
                    all![
                        ThinAtmosphere(CarbonDioxide),
                        MinMeanTemperature(180.0),
                        MaxMeanTemperature(195.0),
                    ],
                ],
                MaxGravity(0.27),
            ],
        ),
        (
            FungoidaSetisis,
            all![
                any![
                    ThinAtmosphere(Ammonia),
                    ThinAtmosphere(Methane),
                ],
                MaxGravity(0.27),
            ],
        ),
        (
            FungoidaStabitis,
            all![
                any![
                    ThinAtmosphere(Water),
                    all![
                        ThinAtmosphere(CarbonDioxide),
                        MinMeanTemperature(180.0),
                        MaxMeanTemperature(195.0),
                    ],
                ],
                MaxGravity(0.27),
            ],
        ),
        (
            OsseusCornibus,
            all![
                ThinAtmosphere(CarbonDioxide),
                any![
                    PlanetClass(RockyBody),
                    PlanetClass(HighMetalContentBody),
                ],
                MaxGravity(0.27),
                MinMeanTemperature(180.0),
                MaxMeanTemperature(195.0),
            ],
        ),
        (
            OsseusDiscus,
            all![
                ThinAtmosphere(Water),
                any![
                    PlanetClass(RockyBody),
                    PlanetClass(HighMetalContentBody),
                ],
                MaxGravity(0.27),
            ],
        ),
        (
            OsseusFractus,
            all![
                ThinAtmosphere(CarbonDioxide),
                any![
                    PlanetClass(RockyBody),
                    PlanetClass(HighMetalContentBody),
                ],
                MaxGravity(0.27),
                MinMeanTemperature(180.0),
                MaxMeanTemperature(195.0),
            ],
        ),
        (
            OsseusPellebantus,
            all![
                ThinAtmosphere(CarbonDioxide),
                any![
                    PlanetClass(RockyBody),
                    PlanetClass(HighMetalContentBody),
                ],
                MaxGravity(0.27),
                MinMeanTemperature(180.0),
                MaxMeanTemperature(195.0),
            ],
        ),
        (
            OsseusPumice,
            all![
                any![
                    ThinAtmosphere(Argon),
                    ThinAtmosphere(ArgonRich),
                    ThinAtmosphere(Methane),
                    ThinAtmosphere(Nitrogen),
                ],
                any![
                    PlanetClass(RockyIceBody),
                    PlanetClass(HighMetalContentBody),
                    PlanetClass(RockyBody),
                ],
                any![
                    PlanetClass(RockyBody),
                    PlanetClass(HighMetalContentBody),
                    PlanetClass(RockyIceBody),
                ],
                MaxGravity(0.27),
                MinMeanTemperature(43.0),
                MaxMeanTemperature(135.0),
            ],
        ),
        (
            OsseusSpiralis,
            all![
                ThinAtmosphere(Ammonia),
                any![
                    PlanetClass(RockyBody),
                    PlanetClass(HighMetalContentBody),
                ],
                MaxGravity(0.27)
            ]
        ),
        (
            ReceptaConditivus,
            all![
                ThinAtmosphere(SulfurDioxide),
                any![
                    PlanetClass(IcyBody),
                    PlanetClass(RockyIceBody),
                ],
                MaxGravity(0.27),
            ],
        ),
        (
            ReceptaDeltahedronix,
            all![
                ThinAtmosphere(SulfurDioxide),
                any![
                    PlanetClass(RockyBody),
                    PlanetClass(HighMetalContentBody),
                ],
                MaxGravity(0.27),
            ],
        ),
        (
            ReceptaUmbrux,
            all![
                ThinAtmosphere(SulfurDioxide),
                MaxGravity(0.27),
            ],
        ),
        (
            SinuousTubersAlbidum,
            all![
                NoAtmosphere,
                PlanetClass(RockyBody),
            ]),
        (
            SinuousTubersBlatteum,
            all![
                NoAtmosphere,
                any![
                    PlanetClass(MetalRichBody),
                    PlanetClass(HighMetalContentBody),
                ],
            ],
        ),
        (
            SinuousTubersCaeruleum,
            all![
                NoAtmosphere,
                PlanetClass(RockyBody),
            ],
        ),
        (
            SinuousTubersLindigoticum,
            all![
                NoAtmosphere,
                PlanetClass(RockyBody),
            ],
        ),
        (
            SinuousTubersPrasinum,
            all![
                NoAtmosphere,
                any![
                    PlanetClass(MetalRichBody),
                    PlanetClass(HighMetalContentBody),
                ],
            ],
        ),
        (
            SinuousTubersRoseum,
            all![
                NoAtmosphere,
                VolcanismType(VolcanismType::SilicateMagma),
            ],
        ),
        (
            SinuousTubersViolaceum,
            all![
                NoAtmosphere,
                any![
                    PlanetClass(MetalRichBody),
                    PlanetClass(HighMetalContentBody),
                ],
            ],
        ),
        (
            SinuousTubersViride,
            all![
                NoAtmosphere,
                any![
                    PlanetClass(MetalRichBody),
                    PlanetClass(HighMetalContentBody),
                ],
            ],
        ),
        (
            StratumAraneamus,
            all![
                ThinAtmosphere(SulfurDioxide),
                PlanetClass(RockyBody),
                MinMeanTemperature(165.0),
                MaxMeanTemperature(375.0),
                MinGravity(0.26),
                MinGravity(0.55),
            ],
        ),
        (
            StratumCucumisis,
            all![
                any![
                    ThinAtmosphere(SulfurDioxide),
                    ThinAtmosphere(CarbonDioxide),
                    ThinAtmosphere(CarbonDioxideRich),
                ],
                PlanetClass(RockyBody),
                MinMeanTemperature(190.0),
                MaxMeanTemperature(375.0),
                MinGravity(0.04),
                MaxGravity(0.6),
            ],
        ),
        (
            StratumExcutitus,
            all![
                any![
                    ThinAtmosphere(Oxygen),
                    ThinAtmosphere(SulfurDioxide),
                    ThinAtmosphere(CarbonDioxide),
                ],
                any![
                    Region(Region::GalacticCenter),
                    Region(Region::OdinsHold),
                    Region(Region::InnerOrionSpur),
                    Region(Region::OuterOrionSpur),
                    Region(Region::OrionCygnusArm),
                    Region(Region::Izanami),
                    Region(Region::Temple),
                    Region(Region::InnerOrionPerseusConflux),
                ],
                PlanetClass(RockyBody),
                MinGravity(0.04),
                MaxGravity(0.48),
                MinMeanTemperature(165.0),
                MaxMeanTemperature(190.0),
            ],
        ),
        (
            StratumFrigus,
            all![
                any![
                    ThinAtmosphere(SulfurDioxide),
                    ThinAtmosphere(CarbonDioxide),
                    ThinAtmosphere(CarbonDioxideRich),
                ],
                PlanetClass(RockyBody),
                MinMeanTemperature(190.0),
                MinGravity(0.04),
                MaxGravity(0.551),
            ],
        ),
        (
            StratumLaminamus,
            all![
                ThinAtmosphere(Ammonia),
                PlanetClass(RockyBody),
                MinMeanTemperature(165.0),
            ],
        ),
        (
            StratumLimaxus,
            all![
                any![
                    ThinAtmosphere(SulfurDioxide),
                    ThinAtmosphere(CarbonDioxide),
                ],
                PlanetClass(RockyBody),
                MinMeanTemperature(165.0),
                MaxMeanTemperature(190.0),
            ],
        ),
        (
            StratumPaleas,
            all![
                any![
                    ThinAtmosphere(Ammonia),
                    ThinAtmosphere(Water),
                    ThinAtmosphere(Oxygen),
                    ThinAtmosphere(CarbonDioxide),
                    ThinAtmosphere(CarbonDioxideRich),
                ],
                PlanetClass(RockyBody),
                MinMeanTemperature(165.0),
                MaxMeanTemperature(450.0),
                MinGravity(0.039),
                MaxGravity(0.6),
            ],
        ),
        (
            StratumTectonicas,
            all![
                AnyThinAtmosphere,
                PlanetClass(HighMetalContentBody),
                MinMeanTemperature(165.0),
            ],
        ),
        (
            TubusCavas,
            all![
                ThinAtmosphere(CarbonDioxide),
                PlanetClass(RockyBody),
                MaxGravity(0.15),
                MinMeanTemperature(160.0),
                MaxMeanTemperature(190.0),
            ],
        ),
        (
            TubusCompagibus,
            all![
                ThinAtmosphere(CarbonDioxide),
                PlanetClass(RockyBody),
                MaxGravity(0.15),
                MinMeanTemperature(160.0),
                MaxMeanTemperature(190.0),
            ],
        ),
        (
            TubusConifer,
            all![
                ThinAtmosphere(CarbonDioxide),
                PlanetClass(RockyBody),
                MinGravity(0.041),
                MaxGravity(0.15),
                MinMeanTemperature(160.0),
                MaxMeanTemperature(190.0),
                VolcanismType(VolcanismType::None),
                MinPressure(0.003),
            ],
        ),
        (
            TubusRosarium,
            all![
                ThinAtmosphere(Ammonia),
                PlanetClass(RockyBody),
                MaxGravity(0.15),
                MinMeanTemperature(160.0),
            ],
        ),
        (
            TubusSororibus,
            all![
                any![
                    ThinAtmosphere(CarbonDioxide),
                    ThinAtmosphere(Ammonia),
                ],
                PlanetClass(HighMetalContentBody),
                MaxGravity(0.15),
                MinMeanTemperature(160.0),
                MaxMeanTemperature(190.0),
            ],
        ),
        (
            TussockAlbata,
            all![
                ThinAtmosphere(CarbonDioxide),
                PlanetClass(RockyBody),
                MaxGravity(0.27),
                MinMeanTemperature(175.0),
                MaxMeanTemperature(180.0),
            ],
        ),
        (
            TussockCapillum,
            all![
                any![
                    PlanetClass(RockyBody),
                    PlanetClass(RockyIceBody),
                ],
                any![
                    ThinAtmosphere(Argon),
                    ThinAtmosphere(Methane),
                ],
                MaxGravity(0.27),
                MinMeanTemperature(80.0),
                MaxMeanTemperature(130.0),
            ],
        ),
        (
            TussockCaputus,
            all![
                ThinAtmosphere(CarbonDioxide),
                PlanetClass(RockyBody),
                MaxGravity(0.27),
                MinMeanTemperature(180.0),
                MaxMeanTemperature(190.0),
            ],
        ),
        (
            TussockCatena,
            all![
                ThinAtmosphere(Ammonia),
                PlanetClass(RockyBody),
                MaxGravity(0.27),
            ],
        ),
        (
            TussockCultro,
            all![
                ThinAtmosphere(Ammonia),
                PlanetClass(RockyBody),
                MaxGravity(0.27),
            ],
        ),
        (
            TussockDivisa,
            all![
                ThinAtmosphere(Ammonia),
                PlanetClass(RockyBody),
                MaxGravity(0.27),
            ],
        ),
        (
            TussockIgnis,
            all![
                ThinAtmosphere(CarbonDioxide),
                PlanetClass(RockyBody),
                MaxGravity(0.27),
                MinMeanTemperature(160.0),
                MaxMeanTemperature(170.0),
            ],
        ),
        (
            TussockPennata,
            all![
                ThinAtmosphere(CarbonDioxide),
                PlanetClass(RockyBody),
                MaxGravity(0.27),
                MinMeanTemperature(145.0),
                MaxMeanTemperature(155.0),
            ],
        ),
        (
            TussockPennatis,
            all![
                ThinAtmosphere(CarbonDioxide),
                PlanetClass(RockyBody),
                MaxGravity(0.27),
                MaxMeanTemperature(195.0),
            ],
        ),
        (
            TussockPropagito,
            all![
                ThinAtmosphere(CarbonDioxide),
                PlanetClass(RockyBody),
                MaxGravity(0.27),
                MaxMeanTemperature(195.0),
            ],
        ),
        (
            TussockSerrati,
            all![
                ThinAtmosphere(CarbonDioxide),
                PlanetClass(RockyBody),
                MaxGravity(0.27),
                MinMeanTemperature(170.0),
                MaxMeanTemperature(175.0),
            ],
        ),
        (
            TussockStigmasis,
            all![
                ThinAtmosphere(SulfurDioxide),
                PlanetClass(RockyBody),
                MaxGravity(0.27),
            ],
        ),
        (
            TussockTriticum,
            all![
                ThinAtmosphere(CarbonDioxide),
                PlanetClass(RockyBody),
                MaxGravity(0.27),
                MinMeanTemperature(190.0),
                MaxMeanTemperature(195.0),
            ],
        ),
        (
            TussockVentusa,
            all![
                ThinAtmosphere(CarbonDioxide),
                PlanetClass(RockyBody),
                MaxGravity(0.27),
                MinMeanTemperature(155.0),
                MaxMeanTemperature(160.0),
            ],
        ),
        (
            TussockVirgam,
            all![
                ThinAtmosphere(Water),
                PlanetClass(RockyBody),
                MaxGravity(0.27),
            ],
        ),
        (
            ThargoidBarnacleCommon,
            Special,
        ),
        (
            ThargoidBarnacleLarge,
            Special,
        ),
        (
            ThargoidBarnacleBarbs,
            Special,
        ),
        (
            ThargoidBarnacleMatrixSubmerged,
            Special,
        ),
        (
            ThargoidBarnacleMatrix,
            Special,
        )
    ];
}

#[cfg(test)]
mod tests {
    use std::env::current_dir;
    use std::fs::File;
    use crate::exobiology::{SpawnCondition, Species};
    use crate::galaxy::{Atmosphere, AtmosphereDensity, AtmosphereType, BodyType, Gravity, PlanetClass, Region, Volcanism, VolcanismClassification, VolcanismType};

    const ALL_CSV_FILES: &[&str] = &[
        "bacterium-cerbrus.csv",
        "cactoida-lapis.csv",
        "fonticulua-campestris.csv",
        "frutexa-acus.csv",
        "frutexa-fabellum.csv",
        "frutexa-fera.csv",
        "fungoida-bullarum.csv",
        "osseus-pumice.csv",
        "stratum-cucumisis.csv",
        "stratum-excutitus.csv",
        "stratum-frigus.csv",
        "stratum-paleas.csv",
        "tussock-capillum.csv",
    ];

    #[derive(Debug)]
    struct PlanetDetails {
        pub name: String,
        pub gravity: Gravity,
        pub temperature: f32,
        pub volcanism: Volcanism,
        pub pressure: f32,
        pub atmosphere: Atmosphere,
        pub planet_class: PlanetClass,
        pub region: Region,
    }

    fn get_test_entries(file_name: &str) -> Vec<PlanetDetails> {
        let file_path = current_dir()
            .unwrap()
            .join("test-files")
            .join("species-planets")
            .join(file_name);

        let file = File::open(file_path)
            .unwrap();

        let mut csv_reader = csv::Reader::from_reader(file);

        csv_reader.records()
            .into_iter()
            .filter_map(|record| {
                match record {
                    Ok(record) => Some(PlanetDetails {
                        name: record.get(10)
                            .unwrap()
                            .to_string(),
                        temperature: record.get(16)?
                            .parse()
                            .ok()?,
                        gravity: Gravity::from_g(
                            record.get(15)?
                                .parse::<f32>()
                                .ok()?
                        ),
                        volcanism: match record.get(14)? {
                            "No volcanism" => Volcanism {
                                kind: VolcanismType::None,
                                classification: VolcanismClassification::Minor,
                            },
                            volcanism => volcanism.parse()
                                .unwrap()
                        },
                        pressure: record.get(13)?
                            .parse()
                            .ok()?,
                        atmosphere: match record.get(12)? {
                            "No atmosphere" => Atmosphere {
                                hot: false,
                                density: AtmosphereDensity::Normal,
                                kind: AtmosphereType::None,
                            },
                            atmosphere => atmosphere.parse()
                                .unwrap(),
                        },
                        planet_class: record.get(11)?
                            .parse()
                            .unwrap(),
                        region: Region::from_name(record.get(5)?),
                    }),
                    Err(_) => None
                }
            })
            .collect()
    }

    /// Tests the target species with the given csv file by checking missed predictions and false
    /// positives.
    fn test_species_planet_details(species: Species, file_name: &str) {
        success_species_planet_details(&species, file_name);
        fail_species_planet_details(&species, file_name);
    }

    /// Checks that all the spawn conditions apply to the specified test file and ensures that the
    /// number of failed predictions stay under a certain percentage.
    fn success_species_planet_details(species: &Species, file_name: &str) {
        let test_cases = get_test_entries(file_name);

        let spawn_conditions = species.spawn_conditions();
        let mut failed_cases = Vec::new();
        let mut entry_count: usize = 0;

        for case in test_cases {
            entry_count += 1;

            if !check_spawn_condition(&spawn_conditions, &case) {
                failed_cases.push(case);
            }
        }

        let ratio = failed_cases.len() as f32 / entry_count as f32;
        dbg!(&ratio);

        // 0.5% of cases are allowed to fail
        if ratio >= 0.005 {
            dbg!(failed_cases.get(0));
            assert!(false);
        }
    }

    /// Checks that the spawn conditions do not apply to all other entries in the test-suite and
    /// ensures that the number of false positives stay under a said percentage.
    fn fail_species_planet_details(species: &Species, file_name: &str) {
        let spawn_conditions = species.spawn_conditions();
        let mut succeeded = Vec::new();
        let mut entry_count: usize = 0;

        for file in ALL_CSV_FILES {
            if file == &file_name {
                continue;
            }

            let test_cases = get_test_entries(file);

            for case in test_cases {
                entry_count += 1;

                if check_spawn_condition(&spawn_conditions, &case) {
                    succeeded.push(case);
                }
            }
        }

        let ratio = succeeded.len() as f32 / entry_count as f32;
        dbg!(&ratio);

        // TODO see how this can be decreased. To do this this probably needs to take into account
        //  which species commonly overlap
        // 20% of cases are allowed to succeed
        if ratio >= 0.20 {
            dbg!(succeeded.get(0));
            assert!(false);
        }
    }

    fn check_spawn_condition(spawn_condition: &SpawnCondition, planet_details: &PlanetDetails) -> bool {
        match spawn_condition {
            SpawnCondition::MinMeanTemperature(min) => {
                &planet_details.temperature >= min
            }
            SpawnCondition::MaxMeanTemperature(max) => {
                &planet_details.temperature <= max
            }
            SpawnCondition::NoAtmosphere => {
                matches!(planet_details.atmosphere.kind, AtmosphereType::None)
            }
            SpawnCondition::AnyThinAtmosphere => {
                matches!(planet_details.atmosphere.density, AtmosphereDensity::Thin)
            }
            SpawnCondition::ThinAtmosphere(atmosphere) => {
                &planet_details.atmosphere.kind == atmosphere
            }
            SpawnCondition::MinGravity(min_gravity) => {
                &planet_details.gravity.as_g() >= min_gravity
            }
            SpawnCondition::MaxGravity(max_gravity) => {
                &planet_details.gravity.as_g() <= max_gravity
            }
            SpawnCondition::PlanetClass(planet_class) => {
                &planet_details.planet_class == planet_class
            }
            // TODO
            SpawnCondition::MainStarClass(_) => true,
            // TODO
            SpawnCondition::ParentStarClass(_) => true,
            SpawnCondition::VolcanismType(volcanism) => {
                &planet_details.volcanism.kind == volcanism
            }
            SpawnCondition::AnyVolcanism => {
                &planet_details.volcanism.kind == &VolcanismType::None
            }
            SpawnCondition::MinPressure(min_pressure) => {
                &planet_details.pressure >= min_pressure
            }
            SpawnCondition::MaxPressure(max_pressure) => {
                &planet_details.pressure <= max_pressure
            }
            SpawnCondition::Region(region) => {
                &planet_details.region == region
            }
            SpawnCondition::Special => false,
            SpawnCondition::Any(conditions) => {
                conditions.iter()
                    .any(|condition| check_spawn_condition(condition, planet_details))
            }
            SpawnCondition::All(conditions) => {
                conditions.iter()
                    .all(|condition| check_spawn_condition(condition, planet_details))
            }
            _ => true,
        }
    }

    // TODO also test against large random set of planets and ensure that only a certain percentage
    //  passes to combat false positives.

    #[test]
    fn bacterium_cerbrus_test_cases_all_pass() {
        test_species_planet_details(Species::BacteriumCerbrus, "bacterium-cerbrus.csv");
    }

    #[test]
    fn cactoida_lapis_test_cases_all_pass() {
        test_species_planet_details(Species::CactoidaLapis, "cactoida-lapis.csv");
    }

    #[test]
    fn fonticulua_campestris_test_cases_all_pass() {
        test_species_planet_details(Species::FonticuluaCampestris, "fonticulua-campestris.csv");
    }

    #[test]
    fn fungoida_ballarum_test_cases_all_pass() {
        test_species_planet_details(Species::FungoidaBullarum, "fungoida-bullarum.csv");
    }

    #[test]
    fn osseus_pumice_test_cases_all_pass() {
        test_species_planet_details(Species::OsseusPumice, "osseus-pumice.csv");
    }

    #[test]
    fn tussock_capillum_test_cases_all_pass() {
        test_species_planet_details(Species::TussockCapillum, "tussock-capillum.csv");
    }

    #[test]
    fn stratum_cucumisis_test_cases_all_pass() {
        test_species_planet_details(Species::StratumCucumisis, "stratum-cucumisis.csv");
    }

    #[test]
    fn stratum_frigus_test_cases_all_pass() {
        test_species_planet_details(Species::StratumFrigus, "stratum-frigus.csv");
    }

    #[test]
    fn frutexa_acus_test_cases_all_pass() {
        test_species_planet_details(Species::FrutexaAcus, "frutexa-acus.csv");
    }

    #[test]
    fn frutexa_fabellum_test_cases_all_pass() {
        test_species_planet_details(Species::FrutexaFlabellum, "frutexa-fabellum.csv");
    }

    #[test]
    fn frutexa_fera_test_cases_all_pass() {
        test_species_planet_details(Species::FrutexaFera, "frutexa-fera.csv");
    }

    #[test]
    fn stratum_excutitus_test_cases_all_pass() {
        test_species_planet_details(Species::StratumExcutitus, "stratum-excutitus.csv");
    }

    #[test]
    fn stratum_paleas_test_cases_all_pass() {
        test_species_planet_details(Species::StratumPaleas, "stratum-paleas.csv");
    }
}
