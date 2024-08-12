use serde::{Deserialize, Serialize};

/// Fired when the player deposits tritium to the given carrier.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CarrierDepositFuelEvent {
    /// The id of the carrier that the player deposited fuel to. This is functionally the same as
    /// the market id.
    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,

    /// The number of tonnes of tritium the player deposited to the carrier.
    pub amount: u16,

    /// The total amount of tritium the carrier has in its tank after the deposit.
    pub total: u16,
}
