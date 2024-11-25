use crate::try_from_deserialize_impl;
use serde::Serialize;
use std::fmt::{Display, Formatter};
use thiserror::Error;

/// Names of the engineers that are available in the game.
#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum Engineer {
    DidiVatermann,
    BillTurner,
    BrooTarquin,
    TheSarge,
    ZacariahNemo,
    LizRyder,
    HeraTani,
    FelicityFarseer,
    RamTah,
    LeiCheung,
    PetraOlmanova,
    ColonelBrisDekker,
    MarshaHicks,
    ElviraMartuuk,
    TheDweller,
    MarcoQwent,
    SeleneJean,
    ProfessorPalin,
    LoriJameson,
    JuriIshmaak,
    TodTheBlasterMcQuinn,
    TianaFortune,
    MelBrandon,
    EtienneDorn,
    ChloeSedesi,
    JudeNavarro,
    DominoGreen,
    HeroFerrari,
    KitFowler,
    WellingtonBeck,
    TerraVelasquez,
    UmaLaszlo,
    OdenGeiger,
    YardenBond,
    Baltanos,
    EleanorBresa,
    RosaDayette,
    YiShen,

    /// This is a special engineer that does not actually exist, but seems to be used for
    /// pre-engineered modules.
    System,

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    #[serde(untagged)]
    Unknown(String),
}

impl Engineer {
    pub fn id(&self) -> u64 {
        match self {
            Engineer::DidiVatermann => 300000,
            Engineer::BillTurner => 300010,
            Engineer::BrooTarquin => 300030,
            Engineer::TheSarge => 300040,
            Engineer::ZacariahNemo => 300050,
            Engineer::LizRyder => 300080,
            Engineer::HeraTani => 300090,
            Engineer::FelicityFarseer => 300100,
            Engineer::RamTah => 300110,
            Engineer::LeiCheung => 300120,
            Engineer::PetraOlmanova => 300130,
            Engineer::ColonelBrisDekker => 300140,
            Engineer::MarshaHicks => 300150,
            Engineer::ElviraMartuuk => 300160,
            Engineer::TheDweller => 300180,
            Engineer::MarcoQwent => 300200,
            Engineer::SeleneJean => 300210,
            Engineer::ProfessorPalin => 300220,
            Engineer::LoriJameson => 300230,
            Engineer::JuriIshmaak => 300250,
            Engineer::TodTheBlasterMcQuinn => 300260,
            Engineer::TianaFortune => 300270,
            Engineer::MelBrandon => 300280,
            Engineer::EtienneDorn => 300290,
            Engineer::ChloeSedesi => 300300,
            Engineer::JudeNavarro => 400001,
            Engineer::DominoGreen => 400002,
            Engineer::HeroFerrari => 400003,
            Engineer::KitFowler => 400004,
            Engineer::WellingtonBeck => 400005,
            Engineer::TerraVelasquez => 400006,
            Engineer::UmaLaszlo => 400007,
            Engineer::OdenGeiger => 400008,
            Engineer::YardenBond => 400009,
            Engineer::Baltanos => 400010,
            Engineer::EleanorBresa => 400011,
            Engineer::RosaDayette => 400012,
            Engineer::YiShen => 400013,
            Engineer::System => 399999,

            #[cfg(feature = "allow-unknown")]
            Engineer::Unknown(_) => 0,
        }
    }

