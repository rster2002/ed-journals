use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

/// A type of economy that can be related to a system or station.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Economy {
    #[serde(rename = "$economy_Agri;")]
    Agriculture,

    #[serde(rename = "$economy_Colony;")]
    Colony,

    #[serde(rename = "$economy_Service;")]
    Service,

    #[serde(rename = "$economy_Industrial;")]
    Industrial,

    #[serde(rename = "$economy_Refinery;")]
    Refinery,

    #[serde(rename = "$economy_Extraction;")]
    Extraction,

    #[serde(rename = "$economy_HighTech;")]
    HighTech,

    #[serde(rename = "$economy_Military;")]
    Military,

    #[serde(rename = "$economy_Carrier;")]
    PrivateEnterprise,

    #[serde(rename = "$economy_Tourism;")]
    Tourism,

    #[serde(rename = "$economy_Engineer;")]
    Engineer,

    #[serde(rename = "$economy_Terraforming;")]
    Terraforming,

    #[serde(rename = "$economy_Rescue;")]
    Rescue,

    #[serde(rename = "$economy_Damaged;")]
    Damaged,

    #[serde(rename = "$economy_Repair;")]
    Repair,

    #[serde(rename = "$economy_Prison;")]
    Prison,

    #[serde(rename = "$economy_Undefined;")]
    Undefined,

    #[serde(rename = "$economy_None;")]
    None,

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    #[serde(untagged)]
    Unknown(String),
}

impl Display for Economy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Economy::Agriculture => "Agriculture",
                Economy::Colony => "Colony",
                Economy::Service => "Service",
                Economy::Industrial => "Industrial",
                Economy::Refinery => "Refinery",
                Economy::Extraction => "Extraction",
                Economy::HighTech => "HighTech",
                Economy::Military => "Military",
                Economy::PrivateEnterprise => "Private Enterprise",
                Economy::Tourism => "Tourism",
                Economy::Engineer => "Engineer",
                Economy::Terraforming => "Terraforming",
                Economy::Rescue => "Rescue",
                Economy::Damaged => "Damaged",
                Economy::Repair => "Repair",
                Economy::Prison => "Prison",
                Economy::Undefined => "Undefined",
                Economy::None => "None",

                #[cfg(feature = "allow-unknown")]
                Economy::Unknown(unknown) => return write!(f, "Unknown economy: {unknown}"),
            }
        )
    }
}
