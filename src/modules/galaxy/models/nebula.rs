use std::fmt::{Display, Formatter};

use crate::galaxy::GalacticDistance;
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, EnumIter)]
pub enum Nebula {
    AquilaDarkRegion,
    B133,
    B352,
    B92,
    BarnardsLoop,
    BlinkingNebula,
    BlueFlashNebula,
    BluePlanetaryNebula,
    BlueSnowballNebula,
    BowTieNebula,
    BoxNebula,
    BubbleNebula,
    BugNebula,
    ButterflyNebula,
    CaliforniaNebula,
    CatsEyeNebula,
    CatsPawNebula,
    CaveNebula,
    CepheusDarkRegion,
    Chamaeleon,
    CoalsackDarkRegion,
    CocoonNebula,
    ConeNebula,
    CoronaAustrDarkRegion,
    CrabNebula,
    CrescentNebula,
    DumbbellNebula,
    EagleNebula,
    EightBurstNebula,
    ElephantsTrunkNebula,
    EtaCarinaNebula,
    FetusNebula,
    FineRingNebula,
    FlameNebula,
    FlamingStarNebula,
    G2DustCloud,
    GhostOfJupiter,
    HeartNebula,
    HelixNebula,
    HindNebula,
    HorseheadDarkRegion,
    HorseheadNebula,
    HubblesVariable,
    Ic1287,
    Ic1297,
    Ic2149,
    Ic2165,
    Ic2448,
    Ic2501,
    Ic2553,
    Ic2621,
    Ic289,
    Ic4191,
    Ic4604,
    Ic4634,
    Ic4663,
    Ic4673,
    Ic4699,
    Ic4776,
    Ic4846,
    Ic4997,
    Ic5117,
    Ic5148,
    Ic5217,
    IrisNebula,
    JellyfishNebula,
    LagoonNebula,
    Lbn623,
    LemonSliceNebula,
    LittleDumbbellNebula,
    LittleGemNebula,
    LittleGhostNebula,
    LupusDarkRegion,
    LupusDarkRegionB,
    Messier78,
    MonkeyHeadNebula,
    MuscaDarkRegion,
    Ngc1333,
    Ngc1360,
    Ngc1491,
    Ngc1501,
    Ngc1514,
    Ngc1535,
    Ngc1931,
    Ngc1999,
    Ngc2022,
    Ngc2371,
    Ngc2392,
    Ngc2438,
    Ngc2440,
    Ngc2452,
    Ngc2792,
    Ngc281,
    Ngc2818,
    Ngc2867,
    Ngc2899,
    Ngc3195,
    Ngc3199,
    Ngc3211,
    Ngc3699,
    Ngc4361,
    Ngc5307,
    Ngc5315,
    Ngc5367,
    Ngc5873,
    Ngc5882,
    Ngc5979,
    Ngc6058,
    Ngc6153,
    Ngc6188,
    Ngc6210,
    Ngc6326,
    Ngc6337,
    Ngc6357,
    Ngc6445,
    Ngc6563,
    Ngc6565,
    Ngc6567,
    Ngc6572,
    Ngc6629,
    Ngc6644,
    Ngc6751,
    Ngc6781,
    Ngc6790,
    Ngc6803,
    Ngc6804,
    Ngc6820,
    Ngc6842,
    Ngc6852,
    Ngc6879,
    Ngc6884,
    Ngc6886,
    Ngc6891,
    Ngc7026,
    Ngc7027,
    Ngc7048,
    Ngc7354,
    Ngc7538,
    Ngc7822,
    NorthAmericaNebula,
    OmegaNebula,
    OphiuchusDarkRegion,
    OphiuchusDarkRegionB,
    OphiuchusDarkRegionC,
    OrionDarkRegion,
    OrionNebula,
    OwlNebula,
    ParrotsHead,
    PelicanNebula,
    PencilNebula,
    PerseusDarkRegion,
    PhantomStreak,
    PipeBowl,
    PipeStem,
    Pleiades,
    PuppisDarkRegion,
    PuppisDarkRegionB,
    RCra,
    RedSpiderNebula,
    RetinaNebula,
    RhoOphiuchi,
    RingNebula,
    RosetteNebula,
    RunningManNebula,
    SadrRegion,
    SaturnNebula,
    ScorpiusDarkRegion,
    ScutumDarkRegion,
    SeagullNebula,
    SkullAndCrossbonesNebula,
    SkullNebula,
    Snake,
    SoulNebula,
    SpiralPlanetaryNebula,
    SpirographNebula,
    StatueOfLibertyNebula,
    StruvesLostNebula,
    TaurusDarkRegion,
    ThorsHelmet,
    TrifidNebula,
    TrifidOfTheNorth,
    VeilNebulaEast,
    VeilNebulaWest,
    VelaDarkRegion,
    VulpeculaDarkRegion,
    WhiteEyedPea,
    WitchHeadNebula,
    FlyooGroaSOZE0,
    EollsGraaeAAAH31,
}