    pub fn get_system_address(&self) -> u64 {
        match self {
            Engineer::DidiVatermann => 3932277478114,
            Engineer::BillTurner => 1109989017963,
            Engineer::BrooTarquin => 4481966019282,
            Engineer::TheSarge => 2827992680811,
            Engineer::ZacariahNemo => 6131367744226,
            Engineer::LizRyder => 1458309141194,
            Engineer::HeraTani => 1733321102034,
            Engineer::FelicityFarseer => 6681123623626,
            Engineer::RamTah => 3790082132323,
            Engineer::LeiCheung => 4305444669811,
            Engineer::PetraOlmanova => 12274907287851,
            Engineer::ColonelBrisDekker => 10477373803,
            Engineer::MarshaHicks => 48996147307082,
            Engineer::ElviraMartuuk => 3107241104074,
            Engineer::TheDweller => 5031654888146,
            Engineer::MarcoQwent => 121569805492,
            Engineer::SeleneJean => 24859942069665,
            Engineer::ProfessorPalin => 113573366131,
            Engineer::LoriJameson => 3932277478106,
            Engineer::JuriIshmaak => 4481899074282,
            Engineer::TodTheBlasterMcQuinn => 3107576681170,
            Engineer::TianaFortune => 164098653,
            Engineer::MelBrandon => 66038577537618,
            Engineer::EtienneDorn => 11887629902418,
            Engineer::ChloeSedesi => 594676730147,
            Engineer::JudeNavarro => 7268024067513,
            Engineer::DominoGreen => 5068464399785,
            Engineer::HeroFerrari => 7269634614689,
            Engineer::KitFowler => 2827975936355,
            Engineer::WellingtonBeck => 2832832893634,
            Engineer::TerraVelasquez => 3721329101171,
            Engineer::UmaLaszlo => 16065190962585,
            Engineer::OdenGeiger => 8879744226018,
            Engineer::YardenBond => 670686455169,
            Engineer::Baltanos => 71536135676490,
            Engineer::EleanorBresa => 38001031029322,
            Engineer::RosaDayette => 59166629864010,
            Engineer::YiShen => 13736779007129,
            Engineer::System => 0,

            #[cfg(feature = "allow-unknown")]
            Engineer::Unknown(_) => 0,
        }
    }

    pub fn get_market_id(&self) -> u64 {
        match self {
            Engineer::DidiVatermann => 128673927,
            Engineer::BillTurner => 128674183,
            Engineer::BrooTarquin => 128674695,
            Engineer::TheSarge => 128674951,
            Engineer::ZacariahNemo => 128675207,
            Engineer::LizRyder => 128675975,
            Engineer::HeraTani => 128676231,
            Engineer::FelicityFarseer => 128676487,
            Engineer::RamTah => 128676743,
            Engineer::LeiCheung => 128676999,
            Engineer::PetraOlmanova => 128677255,
            Engineer::ColonelBrisDekker => 128677511,
            Engineer::MarshaHicks => 128677767,
            Engineer::ElviraMartuuk => 128678023,
            Engineer::TheDweller => 128678535,
            Engineer::MarcoQwent => 128679047,
            Engineer::SeleneJean => 128679303,
            Engineer::ProfessorPalin => 128679559,
            Engineer::LoriJameson => 128679815,
            Engineer::JuriIshmaak => 128680327,
            Engineer::TodTheBlasterMcQuinn => 128680583,
            Engineer::TianaFortune => 128680839,
            Engineer::MelBrandon => 128681095,
            Engineer::EtienneDorn => 128681351,
            Engineer::ChloeSedesi => 128954244,
            Engineer::JudeNavarro => 128972903,
            Engineer::DominoGreen => 128973159,
            Engineer::HeroFerrari => 128973415,
            Engineer::KitFowler => 128973671,
            Engineer::WellingtonBeck => 128973927,
            Engineer::TerraVelasquez => 128974183,
            Engineer::UmaLaszlo => 128974439,
            Engineer::OdenGeiger => 128974695,
            Engineer::YardenBond => 128974951,
            Engineer::Baltanos => 128986843,
            Engineer::EleanorBresa => 128987099,
            Engineer::RosaDayette => 128986587,
            Engineer::YiShen => 128987355,
            Engineer::System => 0,

            #[cfg(feature = "allow-unknown")]
            Engineer::Unknown(_) => 0,
        }
    }
}

