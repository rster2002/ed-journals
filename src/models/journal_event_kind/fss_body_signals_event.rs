use std::str::FromStr;
use serde::Deserialize;
use crate::{from_str_deserialize_impl};

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct FSSBodySignalsEvent {
    pub body_name: String,

    #[serde(rename = "BodyID")]
    pub body_id: u8,
    pub system_address: u64,
    pub signals: Vec<FSSBodySignalEventSignal>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct FSSBodySignalEventSignal {
    #[serde(rename = "Type")]
    pub kind: FSSBodySignalEventSignalType,

    #[serde(rename = "Type_Localised")]
    pub type_localized: String,
    pub count: u8,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum FSSBodySignalEventSignalType {
    Biological,
    Geological,
}

impl FromStr for FSSBodySignalEventSignalType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "$SAA_SignalType_Biological;" => Ok(FSSBodySignalEventSignalType::Biological),
            "$SAA_SignalType_Geological;" => Ok(FSSBodySignalEventSignalType::Geological),
            &_ => Err(s.to_string()),
        }
    }
}

from_str_deserialize_impl!(FSSBodySignalEventSignalType);
