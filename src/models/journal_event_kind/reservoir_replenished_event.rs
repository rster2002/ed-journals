use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct ReservoirReplenishedEvent {
    pub fuel_main: f32,
    pub fuel_reservoir: f32,
}
