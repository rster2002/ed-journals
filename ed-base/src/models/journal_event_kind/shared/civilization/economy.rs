use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
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

    #[serde(rename = "$economy_None;")]
    None,

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}