impl Nebula {
    pub fn center(&self) -> [f32; 3] {
        match self {
            Nebula::AquilaDarkRegion => [-713.2, -17.1, 691.9],
            Nebula::B133 => [-476.2, -115.6, 873.9],
            Nebula::B352 => [-1900.8, 8.1, 123.1],
            Nebula::B92 => [-139.8, -7.2, 640.0],
            Nebula::BarnardsLoop => [617.0, -421.6, -1224.6],
            Nebula::BlinkingNebula => [-1938.2, 443.1, 217.4],
            Nebula::BlueFlashNebula => [-2599.5, 500.3, 1411.4],
            Nebula::BluePlanetaryNebula => [4527.3, 409.7, 2082.3],
            Nebula::BlueSnowballNebula => [-5024.1, -1663.1, -1497.8],
            Nebula::BowTieNebula => [-2986.0, 601.8, -1724.0],
            Nebula::BoxNebula => [-1759.3, 2758.8, 10292.4],
            Nebula::BubbleNebula => [-6568.4, 24.7, -2687.3],
            Nebula::BugNebula => [619.5, 65.3, 3342.4],
            Nebula::ButterflyNebula => [1747.2, 188.3, -2431.5],
            Nebula::CaliforniaNebula => [-337.9, -211.9, -927.0],
            Nebula::CatsEyeNebula => [-2842.4, 1645.0, -323.9],
            Nebula::CatsPawNebula => [852.0, 59.0, 5426.8],
            Nebula::CaveNebula => [-2244.8, 110.1, -825.8],
            Nebula::CepheusDarkRegion => [-1366.6, 242.6, -127.8],
            Nebula::Chamaeleon => [488.5, -148.9, 303.4],
            Nebula::CoalsackDarkRegion => [448.8, -14.2, 258.5],
            Nebula::CocoonNebula => [-3175.2, -308.6, -245.0],
            Nebula::ConeNebula => [854.7, 83.9, -2025.8],
            Nebula::CoronaAustrDarkRegion => [-9.4, -178.9, 490.2],
            Nebula::CrabNebula => [558.5, -707.4, -6941.8],
            Nebula::CrescentNebula => [-4834.7, 212.0, 1255.6],
            Nebula::DumbbellNebula => [-958.2, -71.0, 535.5],
            Nebula::EagleNebula => [-2050.8, 94.9, 6691.9],
            Nebula::EightBurstNebula => [2049.6, 450.9, 75.2],
            Nebula::ElephantsTrunkNebula => [-2661.0, 180.2, -433.2],
            Nebula::EtaCarinaNebula => [8580.0, -139.0, 2701.4],
            Nebula::FetusNebula => [-2881.6, 277.9, -171.2],
            Nebula::FineRingNebula => [513.2, 34.9, 857.5],
            Nebula::FlameNebula => [625.8, -401.5, -1198.5],
            Nebula::FlamingStarNebula => [-233.5, -67.2, -1681.3],
            Nebula::G2DustCloud => [26.7, -22.9, 27900.6],
            Nebula::GhostOfJupiter => [1171.7, 743.9, -183.5],
            Nebula::HeartNebula => [-5319.3, 112.3, -5287.2],
            Nebula::HelixNebula => [-224.6, -587.8, 306.8],
            Nebula::HindNebula => [-33.0, -206.4, -557.3],
            Nebula::HorseheadDarkRegion => [618.5, -399.2, -1200.1],
            Nebula::HorseheadNebula => [649.6, -423.5, -1281.2],
            Nebula::HubblesVariable => [1210.3, 68.0, -2744.2],
            Nebula::Ic1287 => [-358.4, -8.8, 933.5],
            Nebula::Ic1297 => [215.0, -2872.0, 7249.0],
            Nebula::Ic2149 => [-1688.7, 1312.1, -6875.1],
            Nebula::Ic2165 => [9024.5, -3007.0, -10267.5],
            Nebula::Ic2448 => [8457.8, -2355.3, 2393.3],
            Nebula::Ic2501 => [18754.0, -1907.0, 3645.5],
            Nebula::Ic2553 => [12855.3, -1261.1, 3565.1],
            Nebula::Ic2621 => [14361.0, -1297.0, 5685.9],
            Nebula::Ic289 => [-1118.4, 83.0, -1277.6],
            Nebula::Ic4191 => [11811.6, -1205.0, 8148.3],
            Nebula::Ic4604 => [61.4, 182.2, 566.0],
            Nebula::Ic4634 => [-51.2, 1584.9, 7330.4],
            Nebula::Ic4663 => [1523.7, -927.1, 6250.5],
            Nebula::Ic4673 => [-840.7, -561.2, 13361.8],
            Nebula::Ic4699 => [4137.5, -4925.0, 19465.0],
            Nebula::Ic4776 => [-855.5, -5562.0, 23331.0],
            Nebula::Ic4846 => [-11325.5, -4179.0, 21663.5],
            Nebula::Ic4997 => [-6681.4, -1526.5, 4126.5],
            Nebula::Ic5117 => [-2988.1, -266.7, 5.2],
            Nebula::Ic5148 => [-86.3, -2376.9, 1828.4],
            Nebula::Ic5217 => [-9198.6, -884.6, -1721.5],
            Nebula::IrisNebula => [-1413.7, 368.1, -353.6],
            Nebula::JellyfishNebula => [785.2, 251.6, -4922.2],
            Nebula::LagoonNebula => [-468.0, -92.2, 4474.6],
            Nebula::Lbn623 => [-499.9, -16.4, -332.6],
            Nebula::LemonSliceNebula => [-3085.4, 2548.8, -2057.7],
            Nebula::LittleDumbbellNebula => [-1560.7, -382.7, -1351.9],
            Nebula::LittleGemNebula => [-2494.0, -1844.2, 5136.1],
            Nebula::LittleGhostNebula => [-204.1, 503.7, 4869.8],
            Nebula::LupusDarkRegion => [157.2, 129.7, 416.8],
            Nebula::LupusDarkRegionB => [165.8, 79.9, 424.3],
            Nebula::Messier78 => [538.5, -321.5, -1139.3],
            Nebula::MonkeyHeadNebula => [1135.1, 42.3, -6296.6],
            Nebula::MuscaDarkRegion => [420.7, -72.0, 244.3],
            Nebula::Ngc1333 => [-382.2, -383.4, -958.3],
            Nebula::Ngc1360 => [437.2, -925.2, -513.8],
            Nebula::Ngc1491 => [-4909.3, -175.3, -8708.5],
            Nebula::Ngc1501 => [-2071.6, 413.8, -2915.0],
            Nebula::Ngc1514 => [-202.3, -218.7, -807.4],
            Nebula::Ngc1535 => [1422.9, -2733.3, -2853.9],
            Nebula::Ngc1931 => [-743.8, 37.4, -6959.2],
            Nebula::Ngc1999 => [549.3, -374.5, -926.6],
            Nebula::Ngc2022 => [2934.6, -1966.6, -9781.7],
            Nebula::Ngc2371 => [661.5, 1497.7, -4084.1],
            Nebula::Ngc2392 => [234.6, 239.2, -726.5],
            Nebula::Ngc2438 => [2508.3, 228.8, -1973.8],
            Nebula::Ngc2440 => [4653.6, 238.7, -3282.8],
            Nebula::Ngc2452 => [9387.2, -183.3, -4700.8],
            Nebula::Ngc2792 => [8157.0, 586.3, -599.0],
            Nebula::Ngc281 => [-6660.3, -870.1, -4345.6],
            Nebula::Ngc2818 => [8322.6, 1271.0, -1169.7],
            Nebula::Ngc2867 => [12208.2, -1274.7, 1759.2],
            Nebula::Ngc2899 => [6434.6, -430.8, 812.8],
            Nebula::Ngc3195 => [4656.5, -1895.5, 2331.8],
            Nebula::Ngc3199 => [14574.2, -259.6, 3511.9],
            Nebula::Ngc3211 => [8797.9, -785.8, 2572.7],
            Nebula::Ngc3699 => [4150.3, 102.1, 1736.1],
            Nebula::Ngc4361 => [3110.0, 3241.0, 1390.0],
            Nebula::Ngc5307 => [5879.4, 1490.0, 5368.6],
            Nebula::Ngc5315 => [6499.6, -644.5, 5282.1],
            Nebula::Ngc5367 => [1348.5, 751.3, 1418.8],
            Nebula::Ngc5873 => [13791.7, 8670.0, 25191.0],
            Nebula::Ngc5882 => [4616.6, 1543.2, 7331.1],
            Nebula::Ngc5979 => [5443.0, -831.3, 7119.2],
            Nebula::Ngc6058 => [-5473.0, 6794.0, 2587.0],
            Nebula::Ngc6153 => [1670.2, 508.2, 5110.0],
            Nebula::Ngc6188 => [1705.8, -86.8, 4054.0],
            Nebula::Ngc6210 => [-2861.4, 3248.4, 3057.8],
            Nebula::Ngc6326 => [4041.2, -1606.9, 10103.8],
            Nebula::Ngc6337 => [901.2, -94.1, 4815.5],
            Nebula::Ngc6357 => [965.2, 142.3, 8094.4],
            Nebula::Ngc6445 => [-632.6, 306.1, 4444.8],
            Nebula::Ngc6563 => [80.5, -393.9, 3073.8],
            Nebula::Ngc6565 => [-359.0, -473.2, 5870.0],
            Nebula::Ngc6567 => [-851.7, -51.3, 4112.4],
            Nebula::Ngc6572 => [-4334.0, 1608.4, 6282.5],
            Nebula::Ngc6629 => [-1041.2, -568.9, 6289.1],
            Nebula::Ngc6644 => [-1420.0, -1245.3, 9616.3],
            Nebula::Ngc6751 => [-3105.8, -657.9, 5557.1],
            Nebula::Ngc6781 => [-3394.7, -266.9, 3796.7],
            Nebula::Ngc6790 => [-2014.9, -362.1, 2588.3],
            Nebula::Ngc6803 => [-4117.2, -407.6, 3920.8],
            Nebula::Ngc6804 => [-3573.0, -401.0, 3474.6],
            Nebula::Ngc6820 => [-5577.3, -10.1, 3341.2],
            Nebula::Ngc6842 => [-5476.7, 62.8, 2449.8],
            Nebula::Ngc6852 => [-3276.6, -1251.9, 3563.3],
            Nebula::Ngc6879 => [-17024.1, -3172.0, 10971.0],
            Nebula::Ngc6884 => [-2457.3, 309.0, 341.0],
            Nebula::Ngc6886 => [-7731.8, -1205.9, 4445.9],
            Nebula::Ngc6891 => [-6740.9, -1781.8, 4861.7],
            Nebula::Ngc7026 => [-5999.0, 41.9, 104.7],
            Nebula::Ngc7027 => [-3380.3, -207.6, 301.7],
            Nebula::Ngc7048 => [-5596.3, -166.2, 117.2],
            Nebula::Ngc7354 => [-3995.8, 168.5, -1282.9],
            Nebula::Ngc7538 => [-8375.8, 127.4, -3298.3],
            Nebula::Ngc7822 => [-2443.6, 301.4, -1333.5],
            Nebula::NorthAmericaNebula => [-1891.5, -30.8, 150.5],
            Nebula::OmegaNebula => [-1432.1, -75.5, 5308.1],
            Nebula::OphiuchusDarkRegion => [44.9, 151.2, 490.4],
            Nebula::OphiuchusDarkRegionB => [-44.4, 170.0, 487.6],
            Nebula::OphiuchusDarkRegionC => [-11.0, 68.6, 516.7],
            Nebula::OrionDarkRegion => [603.3, -312.2, -1337.7],
            Nebula::OrionNebula => [594.9, -431.4, -1071.8],
            Nebula::OwlNebula => [-624.4, 1847.2, -1018.9],
            Nebula::ParrotsHead => [22.7, -87.8, 989.8],
            Nebula::PelicanNebula => [-1886.9, 5.4, 176.3],
            Nebula::PencilNebula => [812.7, 1.6, -44.6],
            Nebula::PerseusDarkRegion => [-368.0, -321.7, -1047.6],
            Nebula::PhantomStreak => [-3611.9, -306.2, 5395.4],
            Nebula::PipeBowl => [-6.3, 37.2, 497.6],
            Nebula::PipeStem => [8.4, 51.6, 490.2],
            Nebula::Pleiades => [-81.8, -149.4, -343.4],
            Nebula::PuppisDarkRegion => [1438.8, -286.3, -310.8],
            Nebula::PuppisDarkRegionB => [1348.8, -0.9, -360.8],
            Nebula::RCra => [0.0, -128.4, 399.9],
            Nebula::RedSpiderNebula => [-524.6, 40.1, 2957.5],
            Nebula::RetinaNebula => [1868.0, 811.8, 2202.6],
            Nebula::RhoOphiuchi => [55.5, 148.9, 464.8],
            Nebula::RingNebula => [-1984.6, 556.4, 999.9],
            Nebula::RosetteNebula => [2345.2, -173.1, -4741.3],
            Nebula::RunningManNebula => [587.8, -425.4, -1077.6],
            Nebula::SadrRegion => [-1794.7, 53.7, 365.8],
            Nebula::SaturnNebula => [-2623.4, -2952.8, 3382.1],
            Nebula::ScorpiusDarkRegion => [106.2, 4.3, 480.2],
            Nebula::ScutumDarkRegion => [-272.1, 10.9, 594.0],
            Nebula::SeagullNebula => [2656.4, -159.2, -2712.6],
            Nebula::SkullAndCrossbonesNebula => [13389.4, 105.1, -6765.3],
            Nebula::SkullNebula => [-370.2, -1537.9, -181.7],
            Nebula::Snake => [-17.5, 74.7, 599.1],
            Nebula::SoulNebula => [-5099.3, 116.3, -5497.8],
            Nebula::SpiralPlanetaryNebula => [1415.3, -105.6, 1074.3],
            Nebula::SpirographNebula => [577.9, -452.7, -819.3],
            Nebula::StatueOfLibertyNebula => [5590.8, -70.5, 2180.2],
            Nebula::StruvesLostNebula => [-35.3, -179.7, -463.9],
            Nebula::TaurusDarkRegion => [-71.8, -99.5, -450.3],
            Nebula::ThorsHelmet => [2704.2, -19.2, -2469.3],
            Nebula::TrifidNebula => [-635.8, -25.6, 5163.5],
            Nebula::TrifidOfTheNorth => [-640.4, -400.3, -2487.3],
            Nebula::VeilNebulaEast => [-1914.5, -301.8, 496.8],
            Nebula::VeilNebulaWest => [-1398.3, -193.6, 418.9],
            Nebula::VelaDarkRegion => [991.1, -123.5, -49.0],
            Nebula::VulpeculaDarkRegion => [-543.0, 42.8, 352.0],
            Nebula::WhiteEyedPea => [-3882.1, 7841.0, 8212.6],
            Nebula::WitchHeadNebula => [365.1, -413.8, -711.2],
            Nebula::FlyooGroaSOZE0 => [-26482.4, -78.8, 50335.1],
            Nebula::EollsGraaeAAAH31 => [-18874.9, -607.8, 29979.5],
        }
    }

