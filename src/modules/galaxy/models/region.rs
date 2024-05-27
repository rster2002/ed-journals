use serde::{Deserialize, Serialize};

use crate::exploration::r#static::{region::REGION_MAP, region_map::REGIONS};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Region {
    Unknown,

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

impl Region {
    pub fn from_pos(pos: [f32; 3]) -> Option<&'static Region> {
        const X0: f32 = -49985.0;
        const Y0: f32 = -40985.0;
        const Z0: f32 = -24105.0;

        let px = ((pos[0] - X0) * 83.0 / 4096.0).floor();
        let pz = ((pos[2] - Z0) * 83.0 / 4096.0).floor();

        if px >= 0.0 && pz >= 0.0 {
            if let Some(row) = REGION_MAP.get(pz as usize) {
                let mut acc = 0;
                let mut pv = 0;

                for &(a, b) in row.iter() {
                    acc += a;
                    if acc as f32 > px {
                        pv = b;
                        break;
                    }
                }

                return match pv {
                    0 => None,
                    _ => Some(&REGIONS[pv as usize]),
                };
            }
        }

        None
    }
}
