use serde::{Deserialize, Serialize};

/// The kind of crime that can be committed.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Crime {
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
    RecklessWeaponsDischarge,
    PassengerWanted,
    ShuttleDestruction,

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

    #[serde(rename = "collidedAtSpeedInNoFireZone_hulldamage")]
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

    #[serde(rename = "onFoot_breakingAndEntering")]
    OnFootBreakingAndEntering,

    #[serde(rename = "onFoot_carryingIllegalData")]
    OnFootCarryingIllegalData,

    #[serde(rename = "onFoot_carryingStolenGoods")]
    OnFootCarryingStolenGoods,

    #[serde(rename = "onFoot_damagingDefences")]
    OnFootDamagingDefences,

    #[serde(rename = "onFoot_dataTransfer")]
    OnFootDataTransfer,

    #[serde(rename = "onFoot_eBreachUse")]
    OnFootEBreachUse,

    #[serde(rename = "onFoot_overchargeIntent")]
    OnFootOverchargeIntent,

    #[serde(rename = "onfoot_overchargedPort", alias = "onFoot_overchargedPort")]
    OnFootOverchargedPort,

    #[serde(rename = "onFoot_theft")]
    OnFootTheft,

    #[serde(rename = "onFoot_propertyTheft")]
    OnFootPropertyTheft,
}

#[cfg(test)]
mod tests {
    use crate::civilization::Crime;
    use serde_json::Value;

    #[test]
    fn all_crimes_are_parsed_correctly() {
        let content = include_str!("zz_crimes.txt");
        let lines = content.lines();

        for line in lines {
            if line.starts_with('#') {
                continue;
            }

            let result = serde_json::from_value::<Crime>(Value::String(line.to_string()));

            dbg!(&result);
            assert!(result.is_ok());
        }
    }
}
