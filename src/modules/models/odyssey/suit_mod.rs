use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum SuitMod {
    #[serde(alias = "suit_reducedtoolbatteryconsumption")]
    ReducedToolBatteryConsumption,

    #[serde(alias = "suit_increasedbatterycapacity")]
    IncreasedBatteryCapacity,

    #[serde(alias = "suit_increasedsprintduration")]
    IncreasedSprintDuration,
}