impl Display for Engineer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Engineer::DidiVatermann => "Didi Vatermann",
                Engineer::BillTurner => "Bill Turner",
                Engineer::BrooTarquin => "Broo Tarquin",
                Engineer::TheSarge => "The Sarge",
                Engineer::ZacariahNemo => "Zacariah Nemo",
                Engineer::LizRyder => "Liz Ryder",
                Engineer::HeraTani => "Hera Tani",
                Engineer::FelicityFarseer => "Felicity Farseer",
                Engineer::RamTah => "Ram Tah",
                Engineer::LeiCheung => "Lei Cheung",
                Engineer::PetraOlmanova => "Petra Olmanova",
                Engineer::ColonelBrisDekker => "Colonel Bris Dekker",
                Engineer::MarshaHicks => "Marsha Hicks",
                Engineer::ElviraMartuuk => "Elvira Martuuk",
                Engineer::TheDweller => "The Dweller",
                Engineer::MarcoQwent => "Marco Qwent",
                Engineer::SeleneJean => "Selene Jean",
                Engineer::ProfessorPalin => "Professor Palin",
                Engineer::LoriJameson => "Lori Jameson",
                Engineer::JuriIshmaak => "Juri Ishmaak",
                Engineer::TodTheBlasterMcQuinn => "Tod 'The Blaster' McQuinn",
                Engineer::TianaFortune => "Tiana Fortune",
                Engineer::MelBrandon => "Mel Brandon",
                Engineer::EtienneDorn => "Etienne Dorn",
                Engineer::ChloeSedesi => "Chloe Sedesi",
                Engineer::JudeNavarro => "Jude Navarro",
                Engineer::DominoGreen => "Domino Green",
                Engineer::HeroFerrari => "Hero Ferrari",
                Engineer::KitFowler => "Kit Fowler",
                Engineer::WellingtonBeck => "Wellington Beck",
                Engineer::TerraVelasquez => "Terra Velasquez",
                Engineer::UmaLaszlo => "Uma Laszlo",
                Engineer::OdenGeiger => "Oden Geiger",
                Engineer::YardenBond => "Yarden Bond",
                Engineer::Baltanos => "Baltanos",
                Engineer::EleanorBresa => "Eleanor Bresa",
                Engineer::RosaDayette => "Rosa Dayette",
                Engineer::YiShen => "Yi Shen",
                Engineer::System => "System",

                #[cfg(feature = "allow-unknown")]
                Engineer::Unknown(unknown) => return write!(f, "Unknown engineer: {}", unknown),
            }
        )
    }
}

#[derive(Debug, Error)]
pub enum EngineerError {
    #[error("Unknown engineer for id: {0}")]
    UnknownEngineerId(u64),
}

impl TryFrom<u64> for Engineer {
    type Error = EngineerError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Ok(match value {
            300000 => Engineer::DidiVatermann,
            300010 => Engineer::BillTurner,
            300030 => Engineer::BrooTarquin,
            300040 => Engineer::TheSarge,
            300050 => Engineer::ZacariahNemo,
            300080 => Engineer::LizRyder,
            300090 => Engineer::HeraTani,
            300100 => Engineer::FelicityFarseer,
            300110 => Engineer::RamTah,
            300120 => Engineer::LeiCheung,
            300130 => Engineer::PetraOlmanova,
            300140 => Engineer::ColonelBrisDekker,
            300150 => Engineer::MarshaHicks,
            300160 => Engineer::ElviraMartuuk,
            300180 => Engineer::TheDweller,
            300200 => Engineer::MarcoQwent,
            300210 => Engineer::SeleneJean,
            300220 => Engineer::ProfessorPalin,
            300230 => Engineer::LoriJameson,
            300250 => Engineer::JuriIshmaak,
            300260 => Engineer::TodTheBlasterMcQuinn,
            300270 => Engineer::TianaFortune,
            300280 => Engineer::MelBrandon,
            300290 => Engineer::EtienneDorn,
            300300 => Engineer::ChloeSedesi,
            400001 => Engineer::JudeNavarro,
            400002 => Engineer::DominoGreen,
            400003 => Engineer::HeroFerrari,
            400004 => Engineer::KitFowler,
            400005 => Engineer::WellingtonBeck,
            400006 => Engineer::TerraVelasquez,
            400007 => Engineer::UmaLaszlo,
            400008 => Engineer::OdenGeiger,
            400009 => Engineer::YardenBond,
            400010 => Engineer::Baltanos,
            400011 => Engineer::EleanorBresa,
            400012 => Engineer::RosaDayette,
            400013 => Engineer::YiShen,
            399999 => Engineer::System,

            _ => return Err(EngineerError::UnknownEngineerId(value)),
        })
    }
}

try_from_deserialize_impl!(u64 => Engineer);
