use std::any;

use lazy_static::lazy_static;

use crate::exobiology::{SpawnCondition, Species};
use crate::galaxy::{
    AtmosphereType, PlanetClass, Region, StarClass, StarLuminosity, VolcanismType,
};
use crate::materials::Material;
use AtmosphereType::*;
use PlanetClass::*;
use SpawnCondition::*;
use Species::*;
use VolcanismType::*;

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
                MaxGravity(0.276),
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
                MaxGravity(0.276),
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
                MaxGravity(0.276),
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
                MaxGravity(0.276),
                MinMeanTemperature(152.0),
                MaxMeanTemperature(177.0),
                MaxPressure(0.0135),
                any![
                    PlanetClass(RockyBody),
                    PlanetClass(HighMetalContentBody),
                ],
                any![
                    Region(Region::Acheron),
                    Region(Region::DrymansPoint),
                    Region(Region::GalacticCenter),
                    Region(Region::HawkingsGap),
                    Region(Region::InnerOrionPerseusConflux),
                    Region(Region::InnerOrionSpur),
                    Region(Region::InnerScutumCentaurusArm),
                    Region(Region::Izanami),
                    Region(Region::MareSomnia),
                    Region(Region::OdinsHold),
                    Region(Region::OrionCygnusArm),
                    Region(Region::OuterOrionSpur),
                    Region(Region::SagittariusCarinaArm),
                    Region(Region::Temple),
                    Region(Region::TheAbyss),
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
                any![
                    PlanetClass(RockyBody),
                    PlanetClass(HighMetalContentBody),
                ],
                // any![
                //     Region(Region::OuterArm),
                //     Region(Region::OuterOrionSpur),
                //     Region(Region::OuterScutumCentaurusArm),
                //     Region(Region::OuterOrionPerseusConflux),
                //     Region(Region::PerseusArm),
                //     Region(Region::InnerOrionPerseusConflux),
                //     Region(Region::InnerScutumCentaurusArm),
                //     Region(Region::OuterScutumCentaurusArm),
                // ],
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
                    ThinAtmosphere(Methane),
                    ThinAtmosphere(Argon),
                    ThinAtmosphere(ArgonRich),
                    ThinAtmosphere(WaterRich),
                    ThinAtmosphere(Helium),
                    ThinAtmosphere(Nitrogen),
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
                    ThinAtmosphere(Argon),
                    ThinAtmosphere(CarbonDioxide),
                    ThinAtmosphere(CarbonDioxideRich),
                    ThinAtmosphere(Helium),
                    ThinAtmosphere(Methane),
                    ThinAtmosphere(Neon),
                    ThinAtmosphere(NeonRich),
                    ThinAtmosphere(Nitrogen),
                    ThinAtmosphere(Oxygen),
                ],
                any![
                    VolcanismType(CarbonDioxideGeysers),
                    VolcanismType(MethaneGeysers),
                    VolcanismType(MethaneMagma),
                ],
            ],
        ),
        (
            BacteriumVerrata,
            all![
                any![
                    PlanetClass(IcyBody),
                    PlanetClass(RockyBody),
                    PlanetClass(RockyIceBody),
                ],
                any![
                    VolcanismType(WaterMagma),
                    VolcanismType(WaterGeysers),
                ],
                any![
                    all![
                        any![
                            ThinAtmosphere(Argon),
                            ThinAtmosphere(ArgonRich),
                        ],
                        MinGravity(0.169),
                        MaxGravity(0.36),
                        MinMeanTemperature(59.9),
                        MaxMeanTemperature(145.0),
                    ],
                    all![
                        ThinAtmosphere(Ammonia),
                        MinGravity(0.03),
                        MaxGravity(0.09),
                        MinMeanTemperature(160.0),
                        MaxMeanTemperature(176.0),
                    ],
                    all![
                        ThinAtmosphere(Helium),
                        MinGravity(0.49),
                        MaxGravity(0.51),
                        MinMeanTemperature(20.0),
                        MaxMeanTemperature(21.0),
                        MinPressure(0.068),
                    ],
                    all![
                        ThinAtmosphere(Neon),
                        MinGravity(0.28),
                        MaxGravity(0.61),
                        MinMeanTemperature(20.0),
                        MaxMeanTemperature(51.0),
                        MaxPressure(0.0076),
                    ],
                    all![
                        ThinAtmosphere(NeonRich),
                        MinGravity(0.42),
                        MaxGravity(0.61),
                        MinMeanTemperature(20.0),
                        MaxMeanTemperature(65.0),
                        MinPressure(0.006),
                    ],
                    all![
                        any![
                            ThinAtmosphere(CarbonDioxide),
                            ThinAtmosphere(CarbonDioxideRich),
                        ],
                        MinGravity(0.25),
                        MaxGravity(0.39),
                        MinMeanTemperature(167.0),
                        MaxMeanTemperature(235.0),
                    ],
                    all![
                        ThinAtmosphere(Methane),
                        MinGravity(0.08),
                        MaxGravity(0.1),
                        MinMeanTemperature(95.0),
                        MaxMeanTemperature(100.0),
                    ],
                    all![
                        ThinAtmosphere(Nitrogen),
                        MinGravity(0.21),
                        MaxGravity(0.25),
                        MinMeanTemperature(59.0),
                        MaxMeanTemperature(80.0),
                    ],
                    all![
                        ThinAtmosphere(Oxygen),
                        MinGravity(0.24),
                        MaxGravity(0.38),
                        MinMeanTemperature(155.0),
                        MaxMeanTemperature(220.0),
                        MinPressure(0.019),
                    ],
                    all![
                        ThinAtmosphere(Water),
                        MinGravity(0.04),
                        MaxGravity(0.054),
                        MinMeanTemperature(405.0),
                        MaxMeanTemperature(445.0),
                        MinPressure(0.061),
                    ],
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
                    ThinAtmosphere(SulfurDioxide),
                    MinGravity(0.18),
                    MaxGravity(0.61),
                    MinMeanTemperature(153.0),
                    MaxMeanTemperature(500.0),
                ],
                all![
                    any![
                        ThinAtmosphere(CarbonDioxide),
                        ThinAtmosphere(CarbonDioxideRich),
                    ],
                    MinGravity(0.45),
                    MaxGravity(0.61),
                    MinMeanTemperature(300.0),
                    MaxMeanTemperature(500.0),
                    MinPressure(0.006),
                    any![
                        VolcanismType(VolcanismType::None),
                        VolcanismType(SilicateVapourGeysers),
                    ],
                ],
                all![
                    ThinAtmosphere(Water),
                    any![
                        PlanetClass(RockyBody),
                        PlanetClass(HighMetalContentBody),
                    ],
                    MinGravity(0.04),
                    MaxGravity(0.063),
                    MinMeanTemperature(395.0),
                    MaxMeanTemperature(450.0),
                    any![
                        VolcanismType(VolcanismType::None),
                        VolcanismType(WaterMagma),
                    ],
                ],
                all![
                    ThinAtmosphere(WaterRich),
                    any![
                        PlanetClass(IcyBody),
                        PlanetClass(RockyIceBody),
                    ],
                    MinGravity(0.32),
                    MaxGravity(0.44),
                    MinMeanTemperature(220.0),
                    MaxMeanTemperature(330.0),
                    MinPressure(0.01),
                    AnyVolcanism,
                ],
                all![
                    ThinAtmosphere(Argon),
                    any![
                        PlanetClass(IcyBody),
                        PlanetClass(RockyIceBody),
                    ],
                    MinGravity(0.045),
                    MaxGravity(0.28),
                    MinMeanTemperature(50.0),
                    MaxMeanTemperature(170.0),
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
                    MinGravity(0.03),
                    MaxGravity(0.09),
                    MinMeanTemperature(160.0),
                    MaxMeanTemperature(177.0),
                    MinPressure(0.002),
                    MaxPressure(0.02),
                    AnyVolcanism,
                ],
                all![
                    ThinAtmosphere(Helium),
                    PlanetClass(IcyBody),
                    MinGravity(0.49),
                    MaxGravity(0.53),
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
                    MinMeanTemperature(77.0),
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
                    MaxMeanTemperature(61.0),
                    MinPressure(0.001),
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
                    MinPressure(0.002),
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
                    MaxGravity(0.52),
                    MinMeanTemperature(150.0),
                    MaxMeanTemperature(240.0),
                    MinPressure(0.017),
                    AnyVolcanism,
                ],
                all![
                    ThinAtmosphere(SulfurDioxide),
                    any![
                        PlanetClass(RockyBody),
                        PlanetClass(HighMetalContentBody),
                    ],
                    MinGravity(0.18),
                    MaxGravity(0.61),
                    MinMeanTemperature(149.0),
                    MaxMeanTemperature(500.0),
                    MinPressure(0.001),
                    VolcanismType(VolcanismType::None),
                ],
            ],
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
                any![
                    PlanetClass(RockyBody),
                    PlanetClass(HighMetalContentBody),
                ],
                any![
                    Region(Region::InnerOrionSpur),
                    Region(Region::InnerScutumCentaurusArm),
                    Region(Region::GalacticCenter),
                    Region(Region::HawkingsGap),
                    Region(Region::MareSomnia),
                    Region(Region::DrymansPoint),
                    Region(Region::SagittariusCarinaArm),
                    Region(Region::OdinsHold),
                    Region(Region::TheAbyss),
                    Region(Region::Acheron),
                ],
                MaxGravity(0.28),
                MinMeanTemperature(160.0),
                MaxMeanTemperature(180.0),
                MaxPressure(0.014),
                ThinAtmosphere(Ammonia),
            ],
        ),
        (
            CactoidaPullulanta,
            all![
                any![
                    PlanetClass(IcyBody),
                    PlanetClass(RockyBody),
                    PlanetClass(HighMetalContentBody),
                ],
                ThinAtmosphere(CarbonDioxide),
                MaxGravity(0.27),
                MinMeanTemperature(180.0),
                MaxMeanTemperature(196.0),
                MinPressure(0.025),
            ],
        ),
        (
            CactoidaCortexum,
            all![
                any![
                    PlanetClass(RockyBody),
                    PlanetClass(HighMetalContentBody),
                ],
                any![
                    ThinAtmosphere(CarbonDioxide),
                    ThinAtmosphere(Ammonia),
                ],
                MaxGravity(0.276),
                MinMeanTemperature(180.0),
                MaxMeanTemperature(196.0),
                MinPressure(0.025),
            ],
        ),
        (
            CactoidaVermis,
            all![
                MaxGravity(0.276),
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
                any![
                    PlanetClass(RockyBody),
                    PlanetClass(HighMetalContentBody),
                ],
                any![
                    Region(Region::NormaExpanse),
                    Region(Region::TheVoid),
                    Region(Region::GalacticCenter),
                    Region(Region::TrojanBelt),
                    Region(Region::InnerScutumCentaurusArm),
                    Region(Region::OuterScutumCentaurusArm),
                    Region(Region::OdinsHold),
                    Region(Region::TheVeils),
                    Region(Region::AquilasHalo),
                    Region(Region::HieronymusDelta),
                    Region(Region::FormorianFrontier),
                ],
                MaxGravity(0.28),
                MinMeanTemperature(160.0),
                MaxMeanTemperature(180.0),
                ThinAtmosphere(Ammonia),
            ],
        ),
        (
            ClypeusSpeculumi,
            all![
                MaxGravity(0.276),
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
                MaxGravity(0.276),
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
                MaxGravity(0.276),
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
                    MaxGravity(0.276),
                    any![
                        ThinAtmosphere(CarbonDioxide),
                        ThinAtmosphere(CarbonDioxideRich)
                    ],
                    MinMeanTemperature(180.0),
                    MaxMeanTemperature(195.0),
                ],
                all![
                    MaxGravity(0.276),
                    any![
                        ThinAtmosphere(Water),
                        ThinAtmosphere(WaterRich),
                    ],
                ],
            ],
        ),
        (
            ConchaAureolas,
            all![
                ThinAtmosphere(Ammonia),
                any![
                    PlanetClass(RockyBody),
                    PlanetClass(HighMetalContentBody),
                ],
                MinGravity(0.039),
                MaxGravity(0.276),
                MinMeanTemperature(152.0),
                MaxMeanTemperature(177.0),
                MaxPressure(0.013),
            ],
        ),
        (
            ConchaLabiata,
            all![
                ThinAtmosphere(CarbonDioxide),
                any![
                    PlanetClass(RockyBody),
                    PlanetClass(HighMetalContentBody),
                ],
                MinGravity(0.039),
                MaxGravity(0.276),
                MinMeanTemperature(150.0),
                MaxMeanTemperature(195.0),
                MaxMeanTemperature(195.0),
            ],
        ),
        (
            ConchaBiconcavis,
            all![
                MaxGravity(0.276),
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
                MaxGravity(0.276),
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
                MaxGravity(0.276),
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
                MaxGravity(0.276),
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
                MaxGravity(0.276),
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
                MaxGravity(0.276),
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
                MaxGravity(0.276),
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
                MaxGravity(0.276),
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
                MaxGravity(0.276),
            ],
        ),
        (
            FrutexaAcus,
            all![
                any![
                    Region(Region::Temple),
                    Region(Region::GalacticCenter),
                    Region(Region::Izanami),
                    Region(Region::InnerOrionSpur),
                    Region(Region::OuterOrionSpur),
                    Region(Region::OrionCygnusArm),
                    Region(Region::OdinsHold),
                    Region(Region::InnerOrionPerseusConflux),
                ],
                ThinAtmosphere(CarbonDioxide),
                PlanetClass(RockyBody),
                MinGravity(0.04),
                MaxGravity(0.276),
                MinMeanTemperature(147.0),
                MaxMeanTemperature(195.0),
                MinPressure(0.003),
            ],
        ),
        (
            FrutexaCollum,
            all![
                ThinAtmosphere(SulfurDioxide),
                PlanetClass(RockyBody),
                MaxGravity(0.276),
            ],
        ),
        (
            FrutexaFera,
            all![
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
                ThinAtmosphere(CarbonDioxide),
                PlanetClass(RockyBody),
                MinGravity(0.04),
                MaxGravity(0.25),
                MinMeanTemperature(145.0),
                MaxMeanTemperature(200.0),
                MinPressure(0.003),
            ],
        ),
        (
            FrutexaFlabellum,
            all![
                ThinAtmosphere(Ammonia),
                PlanetClass(RockyBody),
                MaxPressure(0.014),
                MinGravity(0.04),
                MaxGravity(0.28),
                MinMeanTemperature(155.0),
                MaxMeanTemperature(177.0),
            ],
        ),
        (
            FrutexaFlammasis,
            all![
                any![
                    Region(Region::GalacticCenter),
                    Region(Region::NormaExpanse),
                    Region(Region::InnerScutumCentaurusArm),
                    Region(Region::OuterScutumCentaurusArm),
                    Region(Region::AquilasHalo),
                    Region(Region::OdinsHold),
                    Region(Region::TheVeils),
                    Region(Region::TheVoid),
                    Region(Region::HieronymusDelta),
                    Region(Region::FormorianFrontier),
                    Region(Region::TrojanBelt),
                ],
                PlanetClass(RockyBody),
                ThinAtmosphere(Ammonia),
                MaxPressure(0.0134),
                MinGravity(0.04),
                MaxGravity(0.28),
                MinMeanTemperature(152.0),
                MaxMeanTemperature(177.0),
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
                MaxGravity(0.276),
            ],
        ),
        (
            FrutexaSponsae,
            all![
                ThinAtmosphere(Water),
                PlanetClass(RockyBody),
                MaxGravity(0.276),
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
                MaxGravity(0.276),
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
                MaxGravity(0.276),
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
                MaxGravity(0.276),
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
                MaxGravity(0.276),
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
                MaxGravity(0.276),
            ],
        ),
        (
            FungoidaGelata,
            all![
                any![
                    PlanetClass(RockyBody),
                    PlanetClass(RockyIceBody),
                    PlanetClass(HighMetalContentBody),
                ],
                MinPressure(0.0026),
                MaxGravity(0.28),
                any![
                    all![
                        ThinAtmosphere(Water),
                        MinMeanTemperature(392.0),
                        MaxMeanTemperature(452.0),
                    ],
                    all![
                        ThinAtmosphere(Ammonia),
                        MinMeanTemperature(160.0),
                        MaxMeanTemperature(177.0),
                    ],
                    all![
                        ThinAtmosphere(CarbonDioxide),
                        MinMeanTemperature(180.0),
                        MaxMeanTemperature(197.0),
                    ],
                    all![
                        ThinAtmosphere(Methane),
                        MinMeanTemperature(79.0),
                        MaxMeanTemperature(107.0),
                    ],
                ],
            ],
        ),
        (
            FungoidaSetisis,
            all![
                any![
                    ThinAtmosphere(Ammonia),
                    ThinAtmosphere(Methane),
                ],
                MaxGravity(0.276),
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
                MaxGravity(0.276),
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
                MaxGravity(0.276),
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
                MaxGravity(0.276),
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
                MaxGravity(0.276),
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
                MaxGravity(0.276),
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
                MaxGravity(0.276),
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
                MaxGravity(0.276)
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
                MaxGravity(0.276),
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
                MaxGravity(0.276),
            ],
        ),
        (
            ReceptaUmbrux,
            all![
                ThinAtmosphere(SulfurDioxide),
                MaxGravity(0.276),
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
                MaxGravity(0.56),
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
                any![
                    Region(Region::InnerOrionSpur),
                    Region(Region::OuterOrionSpur),
                    Region(Region::Izanami),
                    Region(Region::GalacticCenter),
                    Region(Region::OrionCygnusArm),
                    Region(Region::OdinsHold),
                    Region(Region::InnerOrionPerseusConflux),
                    Region(Region::Temple),
                ],
                ThinAtmosphere(Ammonia),
                PlanetClass(RockyBody),
                MaxPressure(0.0134),
                MinGravity(0.04),
                MaxGravity(0.35),
                MinMeanTemperature(165.0),
                MaxMeanTemperature(177.0),
            ],
        ),
        (
            StratumLimaxus,
            all![
                any![
                    ThinAtmosphere(SulfurDioxide),
                    ThinAtmosphere(CarbonDioxide),
                ],
                any![
                    Region(Region::NormaExpanse),
                    Region(Region::InnerScutumCentaurusArm),
                    Region(Region::OuterScutumCentaurusArm),
                    Region(Region::FormorianFrontier),
                    Region(Region::TheVoid),
                    Region(Region::AquilasHalo),
                    Region(Region::HieronymusDelta),
                    Region(Region::TheVeils),
                    Region(Region::TrojanBelt),
                ],
                PlanetClass(RockyBody),
                MinMeanTemperature(165.0),
                MaxMeanTemperature(190.0),
                MinGravity(0.04),
                MaxGravity(0.5),
            ],
        ),
        (
            StratumPaleas,
            any![
                all![
                    ThinAtmosphere(Ammonia),
                    MinMeanTemperature(165.0),
                    MaxMeanTemperature(177.0),
                    // any![
                    //     Region(Region::InnerOrionSpur),
                    //     Region(Region::ArcadianStream),
                    //     Region(Region::NormaExpanse),
                    //     Region(Region::OdinsHold),
                    //     Region(Region::InnerScutumCentaurusArm),
                    //     Region(Region::OuterOrionPerseusConflux),
                    //     Region(Region::TrojanBelt),
                    //     Region(Region::ElysianShore),
                    //     Region(Region::AquilasHalo),
                    //     Region(Region::ErrantMarches),
                    //     Region(Region::VulcanGate),
                    //     Region(Region::Xibalba),
                    //     Region(Region::DrymansPoint),
                    //     Region(Region::SagittariusCarinaArm),
                    // ],
                ],

                all![
                    ThinAtmosphere(CarbonDioxide),
                    MinGravity(0.04),
                    MaxGravity(0.27),
                    MinMeanTemperature(165.0),
                    MaxMeanTemperature(196.0),
                ],

                all![
                    ThinAtmosphere(CarbonDioxide),
                    MinGravity(0.27),
                    MinMeanTemperature(165.0),
                    MaxMeanTemperature(400.0),
                ],

                all![
                    ThinAtmosphere(CarbonDioxideRich),
                    MinMeanTemperature(170.0),
                    MaxMeanTemperature(255.0),
                ],
                all![
                    ThinAtmosphere(Oxygen),
                    MinMeanTemperature(165.0),
                    MaxMeanTemperature(250.0),
                    MinGravity(0.04),
                    MinGravity(0.056),
                ],
                all![
                    ThinAtmosphere(Water),
                    MinMeanTemperature(397.0),
                    MaxMeanTemperature(450.0),
                    MinGravity(0.04),
                    MaxGravity(0.056),
                    MinPressure(0.055),
                ],
                ThinAtmosphere(Nitrogen),
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
                any![
                    Region(Region::InnerScutumCentaurusArm),
                    Region(Region::OuterScutumCentaurusArm),
                    Region(Region::FormorianFrontier),
                    Region(Region::GalacticCenter),
                    Region(Region::NormaExpanse),
                    Region(Region::OdinsHold),
                    Region(Region::TheVoid),
                    Region(Region::AquilasHalo),
                    Region(Region::HieronymusDelta),
                    Region(Region::TheVeils),
                    Region(Region::TrojanBelt),
                ],
                PlanetClass(RockyBody),
                MinGravity(0.04),
                MaxGravity(0.15),
                MinMeanTemperature(160.0),
                MaxMeanTemperature(190.0),
            ],
        ),
        (
            TubusCompagibus,
            all![
                any![
                    Region(Region::GalacticCenter),
                    Region(Region::OdinsHold),
                    Region(Region::InnerScutumCentaurusArm),
                    Region(Region::InnerOrionSpur),
                    Region(Region::HawkingsGap),
                    Region(Region::TheAbyss),
                    Region(Region::Acheron),
                    Region(Region::MareSomnia),
                    Region(Region::DrymansPoint),
                    Region(Region::SagittariusCarinaArm),
                ],
                ThinAtmosphere(CarbonDioxide),
                PlanetClass(RockyBody),
                MinGravity(0.04),
                MaxGravity(0.152),
                MinMeanTemperature(160.0),
                MaxMeanTemperature(195.0),
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
                MaxGravity(0.276),
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
                MaxGravity(0.276),
                MinMeanTemperature(80.0),
                MaxMeanTemperature(130.0),
            ],
        ),
        (
            TussockCaputus,
            all![
                ThinAtmosphere(CarbonDioxide),
                PlanetClass(RockyBody),
                MaxGravity(0.276),
                MinMeanTemperature(180.0),
                MaxMeanTemperature(190.0),
            ],
        ),
        (
            TussockCatena,
            all![
                any![
                    Region(Region::NormaExpanse),
                    Region(Region::InnerScutumCentaurusArm),
                    Region(Region::OuterScutumCentaurusArm),
                    Region(Region::TheVoid),
                    Region(Region::HieronymusDelta),
                    Region(Region::TheVeils),
                    Region(Region::AquilasHalo),
                    Region(Region::FormorianFrontier),
                    Region(Region::TrojanBelt),
                ],
                ThinAtmosphere(Ammonia),
                PlanetClass(RockyBody),
                MaxPressure(0.0134),
                MinGravity(0.04),
                MaxGravity(0.28),
                MinMeanTemperature(152.0),
                MaxMeanTemperature(177.0),
            ],
        ),
        (
            TussockCultro,
            all![
                any![
                    Region(Region::InnerOrionSpur),
                    Region(Region::OuterOrionSpur),
                    Region(Region::GalacticCenter),
                    Region(Region::Izanami),
                    Region(Region::OrionCygnusArm),
                    Region(Region::OdinsHold),
                    Region(Region::Temple),
                    Region(Region::InnerOrionPerseusConflux),
                ],
                ThinAtmosphere(Ammonia),
                PlanetClass(RockyBody),
                MaxPressure(0.0134),
                MinGravity(0.04),
                MaxGravity(0.28),
                MinMeanTemperature(152.0),
                MaxMeanTemperature(177.0),
            ],
        ),
        (
            TussockDivisa,
            all![
                any![
                    PlanetClass(RockyBody),
                    PlanetClass(HighMetalContentBody),
                ],
                any![
                    Region(Region::RykersHope),
                    Region(Region::OuterOrionPerseusConflux),
                    Region(Region::ElysianShore),
                    Region(Region::PerseusArm),
                    Region(Region::VulcanGate),
                    Region(Region::SanguineousRim),
                    Region(Region::AchillessAltar),
                    Region(Region::LysasSong),
                    Region(Region::Tenebrae),
                ],
                ThinAtmosphere(Ammonia),
                MinMeanTemperature(152.0),
                MaxMeanTemperature(177.0),
                MinGravity(0.04),
                MaxGravity(0.276),
            ],
        ),
        (
            TussockIgnis,
            all![
                any![
                    PlanetClass(RockyBody),
                    PlanetClass(HighMetalContentBody),
                ],
                ThinAtmosphere(CarbonDioxide),
                MinGravity(0.04),
                MaxGravity(0.20),
                MinMeanTemperature(161.0),
                MaxMeanTemperature(170.0),
                MinPressure(0.0028),
                MaxPressure(0.055),
            ],
        ),
        (
            TussockPennata,
            all![
                ThinAtmosphere(CarbonDioxide),
                PlanetClass(RockyBody),
                MaxGravity(0.276),
                MinMeanTemperature(145.0),
                MaxMeanTemperature(155.0),
            ],
        ),
        (
            TussockPennatis,
            all![
                any![
                    PlanetClass(RockyBody),
                    PlanetClass(HighMetalContentBody),
                ],
                ThinAtmosphere(CarbonDioxide),
                MaxGravity(0.04),
                MaxGravity(0.276),
                MinMeanTemperature(145.0),
                MaxMeanTemperature(195.0),
            ],
        ),
        (
            TussockPropagito,
            all![
                any![
                    PlanetClass(RockyBody),
                    PlanetClass(HighMetalContentBody),
                ],
                any![
                    Region(Region::GalacticCenter),
                    Region(Region::OdinsHold),
                    Region(Region::InnerScutumCentaurusArm),
                    Region(Region::OuterScutumCentaurusArm),
                    Region(Region::FormorianFrontier),
                    Region(Region::HieronymusDelta),
                    Region(Region::NormaExpanse),
                    Region(Region::TheVoid),
                    Region(Region::AquilasHalo),
                    Region(Region::TrojanBelt),
                    Region(Region::TheVeils),
                ],
                ThinAtmosphere(CarbonDioxide),
                MinGravity(0.04),
                MaxGravity(0.276),
                MinMeanTemperature(145.0),
                MaxMeanTemperature(200.0),
                MinPressure(0.003),
            ],
        ),
        (
            TussockSerrati,
            all![
                ThinAtmosphere(CarbonDioxide),
                PlanetClass(RockyBody),
                MaxGravity(0.276),
                MinMeanTemperature(170.0),
                MaxMeanTemperature(175.0),
            ],
        ),
        (
            TussockStigmasis,
            all![
                ThinAtmosphere(SulfurDioxide),
                PlanetClass(RockyBody),
                MaxGravity(0.276),
            ],
        ),
        (
            TussockTriticum,
            all![
                ThinAtmosphere(CarbonDioxide),
                PlanetClass(RockyBody),
                MaxGravity(0.276),
                MinMeanTemperature(190.0),
                MaxMeanTemperature(195.0),
            ],
        ),
        (
            TussockVentusa,
            all![
                ThinAtmosphere(CarbonDioxide),
                PlanetClass(RockyBody),
                MaxGravity(0.276),
                MinMeanTemperature(155.0),
                MaxMeanTemperature(160.0),
            ],
        ),
        (
            TussockVirgam,
            all![
                ThinAtmosphere(Water),
                PlanetClass(RockyBody),
                MaxGravity(0.276),
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
    use std::collections::HashSet;
    use std::env::current_dir;
    use std::fs::File;
    use crate::exobiology::{SpawnCondition, Species};
    use crate::galaxy::{Atmosphere, AtmosphereDensity, AtmosphereType, BodyType, Gravity, PlanetClass, Region, Volcanism, VolcanismClassification, VolcanismType};

    const ALL_CSV_FILES: &[&str] = &[
        "aleoida-arcus.csv",
        "aleoida-coronamus.csv",
        "aleoida-gravis.csv",
        "aleoida-laminiae.csv",
        "aleoida-spica.csv",
        // "amphora-plant.csv",
        // "anemone.csv",
        "bacterium-acies.csv",
        "bacterium-alcyoneum.csv",
        "bacterium-aurasus.csv",
        "bacterium-bullaris.csv",
        "bacterium-cerbrus.csv",
        "bacterium-informem.csv",
        "bacterium-nebulus.csv",
        "bacterium-omentum.csv",
        "bacterium-scopulum.csv",
        "bacterium-tela.csv",
        "bacterium-verrata.csv",
        "bacterium-vesicula.csv",
        "bacterium-volu.csv",
        "cactoida-cortexum.csv",
        "cactoida-lapis.csv",
        "cactoida-peperatis.csv",
        "cactoida-pullulanta.csv",
        "cactoida-vermis.csv",
        "clypeus-lacrimam.csv",
        "clypeus-margaritus.csv",
        "clypeus-speculumi.csv",
        "concha-aureolas.csv",
        "concha-biconcavis.csv",
        "concha-labiata.csv",
        "concha-renibus.csv",
        "electricae-pluma.csv",
        "electricae-radialem.csv",
        "fonticulua-campestris.csv",
        "fonticulua-digitos.csv",
        "fonticulua-fluctus.csv",
        "fonticulua-lapida.csv",
        "fonticulua-segmentatus.csv",
        "fonticulua-upupam.csv",
        "frutexa-acus.csv",
        "frutexa-collum.csv",
        "frutexa-fabellum.csv",
        "frutexa-fera.csv",
        "frutexa-flammasis.csv",
        "frutexa-metallicum.csv",
        "frutexa-sponsae.csv",
        "fumerola-aquatis.csv",
        "fumerola-carbosis.csv",
        "fumerola-extremus.csv",
        "fumerola-nitris.csv",
        "fungoida-bullarum.csv",
        "fungoida-gelata.csv",
        "fungoida-setisis.csv",
        "fungoida-stabitis.csv",
        "osseus-cornibus.csv",
        "osseus-discus.csv",
        "osseus-fractus.csv",
        "osseus-pellebantus.csv",
        "osseus-pumice.csv",
        "osseus-spiralis.csv",
        "recepta-conditivus.csv",
        "recepta-deltahedronix.csv",
        "recepta-umbrux.csv",
        "stratum-araneamus.csv",
        "stratum-cucumisis.csv",
        "stratum-excutitus.csv",
        "stratum-frigus.csv",
        "stratum-laminamus.csv",
        "stratum-limaxus.csv",
        "stratum-paleas.csv",
        "stratum-tectonicas.csv",
        "tubus-cavas.csv",
        "tubus-compagibus.csv",
        "tubus-conifer.csv",
        "tubus-rosarium.csv",
        "tubus-sororibus.csv",
        "tussock-albata.csv",
        "tussock-capillum.csv",
        "tussock-caputus.csv",
        "tussock-catena.csv",
        "tussock-cultro.csv",
        "tussock-divisa.csv",
        "tussock-ignis.csv",
        "tussock-pennata.csv",
        "tussock-pennatis.csv",
        "tussock-propagito.csv",
        "tussock-serrati.csv",
        "tussock-stigmasis.csv",
        "tussock-triticum.csv",
        "tussock-ventusa.csv",
        "tussock-virgam.csv",
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
        let test_cases = get_test_entries(file_name);

        let spawn_conditions = species.spawn_conditions();
        let mut failed_cases = Vec::new();
        let mut succeeded = Vec::new();
        let mut entry_count: usize = 0;
        let mut exclude_entry_count: usize = 0;

        let mut expect_success = HashSet::new();

        // Check for failed predictions where they should have succeeded
        for case in test_cases {
            entry_count += 1;
            expect_success.insert(case.name.to_string());

            if !check_spawn_condition(&spawn_conditions, &case) {
                failed_cases.push(case);
            }
        }

        let mut skipped_expected: usize = 0;

        for file in ALL_CSV_FILES {
            if file == &file_name {
                continue;
            }

            let test_cases = get_test_entries(file);

            for case in test_cases {
                // This takes care of cases where the test case is expected to success as it's part
                // of multiple test files and takes care of species that have overlapping spawn
                // conditions.
                if expect_success.contains(&case.name) {
                    skipped_expected += 1;
                    continue;
                }

                exclude_entry_count += 1;

                if check_spawn_condition(&spawn_conditions, &case) {
                    succeeded.push(case);
                }
            }
        }

        let failed_ratio = failed_cases.len() as f32 / entry_count as f32;
        let false_pos_ratio = succeeded.len() as f32 / exclude_entry_count as f32;

        dbg!(&failed_ratio, &false_pos_ratio, expect_success.len(), skipped_expected);

        // 0.5% of cases are allowed to fail
        if failed_ratio >= 0.005 {
            dbg!(failed_cases.get(1));
            assert!(false);
        }

        // 25% of cases are allowed to succeed. Some species just have some overlap for spawning
        // conditions so best to not worry too much about this number. It's much more important to
        // make sure none of the actual cases fail.
        if false_pos_ratio >= 0.25 {
            dbg!(succeeded.get(0));
            assert!(false);
        }
    }

    // /// Checks that all the spawn conditions apply to the specified test file and ensures that the
    // /// number of failed predictions stay under a certain percentage.
    // fn success_species_planet_details(species: &Species, file_name: &str) {
    //
    // }
    //
    // /// Checks that the spawn conditions do not apply to all other entries in the test-suite and
    // /// ensures that the number of false positives stay under a said percentage.
    // fn fail_species_planet_details(species: &Species, file_name: &str) {
    //     let spawn_conditions = species.spawn_conditions();
    //
    //     let mut entry_count: usize = 0;
    //
    //
    //
    //
    //     dbg!(&ratio);
    //
    //     // TODO see how this can be decreased. To do this this probably needs to take into account
    //     //  which species commonly overlap
    //
    // }

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
                &planet_details.volcanism.kind != &VolcanismType::None
            }
            SpawnCondition::MinPressure(min_pressure) => {
                &planet_details.pressure >= min_pressure
            }
            SpawnCondition::MaxPressure(max_pressure) => {
                &planet_details.pressure <= max_pressure
            }
            SpawnCondition::Region(region) => {
                &planet_details.region == region || &planet_details.region == &Region::Unknown
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

    // Aleoids
    #[test]
    fn aleoida_arcus_test_cases_all_pass() {
        test_species_planet_details(Species::AleoidaArcus, "aleoida-arcus.csv");
    }

    #[test]
    fn aleoida_coronamus_test_cases_all_pass() {
        test_species_planet_details(Species::AleoidaCoronamus, "aleoida-coronamus.csv");
    }

    #[test]
    fn aleoida_laminiae_test_cases_all_pass() {
        test_species_planet_details(Species::AleoidaLaminiae, "aleoida-laminiae.csv");
    }

    #[test]
    fn aleoida_gravis_test_cases_all_pass() {
        test_species_planet_details(Species::AleoidaGravis, "aleoida-gravis.csv");
    }

    #[test]
    fn aleoida_spica_test_cases_all_pass() {
        test_species_planet_details(Species::AleoidaSpica, "aleoida-spica.csv");
    }

    // Amphora plant

    #[test]
    fn amphora_plant_test_cases_all_pass() {
        test_species_planet_details(Species::AmphoraPlant, "amphora-plant.csv");
    }

    // Anemone

    #[test]
    fn anemone_test_cases_all_pass() {
        todo!()
        // test_species_planet_details(Species::Anemone, "amphora-plant.csv");
    }

    // Bacteria

    #[test]
    fn bacterium_acies_test_cases_all_pass() {
        test_species_planet_details(Species::BacteriumAcies, "bacterium-acies.csv");
    }

    #[test]
    fn bacterium_alcyoneum_test_cases_all_pass() {
        test_species_planet_details(Species::BacteriumAlcyoneum, "bacterium-alcyoneum.csv");
    }

    #[test]
    fn bacterium_aurasus_test_cases_all_pass() {
        test_species_planet_details(Species::BacteriumAurasus, "bacterium-aurasus.csv");
    }

    #[test]
    fn bacterium_bullaris_test_cases_all_pass() {
        test_species_planet_details(Species::BacteriumBullaris, "bacterium-bullaris.csv");
    }

    #[test]
    fn bacterium_cerbrus_test_cases_all_pass() {
        test_species_planet_details(Species::BacteriumCerbrus, "bacterium-cerbrus.csv");
    }

    #[test]
    fn bacterium_informem_test_cases_all_pass() {
        test_species_planet_details(Species::BacteriumInformem, "bacterium-informem.csv");
    }

    #[test]
    fn bacterium_nebulus_test_cases_all_pass() {
        test_species_planet_details(Species::BacteriumNebulus, "bacterium-nebulus.csv");
    }

    #[test]
    fn bacterium_omentum_test_cases_all_pass() {
        test_species_planet_details(Species::BacteriumOmentum, "bacterium-omentum.csv");
    }

    #[test]
    fn bacterium_scopulum_test_cases_all_pass() {
        test_species_planet_details(Species::BacteriumScopulum, "bacterium-scopulum.csv");
    }

    #[test]
    fn bacterium_tela_test_cases_all_pass() {
        test_species_planet_details(Species::BacteriumTela, "bacterium-tela.csv");
    }

    #[test]
    fn bacterium_verrata_test_cases_all_pass() {
        test_species_planet_details(Species::BacteriumVerrata, "bacterium-verrata.csv");
    }

    #[test]
    fn bacterium_vesicula_test_cases_all_pass() {
        test_species_planet_details(Species::BacteriumVesicula, "bacterium-vesicula.csv");
    }

    #[test]
    fn bacterium_volu_test_cases_all_pass() {
        test_species_planet_details(Species::BacteriumVolu, "bacterium-volu.csv");
    }

    // Cactoida

    #[test]
    fn cactoida_cortexum_test_cases_all_pass() {
        test_species_planet_details(Species::CactoidaCortexum, "cactoida-cortexum.csv");
    }

    #[test]
    fn cactoida_lapis_test_cases_all_pass() {
        test_species_planet_details(Species::CactoidaLapis, "cactoida-lapis.csv");
    }

    #[test]
    fn cactoida_peperatis_test_cases_all_pass() {
        test_species_planet_details(Species::CactoidaPeperatis, "cactoida-peperatis.csv");
    }

    #[test]
    fn cactoida_pullulanta_test_cases_all_pass() {
        test_species_planet_details(Species::CactoidaPullulanta, "cactoida-pullulanta.csv");
    }

    #[test]
    fn cactoida_vermis_test_cases_all_pass() {
        test_species_planet_details(Species::CactoidaVermis, "cactoida-vermis.csv");
    }

    // Cypeus

    #[test]
    fn clypeus_lacrimam_test_cases_all_pass() {
        test_species_planet_details(Species::ClypeusLacrimam, "clypeus-lacrimam.csv");
    }

    #[test]
    fn clypeus_margaritus_test_cases_all_pass() {
        test_species_planet_details(Species::ClypeusMargaritus, "clypeus-margaritus.csv");
    }

    #[test]
    fn clypeus_speculumi_test_cases_all_pass() {
        test_species_planet_details(Species::ClypeusSpeculumi, "clypeus-speculumi.csv");
    }

    // Concha

    #[test]
    fn concha_aureolas_test_cases_all_pass() {
        test_species_planet_details(Species::ConchaAureolas, "concha-aureolas.csv");
    }

    #[test]
    fn concha_biconcavis_test_cases_all_pass() {
        test_species_planet_details(Species::ConchaBiconcavis, "concha-biconcavis.csv");
    }

    #[test]
    fn concha_labiata_test_cases_all_pass() {
        test_species_planet_details(Species::ConchaLabiata, "concha-labiata.csv");
    }

    #[test]
    fn concha_renibus_test_cases_all_pass() {
        test_species_planet_details(Species::ConchaRenibus, "concha-renibus.csv");
    }

    // Electricae

    #[test]
    fn electricae_pluma_test_cases_all_pass() {
        test_species_planet_details(Species::ElectricaePluma, "electricae-pluma.csv");
    }

    #[test]
    fn electricae_radialem_test_cases_all_pass() {
        test_species_planet_details(Species::ElectricaeRadialem, "electricae-radialem.csv");
    }

    // Fonticula

    #[test]
    fn fonticulua_campestris_test_cases_all_pass() {
        test_species_planet_details(Species::FonticuluaCampestris, "fonticulua-campestris.csv");
    }

    #[test]
    fn fonticulua_digitos_test_cases_all_pass() {
        test_species_planet_details(Species::FonticuluaDigitos, "fonticulua-digitos.csv");
    }

    #[test]
    fn fonticulua_fluctus_test_cases_all_pass() {
        test_species_planet_details(Species::FonticuluaFluctus, "fonticulua-fluctus.csv");
    }

    #[test]
    fn fonticulua_lapida_test_cases_all_pass() {
        test_species_planet_details(Species::FonticuluaLapida, "fonticulua-lapida.csv");
    }

    #[test]
    fn fonticulua_segmentatus_test_cases_all_pass() {
        test_species_planet_details(Species::FonticuluaSegmentatus, "fonticulua-segmentatus.csv");
    }

    #[test]
    fn fonticulua_upupam_test_cases_all_pass() {
        test_species_planet_details(Species::FonticuluaUpupam, "fonticulua-upupam.csv");
    }

    // Frutexa

    #[test]
    fn frutexa_acus_test_cases_all_pass() {
        test_species_planet_details(Species::FrutexaAcus, "frutexa-acus.csv");
    }

    #[test]
    fn frutexa_collum_test_cases_all_pass() {
        test_species_planet_details(Species::FrutexaCollum, "frutexa-collum.csv");
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
    fn frutexa_flammasis_test_cases_all_pass() {
        test_species_planet_details(Species::FrutexaFlammasis, "frutexa-flammasis.csv");
    }

    #[test]
    fn frutexa_metallicum_test_cases_all_pass() {
        test_species_planet_details(Species::FrutexaMetallicum, "frutexa-metallicum.csv");
    }

    #[test]
    fn frutexa_sponsae_test_cases_all_pass() {
        test_species_planet_details(Species::FrutexaSponsae, "frutexa-sponsae.csv");
    }

    // Fumerola

    #[test]
    fn fumerola_aquatis_test_cases_all_pass() {
        test_species_planet_details(Species::FumerolaAquatis, "fumerola-aquatis.csv");
    }

    #[test]
    fn fumerola_carbosis_test_cases_all_pass() {
        test_species_planet_details(Species::FumerolaCarbosis, "fumerola-carbosis.csv");
    }

    #[test]
    fn fumerola_extremus_test_cases_all_pass() {
        test_species_planet_details(Species::FumerolaExtremus, "fumerola-extremus.csv");
    }

    #[test]
    fn fumerola_nitris_test_cases_all_pass() {
        test_species_planet_details(Species::FumerolaNitris, "fumerola-nitris.csv");
    }

    // Fungoida

    #[test]
    fn fungoida_ballarum_test_cases_all_pass() {
        test_species_planet_details(Species::FungoidaBullarum, "fungoida-bullarum.csv");
    }

    #[test]
    fn fungoida_gelata_test_cases_all_pass() {
        test_species_planet_details(Species::FungoidaGelata, "fungoida-gelata.csv");
    }

    #[test]
    fn fungoida_setisis_test_cases_all_pass() {
        test_species_planet_details(Species::FungoidaSetisis, "fungoida-setisis.csv");
    }

    #[test]
    fn fungoida_stabitis_test_cases_all_pass() {
        test_species_planet_details(Species::FungoidaStabitis, "fungoida-stabitis.csv");
    }

    // Osseus

    #[test]
    fn osseus_cornibus_test_cases_all_pass() {
        test_species_planet_details(Species::OsseusCornibus, "osseus-cornibus.csv");
    }

    #[test]
    fn osseus_discus_test_cases_all_pass() {
        test_species_planet_details(Species::OsseusDiscus, "osseus-discus.csv");
    }

    #[test]
    fn osseus_fractus_test_cases_all_pass() {
        test_species_planet_details(Species::OsseusFractus, "osseus-fractus.csv");
    }

    #[test]
    fn osseus_pellebantus_test_cases_all_pass() {
        test_species_planet_details(Species::OsseusPellebantus, "osseus-pellebantus.csv");
    }

    #[test]
    fn osseus_pumice_test_cases_all_pass() {
        test_species_planet_details(Species::OsseusPumice, "osseus-pumice.csv");
    }

    #[test]
    fn osseus_spiralis_test_cases_all_pass() {
        test_species_planet_details(Species::OsseusSpiralis, "osseus-spiralis.csv");
    }

    // Recepta

    #[test]
    fn recepta_conditivus_test_cases_all_pass() {
        test_species_planet_details(Species::ReceptaConditivus, "recepta-conditivus.csv");
    }

    #[test]
    fn recepta_deltahedronix_test_cases_all_pass() {
        test_species_planet_details(Species::ReceptaDeltahedronix, "recepta-deltahedronix.csv");
    }

    #[test]
    fn recepta_umbrux_test_cases_all_pass() {
        test_species_planet_details(Species::ReceptaUmbrux, "recepta-umbrux.csv");
    }

    // Stratum

    #[test]
    fn stratum_araneamus_test_cases_all_pass() {
        test_species_planet_details(Species::StratumAraneamus, "stratum-araneamus.csv");
    }

    #[test]
    fn stratum_cucumisis_test_cases_all_pass() {
        test_species_planet_details(Species::StratumCucumisis, "stratum-cucumisis.csv");
    }

    #[test]
    fn stratum_excutitus_test_cases_all_pass() {
        test_species_planet_details(Species::StratumExcutitus, "stratum-excutitus.csv");
    }

    #[test]
    fn stratum_frigus_test_cases_all_pass() {
        test_species_planet_details(Species::StratumFrigus, "stratum-frigus.csv");
    }

    #[test]
    fn stratum_laminamus_test_cases_all_pass() {
        test_species_planet_details(Species::StratumLaminamus, "stratum-laminamus.csv");
    }

    #[test]
    fn stratum_limaxus_test_cases_all_pass() {
        test_species_planet_details(Species::StratumLimaxus, "stratum-limaxus.csv");
    }

    #[test]
    fn stratum_paleas_test_cases_all_pass() {
        test_species_planet_details(Species::StratumPaleas, "stratum-paleas.csv");
    }

    #[test]
    fn stratum_tectonicas_test_cases_all_pass() {
        test_species_planet_details(Species::StratumTectonicas, "stratum-tectonicas.csv");
    }

    // Tubus

    #[test]
    fn tubus_cavas_test_cases_all_pass() {
        test_species_planet_details(Species::TubusCavas, "tubus-cavas.csv");
    }

    #[test]
    fn tubus_compagibus_test_cases_all_pass() {
        test_species_planet_details(Species::TubusCompagibus, "tubus-compagibus.csv");
    }

    #[test]
    fn tubus_conifer_test_cases_all_pass() {
        test_species_planet_details(Species::TubusConifer, "tubus-conifer.csv");
    }

    #[test]
    fn tubus_rosarium_test_cases_all_pass() {
        test_species_planet_details(Species::TubusRosarium, "tubus-rosarium.csv");
    }

    #[test]
    fn tubus_sororibus_test_cases_all_pass() {
        test_species_planet_details(Species::TubusSororibus, "tubus-sororibus.csv");
    }

    // Tussock

    #[test]
    fn tussock_albata_test_cases_all_pass() {
        test_species_planet_details(Species::TussockAlbata, "tussock-albata.csv");
    }

    #[test]
    fn tussock_capillum_test_cases_all_pass() {
        test_species_planet_details(Species::TussockCapillum, "tussock-capillum.csv");
    }

    #[test]
    fn tussock_caputus_test_cases_all_pass() {
        test_species_planet_details(Species::TussockCaputus, "tussock-caputus.csv");
    }

    #[test]
    fn tussock_catena_test_cases_all_pass() {
        test_species_planet_details(Species::TussockCatena, "tussock-catena.csv");
    }

    #[test]
    fn tussock_cultro_test_cases_all_pass() {
        test_species_planet_details(Species::TussockCultro, "tussock-cultro.csv");
    }

    #[test]
    fn tussock_divisa_test_cases_all_pass() {
        test_species_planet_details(Species::TussockDivisa, "tussock-divisa.csv");
    }

    #[test]
    fn tussock_ignis_test_cases_all_pass() {
        test_species_planet_details(Species::TussockIgnis, "tussock-ignis.csv");
    }

    #[test]
    fn tussock_pennata_test_cases_all_pass() {
        test_species_planet_details(Species::TussockPennata, "tussock-pennata.csv");
    }

    #[test]
    fn tussock_pennatis_test_cases_all_pass() {
        test_species_planet_details(Species::TussockPennatis, "tussock-pennatis.csv");
    }

    #[test]
    fn tussock_propagito_test_cases_all_pass() {
        test_species_planet_details(Species::TussockPropagito, "tussock-propagito.csv");
    }

    #[test]
    fn tussock_serrati_test_cases_all_pass() {
        test_species_planet_details(Species::TussockSerrati, "tussock-serrati.csv");
    }

    #[test]
    fn tussock_stigmasis_test_cases_all_pass() {
        test_species_planet_details(Species::TussockStigmasis, "tussock-stigmasis.csv");
    }

    #[test]
    fn tussock_triticum_test_cases_all_pass() {
        test_species_planet_details(Species::TussockTriticum, "tussock-triticum.csv");
    }

    #[test]
    fn tussock_ventusa_test_cases_all_pass() {
        test_species_planet_details(Species::TussockVentusa, "tussock-ventusa.csv");
    }

    #[test]
    fn tussock_virgam_test_cases_all_pass() {
        test_species_planet_details(Species::TussockVirgam, "tussock-virgam.csv");
    }
}
