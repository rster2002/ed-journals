use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CommitCrimeEvent {
    pub crime_type: CommitCrimeEventType,
    pub faction: String,
    pub fine: Option<u64>,
    pub bounty: Option<u64>,
}

// TODO this should be moved to its own file
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum CommitCrimeEventType {
    Assault,
    Murder,
    Piracy,
    Interdiction,
    IllegalCargo,
    DisobeyPolice,
    FireInNoFireZone,
    FireInStation,
    DumpingDangerous,
    DumpingNearStation,

    #[serde(rename = "dockingMinorBlockingAirlock")]
    MinorBlockingAirlock,

    #[serde(rename = "dockingMajorBlockingAirlock")]
    MayorBlockingAirlock,

    #[serde(rename = "dockingMinorBlockingLandingPad")]
    MinorBlockingLandingPad,

    #[serde(rename = "dockingMajorBlockingLandingPad")]
    MajorBlockingLandingPad,

    #[serde(rename = "dockingMinorTresspass")]
    MinorTrespass,

    #[serde(rename = "dockingMajorTresspass")]
    MajorTrespass,
    CollidedAtSpeedInNoFireZone,

    #[serde(rename = "CollidedAtSpeedInNoFireZone_HullDamage")]
    CollidedAtSpeedInNoFireZoneHullDamage,

    #[serde(rename = "onFoot_profileCloningIntent")]
    OnFootIdentityTheftCaught,

    #[serde(rename = "onFoot_murder")]
    OnFootMurder,

    #[serde(rename = "onFoot_identityTheft")]
    OnFootIdentityTheft,

    #[serde(rename = "onFoot_recklessEndangerment")]
    OnFootRecklessEndangerment,

    #[serde(rename = "onFoot_trespass")]
    OnFootTrespassing,

    #[serde(rename = "onFoot_detectionOfWeapon")]
    OnFootDetectionOfWeapon,

    #[serde(rename = "onFoot_failureToSubmitToPolice")]
    OnFootFailureToSubmitToPolice,

    #[serde(rename = "onFoot_carryingIllegalGoods")]
    OnFootCarryingIllegalGoods,

    #[serde(rename = "onFoot_arcCutterUse")]
    OnFootArcCutterUse,

    #[serde(rename = "onFoot_propertyTheft")]
    OnFootPropertyTheft,
}
