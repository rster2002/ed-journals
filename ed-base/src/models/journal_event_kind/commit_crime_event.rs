use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct CommitCrimeEvent {
    pub crime_type: CommitCrimeEventType,
    pub faction: String,
    pub fine: Option<u64>,
    pub bounty: Option<u64>,
}

// TODO this should be moved to its own file
#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
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

    #[serde(rename = "dockingMajorTrespass")]
    MajorTrespass,
    CollidedAtSpeedInNoFireZone,

    #[serde(rename = "CollidedAtSpeedInNoFireZone_HullDamage")]
    CollidedAtSpeedInNoFireZoneHullDamage,

    #[serde(rename = "onFoot_identityTheft")]
    OnFootIdentityTheft,
}
