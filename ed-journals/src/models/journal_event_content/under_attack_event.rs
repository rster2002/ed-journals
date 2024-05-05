use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct UnderAttackEvent {
    // TODO when is this empty?
    pub target: Option<UnderAttackEventTarget>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum UnderAttackEventTarget {
    You,
    Fighter,
    Mothership,
}
