use serde::{Deserialize, Serialize};

use crate::modules::materials::{Material, MaterialCategory};

/// Fired when the player trades engineering materials at a material trader.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialTradeEvent {
    /// The id of the market where the player has trades materials.
    #[serde(rename = "MarketID")]
    pub market_id: u64,

    /// The kind of material trader where the transaction has taken place.
    pub trader_type: MaterialCategory,

    /// The materials that were paid.
    pub paid: MaterialTradeEventExchange,

    /// The materials received.
    pub received: MaterialTradeEventExchange,
}

/// An entry for a material exchange, either for paying or receiving.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialTradeEventExchange {
    /// The material paid or received.
    pub material: Material,

    /// The localized name of the material that was exchanged.
    #[serde(rename = "Material_Localised")]
    pub material_localized: Option<String>,

    /// The category the exchanged material is a part off.
    pub category: MaterialCategory,

    /// The number of the given material that was either paid or received during the exchange.
    pub quantity: u16,
}
