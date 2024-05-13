use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Region {
    #[serde(rename = "$Codex_RegionName_1;")]
    GalacticCenter,

    #[serde(rename = "$Codex_RegionName_2;")]
    EmpyreonStraits,

    #[serde(rename = "$Codex_RegionName_3;")]
    RykersHope,

    #[serde(rename = "$Codex_RegionName_4;")]
    OdinsHold,

    #[serde(rename = "$Codex_RegionName_5;")]
    NormaArm,

    #[serde(rename = "$Codex_RegionName_6;")]
    ArcadianStream,

    #[serde(rename = "$Codex_RegionName_7;")]
    Izanami,

    #[serde(rename = "$Codex_RegionName_8;")]
    InnerOrionPerseusConflux,

    #[serde(rename = "$Codex_RegionName_9;")]
    InnerScutumCentaurusArm,

    #[serde(rename = "$Codex_RegionName_10;")]
    NormaExpanse,

    #[serde(rename = "$Codex_RegionName_11;")]
    TrojanBelt,

    #[serde(rename = "$Codex_RegionName_12;")]
    TheVeils,

    #[serde(rename = "$Codex_RegionName_13;")]
    NewtonsVault,

    #[serde(rename = "$Codex_RegionName_14;")]
    TheConduit,

    #[serde(rename = "$Codex_RegionName_15;")]
    OuterOrionPerseusConflux,

    #[serde(rename = "$Codex_RegionName_16;")]
    OrionCygnusArm,

    #[serde(rename = "$Codex_RegionName_17;")]
    Temple,

    #[serde(rename = "$Codex_RegionName_18;")]
    InnerOrionSpur,

    #[serde(rename = "$Codex_RegionName_19;")]
    HawkingsGap,

    #[serde(rename = "$Codex_RegionName_20;")]
    DrymansPoint,

    #[serde(rename = "$Codex_RegionName_21;")]
    SagittariusCarinaArm,

    #[serde(rename = "$Codex_RegionName_22;")]
    MareSomnia,

    #[serde(rename = "$Codex_RegionName_23;")]
    Acheron,

    #[serde(rename = "$Codex_RegionName_24;")]
    FormorianFrontier,

    #[serde(rename = "$Codex_RegionName_25;")]
    HieronymusDelta,

    #[serde(rename = "$Codex_RegionName_26;")]
    OuterScutumCentaurusArm,

    #[serde(rename = "$Codex_RegionName_27;")]
    OuterArm,

    #[serde(rename = "$Codex_RegionName_28;")]
    AquilasHalo,

    #[serde(rename = "$Codex_RegionName_29;")]
    ErrantMarches,

    #[serde(rename = "$Codex_RegionName_30;")]
    PerseusArm,

    #[serde(rename = "$Codex_RegionName_31;")]
    FormidineRift,

    #[serde(rename = "$Codex_RegionName_32;")]
    VulcanGate,

    #[serde(rename = "$Codex_RegionName_33;")]
    ElysianShore,

    #[serde(rename = "$Codex_RegionName_34;")]
    SanguineousRim,

    #[serde(rename = "$Codex_RegionName_35;")]
    OuterOrionSpur,

    #[serde(rename = "$Codex_RegionName_36;")]
    AchillessAltar,

    #[serde(rename = "$Codex_RegionName_37;")]
    Xibalba,

    #[serde(rename = "$Codex_RegionName_38;")]
    LysasSong,

    #[serde(rename = "$Codex_RegionName_39;")]
    Tenebrae,

    #[serde(rename = "$Codex_RegionName_40;")]
    TheAbyss,

    #[serde(rename = "$Codex_RegionName_41;")]
    KeplersCrest,

    #[serde(rename = "$Codex_RegionName_42;")]
    TheVoid,
}
