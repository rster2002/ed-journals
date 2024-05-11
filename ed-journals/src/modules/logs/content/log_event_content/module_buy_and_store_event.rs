use serde::Deserialize;

use crate::modules::shared::ship::ship_module::ShipModule;
use crate::modules::shared::ship::ship_type::ShipType;

/// When buying a module but storing at the current location instead of equipping it to the current
/// ship.
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ModuleBuyAndStoreEvent {
    /// The purchased module that has been stored at the current location.
    pub buy_item: ShipModule,

    /// Localized name of the module that was purchased. Prefer using the [Display] trait on the
    /// [ShipModule] struct.
    #[serde(rename = "BuyItem_Localised")]
    pub buy_item_localized: Option<String>,

    /// The market ID from where the module was purchased.
    #[serde(rename = "MarketID")]
    pub market_id: u64,

    /// The amount of credits that was spent to purchase the module.
    pub buy_price: u64,

    /// The currently active ship time. It can be the case that the bought module can't even be
    /// equipped to this ship type.
    pub ship: ShipType,

    /// The current active ship, but the bought module was not equipped to this ship. It was just
    /// the ship that was active at the time of purchase.
    #[serde(rename = "ShipID")]
    pub ship_id: u8,
}
