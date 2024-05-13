use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum StarLuminosity {
    #[serde(alias = "0")]
    Zero,
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
}
