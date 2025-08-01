use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Ord, Hash, PartialOrd, Eq)]
pub enum StarLuminosity {
    O = 25,
    I = 24,
    Ia0 = 23,
    Ia = 22,
    Ib = 21,
    Iab = 20,
    II = 19,
    IIa = 18,
    IIab = 17,
    IIb = 16,
    III = 15,
    IIIa = 14,
    IIIab = 13,
    IIIb = 12,
    IV = 11,
    IVa = 10,
    IVab = 9,
    IVb = 8,
    V = 7,
    Va = 6,
    Vab = 5,
    Vb = 4,
    Vz = 3,
    VI = 2,
    VII = 1,
    #[serde(alias = "0")]
    Zero = 0,
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