    /// Calculates the distance to the given coordinates from the center of the nebula.
    pub fn distance_to(&self, other: [f32; 3]) -> GalacticDistance {
        GalacticDistance::between(self.center(), other)
    }

    /// Calculates the closest nebula to the given coordinates.
    pub fn closest_to(pos: [f32; 3]) -> Nebula {
        let mut closest = (
            Nebula::AquilaDarkRegion,
            GalacticDistance::from_ly(f32::MAX),
        );

        for nebula in Nebula::iter() {
            let distance = nebula.distance_to(pos);
            if distance.as_ly() < closest.1.as_ly() {
                closest = (nebula, distance);
            }
        }

        closest.0
    }
}

impl Display for Nebula {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Nebula::AquilaDarkRegion => "Aquila Dark Region",
                Nebula::B133 => "B133",
                Nebula::B352 => "B352",
                Nebula::B92 => "B92",
                Nebula::BarnardsLoop => "Barnard's Loop",
                Nebula::BlinkingNebula => "Blinking Nebula",
                Nebula::BlueFlashNebula => "Blue Flash Nebula",
                Nebula::BluePlanetaryNebula => "Blue Planetary Nebula",
                Nebula::BlueSnowballNebula => "Blue Snowball Nebula",
                Nebula::BowTieNebula => "Bow-Tie Nebula",
                Nebula::BoxNebula => "Box Nebula",
                Nebula::BubbleNebula => "Bubble Nebula",
                Nebula::BugNebula => "Bug Nebula",
                Nebula::ButterflyNebula => "Butterfly Nebula",
                Nebula::CaliforniaNebula => "California Nebula",
                Nebula::CatsEyeNebula => "Cat's Eye Nebula",
                Nebula::CatsPawNebula => "Cat's Paw Nebula",
                Nebula::CaveNebula => "Cave Nebula",
                Nebula::CepheusDarkRegion => "Cepheus Dark Region",
                Nebula::Chamaeleon => "Chamaeleon",
                Nebula::CoalsackDarkRegion => "Coalsack Dark Region",
                Nebula::CocoonNebula => "Cocoon Nebula",
                Nebula::ConeNebula => "Cone Nebula",
                Nebula::CoronaAustrDarkRegion => "Corona Austr. Dark Region",
                Nebula::CrabNebula => "Crab Nebula",
                Nebula::CrescentNebula => "Crescent Nebula",
                Nebula::DumbbellNebula => "Dumbbell Nebula",
                Nebula::EagleNebula => "Eagle Nebula",
                Nebula::EightBurstNebula => "Eight Burst Nebula",
                Nebula::ElephantsTrunkNebula => "Elephant's Trunk Nebula",
                Nebula::EtaCarinaNebula => "Eta Carina Nebula",
                Nebula::FetusNebula => "Fetus Nebula",
                Nebula::FineRingNebula => "Fine Ring Nebula",
                Nebula::FlameNebula => "Flame Nebula",
                Nebula::FlamingStarNebula => "Flaming Star Nebula",
                Nebula::G2DustCloud => "G2 Dust Cloud",
                Nebula::GhostOfJupiter => "Ghost of Jupiter",
                Nebula::HeartNebula => "Heart Nebula",
                Nebula::HelixNebula => "Helix Nebula",
                Nebula::HindNebula => "Hind Nebula",
                Nebula::HorseheadDarkRegion => "Horsehead Dark Region",
                Nebula::HorseheadNebula => "Horsehead Nebula",
                Nebula::HubblesVariable => "Hubble's Variable",
                Nebula::Ic1287 => "IC 1287",
                Nebula::Ic1297 => "IC 1297",
                Nebula::Ic2149 => "IC 2149",
                Nebula::Ic2165 => "IC 2165",
                Nebula::Ic2448 => "IC 2448",
                Nebula::Ic2501 => "IC 2501",
                Nebula::Ic2553 => "IC 2553",
                Nebula::Ic2621 => "IC 2621",
                Nebula::Ic289 => "IC 289",
                Nebula::Ic4191 => "IC 4191",
                Nebula::Ic4604 => "IC 4604",
                Nebula::Ic4634 => "IC 4634",
                Nebula::Ic4663 => "IC 4663",
                Nebula::Ic4673 => "IC 4673",
                Nebula::Ic4699 => "IC 4699",
                Nebula::Ic4776 => "IC 4776",
                Nebula::Ic4846 => "IC 4846",
                Nebula::Ic4997 => "IC 4997",
                Nebula::Ic5117 => "IC 5117",
                Nebula::Ic5148 => "IC 5148",
                Nebula::Ic5217 => "IC 5217",
                Nebula::IrisNebula => "Iris Nebula",
                Nebula::JellyfishNebula => "Jellyfish Nebula",
                Nebula::LagoonNebula => "Lagoon Nebula",
                Nebula::Lbn623 => "LBN 623",
                Nebula::LemonSliceNebula => "Lemon Slice Nebula",
                Nebula::LittleDumbbellNebula => "Little Dumbbell Nebula",
                Nebula::LittleGemNebula => "Little Gem Nebula",
                Nebula::LittleGhostNebula => "Little Ghost Nebula",
                Nebula::LupusDarkRegion => "Lupus Dark Region",
                Nebula::LupusDarkRegionB => "Lupus Dark Region B",
                Nebula::Messier78 => "Messier 78",
                Nebula::MonkeyHeadNebula => "Monkey Head Nebula",
                Nebula::MuscaDarkRegion => "Musca Dark Region",
                Nebula::Ngc1333 => "NGC 1333",
                Nebula::Ngc1360 => "NGC 1360",
                Nebula::Ngc1491 => "NGC 1491",
                Nebula::Ngc1501 => "NGC 1501",
                Nebula::Ngc1514 => "NGC 1514",
                Nebula::Ngc1535 => "NGC 1535",
                Nebula::Ngc1931 => "NGC 1931",
                Nebula::Ngc1999 => "NGC 1999",
                Nebula::Ngc2022 => "NGC 2022",
                Nebula::Ngc2371 => "NGC 2371",
                Nebula::Ngc2392 => "NGC 2392",
                Nebula::Ngc2438 => "NGC 2438",
                Nebula::Ngc2440 => "NGC 2440",
                Nebula::Ngc2452 => "NGC 2452",
                Nebula::Ngc2792 => "NGC 2792",
                Nebula::Ngc281 => "NGC 281",
                Nebula::Ngc2818 => "NGC 2818",
                Nebula::Ngc2867 => "NGC 2867",
                Nebula::Ngc2899 => "NGC 2899",
                Nebula::Ngc3195 => "NGC 3195",
                Nebula::Ngc3199 => "NGC 3199",
                Nebula::Ngc3211 => "NGC 3211",
                Nebula::Ngc3699 => "NGC 3699",
                Nebula::Ngc4361 => "NGC 4361",
                Nebula::Ngc5307 => "NGC 5307",
                Nebula::Ngc5315 => "NGC 5315",
                Nebula::Ngc5367 => "NGC 5367",
                Nebula::Ngc5873 => "NGC 5873",
                Nebula::Ngc5882 => "NGC 5882",
                Nebula::Ngc5979 => "NGC 5979",
                Nebula::Ngc6058 => "NGC 6058",
                Nebula::Ngc6153 => "NGC 6153",
                Nebula::Ngc6188 => "NGC 6188",
                Nebula::Ngc6210 => "NGC 6210",
                Nebula::Ngc6326 => "NGC 6326",
                Nebula::Ngc6337 => "NGC 6337",
                Nebula::Ngc6357 => "NGC 6357",
                Nebula::Ngc6445 => "NGC 6445",
                Nebula::Ngc6563 => "NGC 6563",
                Nebula::Ngc6565 => "NGC 6565",
                Nebula::Ngc6567 => "NGC 6567",
                Nebula::Ngc6572 => "NGC 6572",
                Nebula::Ngc6629 => "NGC 6629",
                Nebula::Ngc6644 => "NGC 6644",
                Nebula::Ngc6751 => "NGC 6751",
                Nebula::Ngc6781 => "NGC 6781",
                Nebula::Ngc6790 => "NGC 6790",
                Nebula::Ngc6803 => "NGC 6803",
                Nebula::Ngc6804 => "NGC 6804",
                Nebula::Ngc6820 => "NGC 6820",
                Nebula::Ngc6842 => "NGC 6842",
                Nebula::Ngc6852 => "NGC 6852",
                Nebula::Ngc6879 => "NGC 6879",
                Nebula::Ngc6884 => "NGC 6884",
                Nebula::Ngc6886 => "NGC 6886",
                Nebula::Ngc6891 => "NGC 6891",
                Nebula::Ngc7026 => "NGC 7026",
                Nebula::Ngc7027 => "NGC 7027",
                Nebula::Ngc7048 => "NGC 7048",
                Nebula::Ngc7354 => "NGC 7354",
                Nebula::Ngc7538 => "NGC 7538",
                Nebula::Ngc7822 => "NGC 7822",
                Nebula::NorthAmericaNebula => "North America Nebula",
                Nebula::OmegaNebula => "Omega Nebula",
                Nebula::OphiuchusDarkRegion => "Ophiuchus Dark Region",
                Nebula::OphiuchusDarkRegionB => "Ophiuchus Dark Region B",
                Nebula::OphiuchusDarkRegionC => "Ophiuchus Dark Region C",
                Nebula::OrionDarkRegion => "Orion Dark Region",
                Nebula::OrionNebula => "Orion Nebula",
                Nebula::OwlNebula => "Owl Nebula",
                Nebula::ParrotsHead => "Parrot's Head",
                Nebula::PelicanNebula => "Pelican Nebula",
                Nebula::PencilNebula => "Pencil Nebula",
                Nebula::PerseusDarkRegion => "Perseus Dark Region",
                Nebula::PhantomStreak => "Phantom Streak",
                Nebula::PipeBowl => "Pipe (Bowl)",
                Nebula::PipeStem => "Pipe (Stem)",
                Nebula::Pleiades => "Pleiades",
                Nebula::PuppisDarkRegion => "Puppis Dark Region",
                Nebula::PuppisDarkRegionB => "Puppis Dark Region B",
                Nebula::RCra => "R Cra",
                Nebula::RedSpiderNebula => "Red Spider Nebula",
                Nebula::RetinaNebula => "Retina Nebula",
                Nebula::RhoOphiuchi => "Rho Ophiuchi",
                Nebula::RingNebula => "Ring Nebula",
                Nebula::RosetteNebula => "Rosette Nebula",
                Nebula::RunningManNebula => "Running Man Nebula",
                Nebula::SadrRegion => "Sadr Region",
                Nebula::SaturnNebula => "Saturn Nebula",
                Nebula::ScorpiusDarkRegion => "Scorpius Dark Region",
                Nebula::ScutumDarkRegion => "Scutum Dark Region",
                Nebula::SeagullNebula => "Seagull Nebula",
                Nebula::SkullAndCrossbonesNebula => "Skull and Crossbones Nebula",
                Nebula::SkullNebula => "Skull Nebula",
                Nebula::Snake => "Snake",
                Nebula::SoulNebula => "Soul Nebula",
                Nebula::SpiralPlanetaryNebula => "Spiral Planetary Nebula",
                Nebula::SpirographNebula => "Spirograph Nebula",
                Nebula::StatueOfLibertyNebula => "Statue of Liberty Nebula",
                Nebula::StruvesLostNebula => "Struve's Lost Nebula",
                Nebula::TaurusDarkRegion => "Taurus Dark Region",
                Nebula::ThorsHelmet => "Thor's Helmet",
                Nebula::TrifidNebula => "Trifid Nebula",
                Nebula::TrifidOfTheNorth => "Trifid of the North",
                Nebula::VeilNebulaEast => "Veil Nebula East",
                Nebula::VeilNebulaWest => "Veil Nebula West",
                Nebula::VelaDarkRegion => "Vela Dark Region",
                Nebula::VulpeculaDarkRegion => "Vulpecula Dark Region",
                Nebula::WhiteEyedPea => "White Eyed Pea",
                Nebula::WitchHeadNebula => "Witch Head Nebula",
                Nebula::FlyooGroaSOZE0 => "Flyoo Groa SO-Z E0",
                Nebula::EollsGraaeAAAH31 => "Eolls Graae AA-A H31",
            }
        )
    }
}
