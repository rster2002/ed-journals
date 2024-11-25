use std::cmp::Ordering;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum StarLuminosity {
    O,
    I,
    Ia0,
    Ia,
    Ib,
    Iab,
    II,
    IIa,
    IIab,
    IIb,
    III,
    IIIa,
    IIIab,
    IIIb,
    IV,
    IVa,
    IVab,
    IVb,
    V,
    Va,
    Vab,
    Vb,
    Vz,
    VI,
    VII,

    #[serde(alias = "0")]
    Zero,

    #[serde(untagged)]
    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    Unknown(String),
}

impl StarLuminosity {
    fn order_number(&self) -> u8 {
        match self {
            StarLuminosity::O => 26,
            StarLuminosity::I => 25,
            StarLuminosity::Ia0 => 24,
            StarLuminosity::Ia => 23,
            StarLuminosity::Ib => 22,
            StarLuminosity::Iab => 21,
            StarLuminosity::II => 20,
            StarLuminosity::IIa => 19,
            StarLuminosity::IIab => 18,
            StarLuminosity::IIb => 17,
            StarLuminosity::III => 16,
            StarLuminosity::IIIa => 15,
            StarLuminosity::IIIab => 14,
            StarLuminosity::IIIb => 13,
            StarLuminosity::IV => 12,
            StarLuminosity::IVa => 11,
            StarLuminosity::IVab => 10,
            StarLuminosity::IVb => 9,
            StarLuminosity::V => 8,
            StarLuminosity::Va => 7,
            StarLuminosity::Vab => 6,
            StarLuminosity::Vb => 5,
            StarLuminosity::Vz => 4,
            StarLuminosity::VI => 3,
            StarLuminosity::VII => 2,
            StarLuminosity::Zero => 1,

            #[cfg(feature = "allow-unknown")]
            StarLuminosity::Unknown(_) => 0,
        }
    }
}

impl PartialEq<Self> for StarLuminosity {
    fn eq(&self, other: &Self) -> bool {
        self.order_number() == other.order_number()
    }
}

impl Eq for StarLuminosity {}

impl PartialOrd<Self> for StarLuminosity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.order_number().cmp(&other.order_number()))
    }
}

impl Ord for StarLuminosity {
    fn cmp(&self, other: &Self) -> Ordering {
        self.order_number().cmp(&other.order_number())
    }
}

impl Hash for StarLuminosity {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            #[cfg(feature = "allow-unknown")]
            StarLuminosity::Unknown(unknown) => unknown.hash(state),
            _ => self.order_number().hash(state),
        }
    }
}

impl Display for StarLuminosity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                StarLuminosity::O => "O",
                StarLuminosity::I => "I",
                StarLuminosity::Ia0 => "Ia0",
                StarLuminosity::Ia => "Ia",
                StarLuminosity::Ib => "Ib",
                StarLuminosity::Iab => "Iab",
                StarLuminosity::II => "II",
                StarLuminosity::IIa => "IIa",
                StarLuminosity::IIab => "IIab",
                StarLuminosity::IIb => "IIb",
                StarLuminosity::III => "III",
                StarLuminosity::IIIa => "IIIa",
                StarLuminosity::IIIab => "IIIab",
                StarLuminosity::IIIb => "IIIb",
                StarLuminosity::IV => "IV",
                StarLuminosity::IVa => "IVa",
                StarLuminosity::IVab => "IVab",
                StarLuminosity::IVb => "IVb",
                StarLuminosity::V => "V",
                StarLuminosity::Va => "Va",
                StarLuminosity::Vab => "Vab",
                StarLuminosity::Vb => "Vb",
                StarLuminosity::Vz => "Vz",
                StarLuminosity::VI => "VI",
                StarLuminosity::VII => "VII",
                StarLuminosity::Zero => "Zero",

                #[cfg(feature = "allow-unknown")]
                StarLuminosity::Unknown(unknown) => return write!(f, "Unknown luminosity: {}", unknown),
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_luminosity_comparison() {
        assert!(StarLuminosity::O > StarLuminosity::I);
        assert!(StarLuminosity::I > StarLuminosity::Ia0);
        assert!(StarLuminosity::Ia0 > StarLuminosity::Ia);
        assert!(StarLuminosity::Ia > StarLuminosity::Ib);
        assert!(StarLuminosity::Ib > StarLuminosity::Iab);
        assert!(StarLuminosity::Iab > StarLuminosity::II);
        assert!(StarLuminosity::II > StarLuminosity::IIa);
        assert!(StarLuminosity::IIa > StarLuminosity::IIab);
        assert!(StarLuminosity::IIab > StarLuminosity::IIb);
        assert!(StarLuminosity::IIb > StarLuminosity::III);
        assert!(StarLuminosity::III > StarLuminosity::IIIa);
        assert!(StarLuminosity::IIIa > StarLuminosity::IIIab);
        assert!(StarLuminosity::IIIab > StarLuminosity::IIIb);
        assert!(StarLuminosity::IIIb > StarLuminosity::IV);
        assert!(StarLuminosity::IV > StarLuminosity::IVa);
        assert!(StarLuminosity::IVa > StarLuminosity::IVab);
        assert!(StarLuminosity::IVab > StarLuminosity::IVb);
        assert!(StarLuminosity::IVb > StarLuminosity::V);
        assert!(StarLuminosity::V > StarLuminosity::Va);
        assert!(StarLuminosity::Va > StarLuminosity::Vab);
        assert!(StarLuminosity::Vab > StarLuminosity::Vb);
        assert!(StarLuminosity::Vb > StarLuminosity::Vz);
        assert!(StarLuminosity::Vz > StarLuminosity::VI);
        assert!(StarLuminosity::VI > StarLuminosity::VII);
        assert!(StarLuminosity::VII > StarLuminosity::Zero);
    }
}
