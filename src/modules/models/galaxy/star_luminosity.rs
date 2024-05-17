use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Ord, PartialOrd, Eq)]
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
}
