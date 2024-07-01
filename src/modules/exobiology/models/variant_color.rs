use std::fmt::{Display, Formatter};

use serde::Serialize;
use thiserror::Error;

use crate::modules::exobiology::{Genus, Species, VariantSource};
use crate::modules::galaxy::StarClass;
use crate::modules::materials::Material;

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub enum VariantColor {
    Amethyst,
    Aquamarine,
    Blue,
    Cobalt,
    Cyan,
    Emerald,
    Gold,
    Green,
    Grey,
    Indigo,
    Lime,
    Magenta,
    Maroon,
    Mauve,
    Mulberry,
    Ocher,
    Orange,
    Peach,
    Red,
    Sage,
    Teal,
    Turquoise,
    White,
    Yellow,

    /// For species or genus that do not have color variation.
    None,

    /// This should realistically never happen without changes in the logs.
    #[cfg(not(feature = "strict"))]
    Unknown,
}

#[derive(Debug, Error)]
pub enum VariantColorError {
    #[error("Unknown variant color")]
    UnknownVariant,
}

impl TryFrom<(&Species, &VariantSource)> for VariantColor {
    type Error = VariantColorError;

    fn try_from(value: (&Species, &VariantSource)) -> Result<Self, Self::Error> {
        let genus: Genus = value.0.into();

        Ok(match (genus, value.0, value.1) {
            // Aleoida
            (Genus::Aleoida, _, VariantSource::StarClass(StarClass::B)) => VariantColor::Yellow,
            (Genus::Aleoida, _, VariantSource::StarClass(StarClass::A)) => VariantColor::Green,
            (Genus::Aleoida, _, VariantSource::StarClass(StarClass::F)) => VariantColor::Teal,
            (Genus::Aleoida, _, VariantSource::StarClass(StarClass::K)) => VariantColor::Turquoise,
            (Genus::Aleoida, _, VariantSource::StarClass(StarClass::M)) => VariantColor::Emerald,
            (Genus::Aleoida, _, VariantSource::StarClass(StarClass::L)) => VariantColor::Lime,
            (Genus::Aleoida, _, VariantSource::StarClass(StarClass::T)) => VariantColor::Sage,
            (Genus::Aleoida, _, VariantSource::StarClass(StarClass::TTS)) => VariantColor::Mauve,
            (Genus::Aleoida, _, VariantSource::StarClass(StarClass::Ae)) => VariantColor::Red,
            (Genus::Aleoida, _, VariantSource::StarClass(StarClass::Y)) => VariantColor::Amethyst,
            (Genus::Aleoida, _, VariantSource::StarClass(StarClass::W)) => VariantColor::Grey,
            (Genus::Aleoida, _, VariantSource::StarClass(StarClass::D)) => VariantColor::Indigo,
            (Genus::Aleoida, _, VariantSource::StarClass(StarClass::N)) => VariantColor::Ocher,

            (Genus::AmphoraPlant, _, _) => VariantColor::None,

            (Genus::Anemone, _, _) => VariantColor::None,

            (Genus::BarkMound, _, _) => VariantColor::None,

            // Bacterium
            (_, Species::BacteriumAcies, VariantSource::Material(Material::Antimony)) => {
                VariantColor::Cyan
            }
            (_, Species::BacteriumAcies, VariantSource::Material(Material::Polonium)) => {
                VariantColor::Cyan
            }
            (_, Species::BacteriumAcies, VariantSource::Material(Material::Ruthenium)) => {
                VariantColor::Cobalt
            }
            (_, Species::BacteriumAcies, VariantSource::Material(Material::Technetium)) => {
                VariantColor::Lime
            }
            (_, Species::BacteriumAcies, VariantSource::Material(Material::Tellurium)) => {
                VariantColor::White
            }
            (_, Species::BacteriumAcies, VariantSource::Material(Material::Yttrium)) => {
                VariantColor::Aquamarine
            }

            (
                _,
                Species::BacteriumAlcyoneum | Species::BacteriumAurasus | Species::BacteriumCerbrus,
                VariantSource::StarClass(StarClass::O),
            ) => VariantColor::Turquoise,
            (
                _,
                Species::BacteriumAlcyoneum | Species::BacteriumAurasus | Species::BacteriumCerbrus,
                VariantSource::StarClass(StarClass::B),
            ) => VariantColor::Grey,
            (
                _,
                Species::BacteriumAlcyoneum | Species::BacteriumAurasus | Species::BacteriumCerbrus,
                VariantSource::StarClass(StarClass::A),
            ) => VariantColor::Yellow,
            (
                _,
                Species::BacteriumAlcyoneum | Species::BacteriumAurasus | Species::BacteriumCerbrus,
                VariantSource::StarClass(StarClass::F),
            ) => VariantColor::Lime,
            (
                _,
                Species::BacteriumAlcyoneum | Species::BacteriumAurasus | Species::BacteriumCerbrus,
                VariantSource::StarClass(StarClass::G),
            ) => VariantColor::Emerald,
            (
                _,
                Species::BacteriumAlcyoneum | Species::BacteriumAurasus | Species::BacteriumCerbrus,
                VariantSource::StarClass(StarClass::K),
            ) => VariantColor::Green,
            (
                _,
                Species::BacteriumAlcyoneum | Species::BacteriumAurasus | Species::BacteriumCerbrus,
                VariantSource::StarClass(StarClass::M),
            ) => VariantColor::Teal,
            (
                _,
                Species::BacteriumAlcyoneum | Species::BacteriumAurasus | Species::BacteriumCerbrus,
                VariantSource::StarClass(StarClass::L),
            ) => VariantColor::Sage,
            (
                _,
                Species::BacteriumAlcyoneum | Species::BacteriumAurasus | Species::BacteriumCerbrus,
                VariantSource::StarClass(StarClass::T),
            ) => VariantColor::Red,
            (
                _,
                Species::BacteriumAlcyoneum | Species::BacteriumAurasus | Species::BacteriumCerbrus,
                VariantSource::StarClass(StarClass::TTS),
            ) => VariantColor::Maroon,
            (
                _,
                Species::BacteriumAlcyoneum | Species::BacteriumAurasus | Species::BacteriumCerbrus,
                VariantSource::StarClass(StarClass::W),
            ) => VariantColor::Amethyst,
            (
                _,
                Species::BacteriumAlcyoneum | Species::BacteriumAurasus | Species::BacteriumCerbrus,
                VariantSource::StarClass(StarClass::D),
            ) => VariantColor::Ocher,
            (
                _,
                Species::BacteriumAlcyoneum | Species::BacteriumAurasus | Species::BacteriumCerbrus,
                VariantSource::StarClass(StarClass::N),
            ) => VariantColor::Indigo,
            (
                _,
                Species::BacteriumAlcyoneum | Species::BacteriumAurasus | Species::BacteriumCerbrus,
                VariantSource::StarClass(StarClass::Y),
            ) => VariantColor::Mauve,
            (
                _,
                Species::BacteriumAlcyoneum | Species::BacteriumAurasus | Species::BacteriumCerbrus,
                VariantSource::StarClass(StarClass::Ae),
            ) => VariantColor::Orange,

            (_, Species::BacteriumBullaris, VariantSource::Material(Material::Antimony)) => {
                VariantColor::Cobalt
            }
            (_, Species::BacteriumBullaris, VariantSource::Material(Material::Polonium)) => {
                VariantColor::Yellow
            }
            (_, Species::BacteriumBullaris, VariantSource::Material(Material::Ruthenium)) => {
                VariantColor::Aquamarine
            }
            (_, Species::BacteriumBullaris, VariantSource::Material(Material::Technetium)) => {
                VariantColor::Gold
            }
            (_, Species::BacteriumBullaris, VariantSource::Material(Material::Tellurium)) => {
                VariantColor::Lime
            }
            (_, Species::BacteriumBullaris, VariantSource::Material(Material::Yttrium)) => {
                VariantColor::Red
            }

            (_, Species::BacteriumInformem, VariantSource::Material(Material::Antimony)) => {
                VariantColor::Red
            }
            (_, Species::BacteriumInformem, VariantSource::Material(Material::Polonium)) => {
                VariantColor::Lime
            }
            (_, Species::BacteriumInformem, VariantSource::Material(Material::Ruthenium)) => {
                VariantColor::Gold
            }
            (_, Species::BacteriumInformem, VariantSource::Material(Material::Technetium)) => {
                VariantColor::Aquamarine
            }
            (_, Species::BacteriumInformem, VariantSource::Material(Material::Tellurium)) => {
                VariantColor::Yellow
            }
            (_, Species::BacteriumInformem, VariantSource::Material(Material::Yttrium)) => {
                VariantColor::Cobalt
            }

            (_, Species::BacteriumNebulus, VariantSource::Material(Material::Antimony)) => {
                VariantColor::Magenta
            }
            (_, Species::BacteriumNebulus, VariantSource::Material(Material::Polonium)) => {
                VariantColor::Gold
            }
            (_, Species::BacteriumNebulus, VariantSource::Material(Material::Ruthenium)) => {
                VariantColor::Orange
            }
            (_, Species::BacteriumNebulus, VariantSource::Material(Material::Tellurium)) => {
                VariantColor::Green
            }
            (_, Species::BacteriumNebulus, VariantSource::Material(Material::Yttrium)) => {
                VariantColor::Cobalt
            }
            (_, Species::BacteriumNebulus, VariantSource::Material(Material::Technetium)) => {
                VariantColor::Cyan
            }

            (_, Species::BacteriumOmentum, VariantSource::Material(Material::Boron)) => {
                VariantColor::Lime
            }
            (_, Species::BacteriumOmentum, VariantSource::Material(Material::Cadmium)) => {
                VariantColor::Lime
            }
            (_, Species::BacteriumOmentum, VariantSource::Material(Material::Mercury)) => {
                VariantColor::White
            }
            (_, Species::BacteriumOmentum, VariantSource::Material(Material::Molybdenum)) => {
                VariantColor::Aquamarine
            }
            (_, Species::BacteriumOmentum, VariantSource::Material(Material::Niobium)) => {
                VariantColor::Peach
            }
            (_, Species::BacteriumOmentum, VariantSource::Material(Material::Tin)) => {
                VariantColor::Red
            }
            (_, Species::BacteriumOmentum, VariantSource::Material(Material::Tungsten)) => {
                VariantColor::Blue
            }

            (_, Species::BacteriumScopulum, VariantSource::Material(Material::Boron)) => {
                VariantColor::White
            }
            (_, Species::BacteriumScopulum, VariantSource::Material(Material::Cadmium)) => {
                VariantColor::White
            }
            (_, Species::BacteriumScopulum, VariantSource::Material(Material::Mercury)) => {
                VariantColor::Peach
            }
            (_, Species::BacteriumScopulum, VariantSource::Material(Material::Molybdenum)) => {
                VariantColor::Lime
            }
            (_, Species::BacteriumScopulum, VariantSource::Material(Material::Niobium)) => {
                VariantColor::Red
            }
            (_, Species::BacteriumScopulum, VariantSource::Material(Material::Tin)) => {
                VariantColor::Mulberry
            }
            (_, Species::BacteriumScopulum, VariantSource::Material(Material::Tungsten)) => {
                VariantColor::Aquamarine
            }

            (_, Species::BacteriumTela, VariantSource::Material(Material::Boron)) => {
                VariantColor::Gold
            }
            (_, Species::BacteriumTela, VariantSource::Material(Material::Cadmium)) => {
                VariantColor::Gold
            }
            (_, Species::BacteriumTela, VariantSource::Material(Material::Mercury)) => {
                VariantColor::Orange
            }
            (_, Species::BacteriumTela, VariantSource::Material(Material::Molybdenum)) => {
                VariantColor::Yellow
            }
            (_, Species::BacteriumTela, VariantSource::Material(Material::Niobium)) => {
                VariantColor::Magenta
            }
            (_, Species::BacteriumTela, VariantSource::Material(Material::Tin)) => {
                VariantColor::Cobalt
            }
            (_, Species::BacteriumTela, VariantSource::Material(Material::Tungsten)) => {
                VariantColor::Green
            }

            (_, Species::BacteriumVerrata, VariantSource::Material(Material::Boron)) => {
                VariantColor::Peach
            }
            (_, Species::BacteriumVerrata, VariantSource::Material(Material::Cadmium)) => {
                VariantColor::Peach
            }
            (_, Species::BacteriumVerrata, VariantSource::Material(Material::Mercury)) => {
                VariantColor::Red
            }
            (_, Species::BacteriumVerrata, VariantSource::Material(Material::Molybdenum)) => {
                VariantColor::White
            }
            (_, Species::BacteriumVerrata, VariantSource::Material(Material::Niobium)) => {
                VariantColor::Mulberry
            }
            (_, Species::BacteriumVerrata, VariantSource::Material(Material::Tin)) => {
                VariantColor::Blue
            }
            (_, Species::BacteriumVerrata, VariantSource::Material(Material::Tungsten)) => {
                VariantColor::Lime
            }

            (_, Species::BacteriumVesicula, VariantSource::Material(Material::Antimony)) => {
                VariantColor::Cyan
            }
            (_, Species::BacteriumVesicula, VariantSource::Material(Material::Polonium)) => {
                VariantColor::Orange
            }
            (_, Species::BacteriumVesicula, VariantSource::Material(Material::Ruthenium)) => {
                VariantColor::Mulberry
            }
            (_, Species::BacteriumVesicula, VariantSource::Material(Material::Technetium)) => {
                VariantColor::Gold
            }
            (_, Species::BacteriumVesicula, VariantSource::Material(Material::Tellurium)) => {
                VariantColor::Red
            }
            (_, Species::BacteriumVesicula, VariantSource::Material(Material::Yttrium)) => {
                VariantColor::Lime
            }

            (_, Species::BacteriumVolu, VariantSource::Material(Material::Antimony)) => {
                VariantColor::Red
            }
            (_, Species::BacteriumVolu, VariantSource::Material(Material::Polonium)) => {
                VariantColor::Aquamarine
            }
            (_, Species::BacteriumVolu, VariantSource::Material(Material::Ruthenium)) => {
                VariantColor::Cobalt
            }
            (_, Species::BacteriumVolu, VariantSource::Material(Material::Tellurium)) => {
                VariantColor::Cyan
            }
            (_, Species::BacteriumVolu, VariantSource::Material(Material::Yttrium)) => {
                VariantColor::Gold
            }
            (_, Species::BacteriumVolu, VariantSource::Material(Material::Technetium)) => {
                VariantColor::Lime
            }

            (Genus::BrainTree, _, _) => VariantColor::None,

            // TODO figure out the remaining colors. They can be one of Emerald, Lime, or Maroon
            (Genus::Cactoida, _, VariantSource::StarClass(StarClass::A)) => VariantColor::Green,
            // Ae
            // B
            (Genus::Cactoida, _, VariantSource::StarClass(StarClass::D)) => VariantColor::Turquoise,
            (Genus::Cactoida, _, VariantSource::StarClass(StarClass::F)) => VariantColor::Yellow,
            (Genus::Cactoida, _, VariantSource::StarClass(StarClass::G)) => VariantColor::Teal,
            // K
            (Genus::Cactoida, _, VariantSource::StarClass(StarClass::M)) => VariantColor::Amethyst,
            (Genus::Cactoida, _, VariantSource::StarClass(StarClass::L)) => VariantColor::Mauve,
            (Genus::Cactoida, _, VariantSource::StarClass(StarClass::N)) => VariantColor::Sage,
            (Genus::Cactoida, _, VariantSource::StarClass(StarClass::O)) => VariantColor::Grey,
            (Genus::Cactoida, _, VariantSource::StarClass(StarClass::TTS)) => VariantColor::Red,
            (Genus::Cactoida, _, VariantSource::StarClass(StarClass::T)) => VariantColor::Orange,
            (Genus::Cactoida, _, VariantSource::StarClass(StarClass::W)) => VariantColor::Indigo,
            (Genus::Cactoida, _, VariantSource::StarClass(StarClass::Y)) => VariantColor::Ocher,

            // TODO figure out the remaining colors. They can be one of Emerald, Indigo, or Red
            (Genus::Clypeus, _, VariantSource::StarClass(StarClass::A)) => VariantColor::Orange,
            // Ae
            (Genus::Clypeus, _, VariantSource::StarClass(StarClass::B)) => VariantColor::Maroon,
            (Genus::Clypeus, _, VariantSource::StarClass(StarClass::D)) => VariantColor::Lime,
            (Genus::Clypeus, _, VariantSource::StarClass(StarClass::F)) => VariantColor::Mauve,
            (Genus::Clypeus, _, VariantSource::StarClass(StarClass::G)) => VariantColor::Amethyst,
            (Genus::Clypeus, _, VariantSource::StarClass(StarClass::K)) => VariantColor::Grey,
            (Genus::Clypeus, _, VariantSource::StarClass(StarClass::L)) => VariantColor::Teal,
            (Genus::Clypeus, _, VariantSource::StarClass(StarClass::M)) => VariantColor::Turquoise,
            (Genus::Clypeus, _, VariantSource::StarClass(StarClass::N)) => VariantColor::Yellow,
            // O
            // W
            (Genus::Clypeus, _, VariantSource::StarClass(StarClass::Y)) => VariantColor::Green,

            (
                _,
                Species::ConchaAureolas | Species::ConchaLabiata,
                VariantSource::StarClass(StarClass::A),
            ) => VariantColor::Teal,
            (
                _,
                Species::ConchaAureolas | Species::ConchaLabiata,
                VariantSource::StarClass(StarClass::F),
            ) => VariantColor::Grey,
            (
                _,
                Species::ConchaAureolas | Species::ConchaLabiata,
                VariantSource::StarClass(StarClass::G),
            ) => VariantColor::Turquoise,
            (
                _,
                Species::ConchaAureolas | Species::ConchaLabiata,
                VariantSource::StarClass(StarClass::K),
            ) => VariantColor::Red,
            (
                _,
                Species::ConchaAureolas | Species::ConchaLabiata,
                VariantSource::StarClass(StarClass::L),
            ) => VariantColor::Orange,
            (
                _,
                Species::ConchaAureolas | Species::ConchaLabiata,
                VariantSource::StarClass(StarClass::N),
            ) => VariantColor::Emerald,
            (
                _,
                Species::ConchaAureolas | Species::ConchaLabiata,
                VariantSource::StarClass(StarClass::B),
            ) => VariantColor::Indigo,
            (
                _,
                Species::ConchaAureolas | Species::ConchaLabiata,
                VariantSource::StarClass(StarClass::Y),
            ) => VariantColor::Yellow,
            (
                _,
                Species::ConchaAureolas | Species::ConchaLabiata,
                VariantSource::StarClass(StarClass::D),
            ) => VariantColor::Green,
            (
                _,
                Species::ConchaAureolas | Species::ConchaLabiata,
                VariantSource::StarClass(StarClass::W),
            ) => VariantColor::Lime,

            (_, Species::ConchaBiconcavis, VariantSource::Material(Material::Antimony)) => {
                VariantColor::Peach
            }
            (_, Species::ConchaBiconcavis, VariantSource::Material(Material::Ruthenium)) => {
                VariantColor::Orange
            }
            (_, Species::ConchaBiconcavis, VariantSource::Material(Material::Tellurium)) => {
                VariantColor::Yellow
            }
            (_, Species::ConchaBiconcavis, VariantSource::Material(Material::Polonium)) => {
                VariantColor::Red
            }
            (_, Species::ConchaBiconcavis, VariantSource::Material(Material::Yttrium)) => {
                VariantColor::Gold
            }
            (_, Species::ConchaBiconcavis, VariantSource::Material(Material::Technetium)) => {
                VariantColor::White
            }

            (_, Species::ConchaRenibus, VariantSource::Material(Material::Boron)) => {
                VariantColor::Red
            }
            (_, Species::ConchaRenibus, VariantSource::Material(Material::Cadmium)) => {
                VariantColor::Red
            }
            (_, Species::ConchaRenibus, VariantSource::Material(Material::Mercury)) => {
                VariantColor::Mulberry
            }
            (_, Species::ConchaRenibus, VariantSource::Material(Material::Molybdenum)) => {
                VariantColor::Peach
            }
            (_, Species::ConchaRenibus, VariantSource::Material(Material::Niobium)) => {
                VariantColor::Blue
            }
            (_, Species::ConchaRenibus, VariantSource::Material(Material::Tin)) => {
                VariantColor::Aquamarine
            }
            (_, Species::ConchaRenibus, VariantSource::Material(Material::Tungsten)) => {
                VariantColor::White
            }

            (Genus::CrystallineShards, _, _) => VariantColor::None,

            (_, Species::ElectricaePluma, VariantSource::Material(Material::Antimony)) => {
                VariantColor::Cobalt
            }
            (_, Species::ElectricaePluma, VariantSource::Material(Material::Polonium)) => {
                VariantColor::Cyan
            }
            (_, Species::ElectricaePluma, VariantSource::Material(Material::Ruthenium)) => {
                VariantColor::Blue
            }
            (_, Species::ElectricaePluma, VariantSource::Material(Material::Technetium)) => {
                VariantColor::Magenta
            }
            (_, Species::ElectricaePluma, VariantSource::Material(Material::Tellurium)) => {
                VariantColor::Red
            }
            (_, Species::ElectricaePluma, VariantSource::Material(Material::Yttrium)) => {
                VariantColor::Mulberry
            }

            (_, Species::ElectricaeRadialem, VariantSource::Material(Material::Antimony)) => {
                VariantColor::Cyan
            }
            (_, Species::ElectricaeRadialem, VariantSource::Material(Material::Polonium)) => {
                VariantColor::Cobalt
            }
            (_, Species::ElectricaeRadialem, VariantSource::Material(Material::Ruthenium)) => {
                VariantColor::Blue
            }
            (_, Species::ElectricaeRadialem, VariantSource::Material(Material::Technetium)) => {
                VariantColor::Aquamarine
            }
            (_, Species::ElectricaeRadialem, VariantSource::Material(Material::Tellurium)) => {
                VariantColor::Magenta
            }
            (_, Species::ElectricaeRadialem, VariantSource::Material(Material::Yttrium)) => {
                VariantColor::Green
            }

            (Genus::Fonticulua, _, VariantSource::StarClass(StarClass::B)) => VariantColor::Lime,
            (Genus::Fonticulua, _, VariantSource::StarClass(StarClass::A)) => VariantColor::Green,
            (Genus::Fonticulua, _, VariantSource::StarClass(StarClass::F)) => VariantColor::Yellow,
            (Genus::Fonticulua, _, VariantSource::StarClass(StarClass::G)) => VariantColor::Teal,
            (Genus::Fonticulua, _, VariantSource::StarClass(StarClass::K)) => VariantColor::Emerald,
            (Genus::Fonticulua, _, VariantSource::StarClass(StarClass::M)) => {
                VariantColor::Amethyst
            }
            (Genus::Fonticulua, _, VariantSource::StarClass(StarClass::L)) => VariantColor::Mauve,
            (Genus::Fonticulua, _, VariantSource::StarClass(StarClass::T)) => VariantColor::Orange,
            (Genus::Fonticulua, _, VariantSource::StarClass(StarClass::TTS)) => VariantColor::Red,
            (Genus::Fonticulua, _, VariantSource::StarClass(StarClass::Y)) => VariantColor::Ocher,
            (Genus::Fonticulua, _, VariantSource::StarClass(StarClass::D)) => {
                VariantColor::Turquoise
            }
            (Genus::Fonticulua, _, VariantSource::StarClass(StarClass::N)) => VariantColor::Sage,
            (Genus::Fonticulua, _, VariantSource::StarClass(StarClass::Ae)) => {
                VariantColor::Maroon
            },
            (Genus::Fonticulua, _, VariantSource::StarClass(StarClass::O)) => {
                VariantColor::Grey
            },
            (Genus::Fonticulua, _, VariantSource::StarClass(StarClass::W)) => {
                VariantColor::Indigo
            },

            (Genus::Frutexa, _, VariantSource::StarClass(StarClass::Y)) => VariantColor::Amethyst,
            (Genus::Frutexa, _, VariantSource::StarClass(StarClass::G)) => VariantColor::Emerald,
            (Genus::Frutexa, _, VariantSource::StarClass(StarClass::F)) => VariantColor::Green,
            (Genus::Frutexa, _, VariantSource::StarClass(StarClass::M)) => VariantColor::Grey,
            (Genus::Frutexa, _, VariantSource::StarClass(StarClass::D)) => VariantColor::Indigo,
            (Genus::Frutexa, _, VariantSource::StarClass(StarClass::B)) => VariantColor::Lime,
            (Genus::Frutexa, _, VariantSource::StarClass(StarClass::TTS)) => VariantColor::Mauve,
            (Genus::Frutexa, _, VariantSource::StarClass(StarClass::W)) => VariantColor::Orange,
            (Genus::Frutexa, _, VariantSource::StarClass(StarClass::N)) => VariantColor::Red,
            (Genus::Frutexa, _, VariantSource::StarClass(StarClass::L)) => VariantColor::Teal,
            (Genus::Frutexa, _, VariantSource::StarClass(StarClass::O)) => VariantColor::Yellow,

            (_, Species::FumerolaAquatis, VariantSource::Material(Material::Boron)) => {
                VariantColor::Green
            }
            (_, Species::FumerolaAquatis, VariantSource::Material(Material::Cadmium)) => {
                VariantColor::Green
            }
            (_, Species::FumerolaAquatis, VariantSource::Material(Material::Mercury)) => {
                VariantColor::Yellow
            }
            (_, Species::FumerolaAquatis, VariantSource::Material(Material::Molybdenum)) => {
                VariantColor::Cyan
            }
            (_, Species::FumerolaAquatis, VariantSource::Material(Material::Niobium)) => {
                VariantColor::Gold
            }
            (_, Species::FumerolaAquatis, VariantSource::Material(Material::Tin)) => {
                VariantColor::Orange
            }
            (_, Species::FumerolaAquatis, VariantSource::Material(Material::Tungsten)) => {
                VariantColor::Cobalt
            }

            (_, Species::FumerolaCarbosis, VariantSource::Material(Material::Boron)) => {
                VariantColor::Orange
            }
            (_, Species::FumerolaCarbosis, VariantSource::Material(Material::Cadmium)) => {
                VariantColor::Orange
            }
            (_, Species::FumerolaCarbosis, VariantSource::Material(Material::Mercury)) => {
                VariantColor::Magenta
            }
            (_, Species::FumerolaCarbosis, VariantSource::Material(Material::Molybdenum)) => {
                VariantColor::Gold
            }
            (_, Species::FumerolaCarbosis, VariantSource::Material(Material::Niobium)) => {
                VariantColor::Cobalt
            }
            (_, Species::FumerolaCarbosis, VariantSource::Material(Material::Tin)) => {
                VariantColor::Cyan
            }
            (_, Species::FumerolaCarbosis, VariantSource::Material(Material::Tungsten)) => {
                VariantColor::Yellow
            }

            (_, Species::FumerolaExtremus, VariantSource::Material(Material::Boron)) => {
                VariantColor::Aquamarine
            }
            (_, Species::FumerolaExtremus, VariantSource::Material(Material::Cadmium)) => {
                VariantColor::Aquamarine
            }
            (_, Species::FumerolaExtremus, VariantSource::Material(Material::Mercury)) => {
                VariantColor::Lime
            }
            (_, Species::FumerolaExtremus, VariantSource::Material(Material::Molybdenum)) => {
                VariantColor::Blue
            }
            (_, Species::FumerolaExtremus, VariantSource::Material(Material::Niobium)) => {
                VariantColor::White
            }
            (_, Species::FumerolaExtremus, VariantSource::Material(Material::Tin)) => {
                VariantColor::Peach
            }
            (_, Species::FumerolaExtremus, VariantSource::Material(Material::Tungsten)) => {
                VariantColor::Mulberry
            }

            (_, Species::FumerolaNitris, VariantSource::Material(Material::Boron)) => {
                VariantColor::White
            }
            (_, Species::FumerolaNitris, VariantSource::Material(Material::Cadmium)) => {
                VariantColor::White
            }
            (_, Species::FumerolaNitris, VariantSource::Material(Material::Mercury)) => {
                VariantColor::Peach
            }
            (_, Species::FumerolaNitris, VariantSource::Material(Material::Molybdenum)) => {
                VariantColor::Lime
            }
            (_, Species::FumerolaNitris, VariantSource::Material(Material::Niobium)) => {
                VariantColor::Red
            }
            (_, Species::FumerolaNitris, VariantSource::Material(Material::Tin)) => {
                VariantColor::Mulberry
            }
            (_, Species::FumerolaNitris, VariantSource::Material(Material::Tungsten)) => {
                VariantColor::Aquamarine
            }

            (_, Species::FungoidaBullarum, VariantSource::Material(Material::Antimony)) => {
                VariantColor::Red
            }
            (_, Species::FungoidaBullarum, VariantSource::Material(Material::Polonium)) => {
                VariantColor::Mulberry
            }
            (_, Species::FungoidaBullarum, VariantSource::Material(Material::Ruthenium)) => {
                VariantColor::Magenta
            }
            (_, Species::FungoidaBullarum, VariantSource::Material(Material::Technetium)) => {
                VariantColor::Peach
            }
            (_, Species::FungoidaBullarum, VariantSource::Material(Material::Tellurium)) => {
                VariantColor::Gold
            }
            (_, Species::FungoidaBullarum, VariantSource::Material(Material::Yttrium)) => {
                VariantColor::Orange
            }

            (_, Species::FungoidaGelata, VariantSource::Material(Material::Boron)) => {
                VariantColor::Cyan
            }
            (_, Species::FungoidaGelata, VariantSource::Material(Material::Cadmium)) => {
                VariantColor::Cyan
            }
            (_, Species::FungoidaGelata, VariantSource::Material(Material::Mercury)) => {
                VariantColor::Lime
            }
            (_, Species::FungoidaGelata, VariantSource::Material(Material::Molybdenum)) => {
                VariantColor::Mulberry
            }
            (_, Species::FungoidaGelata, VariantSource::Material(Material::Niobium)) => {
                VariantColor::Green
            }
            (_, Species::FungoidaGelata, VariantSource::Material(Material::Tin)) => {
                VariantColor::Red
            }
            (_, Species::FungoidaGelata, VariantSource::Material(Material::Tungsten)) => {
                VariantColor::Orange
            }

            (_, Species::FungoidaSetisis, VariantSource::Material(Material::Antimony)) => {
                VariantColor::Peach
            }
            (_, Species::FungoidaSetisis, VariantSource::Material(Material::Polonium)) => {
                VariantColor::White
            }
            (_, Species::FungoidaSetisis, VariantSource::Material(Material::Ruthenium)) => {
                VariantColor::Gold
            }
            (_, Species::FungoidaSetisis, VariantSource::Material(Material::Technetium)) => {
                VariantColor::Lime
            }
            (_, Species::FungoidaSetisis, VariantSource::Material(Material::Tellurium)) => {
                VariantColor::Yellow
            }
            (_, Species::FungoidaSetisis, VariantSource::Material(Material::Yttrium)) => {
                VariantColor::Orange
            }

            (_, Species::FungoidaStabitis, VariantSource::Material(Material::Boron)) => {
                VariantColor::Blue
            }
            (_, Species::FungoidaStabitis, VariantSource::Material(Material::Cadmium)) => {
                VariantColor::Blue
            }
            (_, Species::FungoidaStabitis, VariantSource::Material(Material::Mercury)) => {
                VariantColor::Green
            }
            (_, Species::FungoidaStabitis, VariantSource::Material(Material::Molybdenum)) => {
                VariantColor::Magenta
            }
            (_, Species::FungoidaStabitis, VariantSource::Material(Material::Niobium)) => {
                VariantColor::White
            }
            (_, Species::FungoidaStabitis, VariantSource::Material(Material::Tin)) => {
                VariantColor::Orange
            }
            (_, Species::FungoidaStabitis, VariantSource::Material(Material::Tungsten)) => {
                VariantColor::Peach
            }

            (_, Species::OsseusCornibus, VariantSource::StarClass(StarClass::A)) => VariantColor::Lime,
            (_, Species::OsseusCornibus, VariantSource::StarClass(StarClass::Ae)) => VariantColor::Orange, // This is a guess
            (_, Species::OsseusCornibus, VariantSource::StarClass(StarClass::F)) => VariantColor::Turquoise,
            (_, Species::OsseusCornibus, VariantSource::StarClass(StarClass::G)) => VariantColor::Grey,
            (_, Species::OsseusCornibus, VariantSource::StarClass(StarClass::K)) => VariantColor::Indigo,
            (_, Species::OsseusCornibus, VariantSource::StarClass(StarClass::T)) => VariantColor::Emerald,
            (_, Species::OsseusCornibus, VariantSource::StarClass(StarClass::O)) => VariantColor::Emerald, // This is a guess
            (_, Species::OsseusCornibus, VariantSource::StarClass(StarClass::TTS)) => VariantColor::Green,
            (_, Species::OsseusCornibus, VariantSource::StarClass(StarClass::Y)) => VariantColor::Maroon,

            (_, Species::OsseusFractus, VariantSource::StarClass(StarClass::A)) => VariantColor::Lime,
            (_, Species::OsseusFractus, VariantSource::StarClass(StarClass::Ae)) => VariantColor::Orange, // This is a guess
            (_, Species::OsseusFractus, VariantSource::StarClass(StarClass::F)) => VariantColor::Turquoise,
            (_, Species::OsseusFractus, VariantSource::StarClass(StarClass::G)) => VariantColor::Grey,
            (_, Species::OsseusFractus, VariantSource::StarClass(StarClass::K)) => VariantColor::Indigo,
            (_, Species::OsseusFractus, VariantSource::StarClass(StarClass::T)) => VariantColor::Emerald,
            (_, Species::OsseusFractus, VariantSource::StarClass(StarClass::O)) => VariantColor::Yellow, // This is a guess
            (_, Species::OsseusFractus, VariantSource::StarClass(StarClass::TTS)) => VariantColor::Green,
            (_, Species::OsseusFractus, VariantSource::StarClass(StarClass::Y)) => VariantColor::Maroon,

            (_, Species::OsseusPellebantus, VariantSource::StarClass(StarClass::A)) => VariantColor::Lime,
            (_, Species::OsseusPellebantus, VariantSource::StarClass(StarClass::Ae)) => VariantColor::Orange, // This is a guess
            (_, Species::OsseusPellebantus, VariantSource::StarClass(StarClass::F)) => VariantColor::Turquoise,
            (_, Species::OsseusPellebantus, VariantSource::StarClass(StarClass::G)) => VariantColor::Grey,
            (_, Species::OsseusPellebantus, VariantSource::StarClass(StarClass::K)) => VariantColor::Indigo,
            (_, Species::OsseusPellebantus, VariantSource::StarClass(StarClass::T)) => VariantColor::Emerald,
            (_, Species::OsseusPellebantus, VariantSource::StarClass(StarClass::O)) => VariantColor::Yellow, // This is a guess
            (_, Species::OsseusPellebantus, VariantSource::StarClass(StarClass::TTS)) => VariantColor::Green,
            (_, Species::OsseusPellebantus, VariantSource::StarClass(StarClass::Y)) => VariantColor::Maroon,

            (_, Species::OsseusSpiralis, VariantSource::StarClass(StarClass::A)) => VariantColor::Lime,
            (_, Species::OsseusSpiralis, VariantSource::StarClass(StarClass::Ae)) => VariantColor::Orange,
            (_, Species::OsseusSpiralis, VariantSource::StarClass(StarClass::F)) => VariantColor::Turquoise,
            (_, Species::OsseusSpiralis, VariantSource::StarClass(StarClass::G)) => VariantColor::Grey,
            (_, Species::OsseusSpiralis, VariantSource::StarClass(StarClass::K)) => VariantColor::Indigo,
            (_, Species::OsseusSpiralis, VariantSource::StarClass(StarClass::T)) => VariantColor::Emerald,
            (_, Species::OsseusSpiralis, VariantSource::StarClass(StarClass::TTS)) => VariantColor::Green,
            (_, Species::OsseusSpiralis, VariantSource::StarClass(StarClass::O)) => VariantColor::Yellow,
            (_, Species::OsseusSpiralis, VariantSource::StarClass(StarClass::Y)) => VariantColor::Maroon,


            (_, Species::OsseusDiscus, VariantSource::Material(Material::Boron)) => {
                VariantColor::White
            }
            (_, Species::OsseusDiscus, VariantSource::Material(Material::Cadmium)) => {
                VariantColor::White
            }
            (_, Species::OsseusDiscus, VariantSource::Material(Material::Mercury)) => {
                VariantColor::Lime
            }
            (_, Species::OsseusDiscus, VariantSource::Material(Material::Molybdenum)) => {
                VariantColor::Peach
            }
            (_, Species::OsseusDiscus, VariantSource::Material(Material::Niobium)) => {
                VariantColor::Aquamarine
            }
            (_, Species::OsseusDiscus, VariantSource::Material(Material::Tin)) => {
                VariantColor::Blue
            }
            (_, Species::OsseusDiscus, VariantSource::Material(Material::Tungsten)) => {
                VariantColor::Red
            }

            (_, Species::OsseusPumice, VariantSource::Material(Material::Antimony)) => {
                VariantColor::White
            }
            (_, Species::OsseusPumice, VariantSource::Material(Material::Polonium)) => {
                VariantColor::Peach
            }
            (_, Species::OsseusPumice, VariantSource::Material(Material::Ruthenium)) => {
                VariantColor::Gold
            }
            (_, Species::OsseusPumice, VariantSource::Material(Material::Technetium)) => {
                VariantColor::Lime
            }
            (_, Species::OsseusPumice, VariantSource::Material(Material::Tellurium)) => {
                VariantColor::Green
            }
            (_, Species::OsseusPumice, VariantSource::Material(Material::Yttrium)) => {
                VariantColor::Yellow
            }

            (_, Species::ReceptaConditivus, VariantSource::Material(Material::Yttrium)) => {
                VariantColor::Green
            }
            (_, Species::ReceptaConditivus, VariantSource::Material(Material::Antimony)) => {
                VariantColor::Lime
            }
            (_, Species::ReceptaConditivus, VariantSource::Material(Material::Polonium)) => {
                VariantColor::White
            }
            (_, Species::ReceptaConditivus, VariantSource::Material(Material::Ruthenium)) => {
                VariantColor::Yellow
            }
            (_, Species::ReceptaConditivus, VariantSource::Material(Material::Tellurium)) => {
                VariantColor::Cyan
            }
            (_, Species::ReceptaConditivus, VariantSource::Material(Material::Technetium)) => {
                VariantColor::Aquamarine
            }

            (_, Species::ReceptaDeltahedronix, VariantSource::Material(Material::Boron)) => {
                VariantColor::Lime
            }
            (_, Species::ReceptaDeltahedronix, VariantSource::Material(Material::Cadmium)) => {
                VariantColor::Lime
            }
            (_, Species::ReceptaDeltahedronix, VariantSource::Material(Material::Mercury)) => {
                VariantColor::Cyan
            }
            (_, Species::ReceptaDeltahedronix, VariantSource::Material(Material::Molybdenum)) => {
                VariantColor::Gold
            }
            (_, Species::ReceptaDeltahedronix, VariantSource::Material(Material::Niobium)) => {
                VariantColor::Mulberry
            }
            (_, Species::ReceptaDeltahedronix, VariantSource::Material(Material::Tin)) => {
                VariantColor::Orange
            }
            (_, Species::ReceptaDeltahedronix, VariantSource::Material(Material::Tungsten)) => {
                VariantColor::Red
            }

            // TODO figure out O and W. Possible remaining colors: Green and Indigo
            // O
            // W
            (_, Species::ReceptaUmbrux, VariantSource::StarClass(StarClass::L)) => {
                VariantColor::Ocher
            }
            (_, Species::ReceptaUmbrux, VariantSource::StarClass(StarClass::TTS)) => {
                VariantColor::Sage
            }
            (_, Species::ReceptaUmbrux, VariantSource::StarClass(StarClass::T)) => {
                VariantColor::Teal
            }
            (_, Species::ReceptaUmbrux, VariantSource::StarClass(StarClass::B)) => {
                VariantColor::Turquoise
            }
            (_, Species::ReceptaUmbrux, VariantSource::StarClass(StarClass::M)) => {
                VariantColor::Maroon
            }
            (_, Species::ReceptaUmbrux, VariantSource::StarClass(StarClass::A)) => {
                VariantColor::Amethyst
            }
            (_, Species::ReceptaUmbrux, VariantSource::StarClass(StarClass::F)) => {
                VariantColor::Mauve
            }
            (_, Species::ReceptaUmbrux, VariantSource::StarClass(StarClass::G)) => {
                VariantColor::Orange
            }
            (_, Species::ReceptaUmbrux, VariantSource::StarClass(StarClass::K)) => {
                VariantColor::Red
            }
            (_, Species::ReceptaUmbrux, VariantSource::StarClass(StarClass::D)) => {
                VariantColor::Yellow
            }
            (_, Species::ReceptaUmbrux, VariantSource::StarClass(StarClass::Y)) => {
                VariantColor::Lime
            }
            (_, Species::ReceptaUmbrux, VariantSource::StarClass(StarClass::N)) => {
                VariantColor::Emerald
            }
            (_, Species::ReceptaUmbrux, VariantSource::StarClass(StarClass::Ae)) => {
                VariantColor::Grey
            }

            (Genus::Stratum, _, VariantSource::StarClass(StarClass::F)) => VariantColor::Emerald,
            (Genus::Stratum, _, VariantSource::StarClass(StarClass::K)) => VariantColor::Lime,
            (Genus::Stratum, _, VariantSource::StarClass(StarClass::M)) => VariantColor::Green,
            (Genus::Stratum, _, VariantSource::StarClass(StarClass::L)) => VariantColor::Turquoise,
            (Genus::Stratum, _, VariantSource::StarClass(StarClass::T)) => VariantColor::Grey,
            (Genus::Stratum, _, VariantSource::StarClass(StarClass::TTS)) => VariantColor::Amethyst,
            (Genus::Stratum, _, VariantSource::StarClass(StarClass::D)) => VariantColor::Mauve,
            (Genus::Stratum, _, VariantSource::StarClass(StarClass::Ae)) => VariantColor::Teal,
            (Genus::Stratum, _, VariantSource::StarClass(StarClass::Y)) => VariantColor::Indigo,
            (Genus::Stratum, _, VariantSource::StarClass(StarClass::W)) => VariantColor::Red,

            (Genus::Tubus, _, VariantSource::StarClass(StarClass::B)) => VariantColor::Emerald,
            (Genus::Tubus, _, VariantSource::StarClass(StarClass::A)) => VariantColor::Indigo,
            (Genus::Tubus, _, VariantSource::StarClass(StarClass::F)) => VariantColor::Grey,
            (Genus::Tubus, _, VariantSource::StarClass(StarClass::G)) => VariantColor::Red,
            (Genus::Tubus, _, VariantSource::StarClass(StarClass::K)) => VariantColor::Maroon,
            (Genus::Tubus, _, VariantSource::StarClass(StarClass::M)) => VariantColor::Teal,
            (Genus::Tubus, _, VariantSource::StarClass(StarClass::L)) => VariantColor::Turquoise,
            (Genus::Tubus, _, VariantSource::StarClass(StarClass::T)) => VariantColor::Mauve,
            (Genus::Tubus, _, VariantSource::StarClass(StarClass::TTS)) => VariantColor::Ocher,
            (Genus::Tubus, _, VariantSource::StarClass(StarClass::N)) => VariantColor::Amethyst,
            (Genus::Tubus, _, VariantSource::StarClass(StarClass::D)) => VariantColor::Yellow,
            (Genus::Tubus, _, VariantSource::StarClass(StarClass::O)) => VariantColor::Green,
            (Genus::Tubus, _, VariantSource::StarClass(StarClass::W)) => VariantColor::Lime,

            // TODO add Ae and TTS. Possible colors: Amethyst and Turquoise
            // Ae
            (Genus::Tussock, _, VariantSource::StarClass(StarClass::D)) => VariantColor::Maroon,
            (Genus::Tussock, _, VariantSource::StarClass(StarClass::F)) => VariantColor::Yellow,
            (Genus::Tussock, _, VariantSource::StarClass(StarClass::G)) => VariantColor::Lime,
            (Genus::Tussock, _, VariantSource::StarClass(StarClass::K)) => VariantColor::Green,
            (Genus::Tussock, _, VariantSource::StarClass(StarClass::L)) => VariantColor::Sage,
            (Genus::Tussock, _, VariantSource::StarClass(StarClass::M)) => VariantColor::Emerald,
            // TTS
            (Genus::Tussock, _, VariantSource::StarClass(StarClass::T)) => VariantColor::Teal,
            (Genus::Tussock, _, VariantSource::StarClass(StarClass::W)) => VariantColor::Orange,
            (Genus::Tussock, _, VariantSource::StarClass(StarClass::Y)) => VariantColor::Red,

            (Genus::SinuousTubers, _, _) => VariantColor::None,

            #[cfg(not(feature = "strict"))]
            (_, _, _) => VariantColor::Unknown,

            #[cfg(feature = "strict")]
            (_, _, _) => {
                #[cfg(test)]
                dbg!(value);
                return Err(VariantColorError::UnknownVariant);
            }
        })
    }
}

impl Display for VariantColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                VariantColor::Amethyst => "Amethyst",
                VariantColor::Aquamarine => "Aquamarine",
                VariantColor::Blue => "Blue",
                VariantColor::Cobalt => "Cobalt",
                VariantColor::Cyan => "Cyan",
                VariantColor::Emerald => "Emerald",
                VariantColor::Gold => "Gold",
                VariantColor::Green => "Green",
                VariantColor::Grey => "Grey",
                VariantColor::Indigo => "Indigo",
                VariantColor::Lime => "Lime",
                VariantColor::Magenta => "Magenta",
                VariantColor::Maroon => "Maroon",
                VariantColor::Mauve => "Mauve",
                VariantColor::Mulberry => "Mulberry",
                VariantColor::Ocher => "Ocher",
                VariantColor::Orange => "Orange",
                VariantColor::Peach => "Peach",
                VariantColor::Red => "Red",
                VariantColor::Sage => "Sage",
                VariantColor::Teal => "Teal",
                VariantColor::Turquoise => "Turquoise",
                VariantColor::White => "White",
                VariantColor::Yellow => "Yellow",
                VariantColor::None => "None",

                #[cfg(not(feature = "strict"))]
                VariantColor::Unknown => "Unknown",
            }
        )
    }
}
