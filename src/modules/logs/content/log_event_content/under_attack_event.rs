use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct UnderAttackEvent {
    // TODO when is this empty?
    pub target: Option<UnderAttackEventTarget>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub enum UnderAttackEventTarget {
    You,
    Fighter,
    Mothership,
}
