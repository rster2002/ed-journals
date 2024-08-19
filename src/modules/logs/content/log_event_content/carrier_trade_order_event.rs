use serde::{Deserialize, Serialize};

use crate::mixed::MixedCommodity;

/// Fired when managing a trade order. This is fired for trade orders for both ship commodities and
/// odyssey items.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CarrierTradeOrderEvent {
    /// The id of the carrier that the player deposited fuel to. This is functionally the same as
    /// the market id.
    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,

    /// Whether the trade order is going through the black market.
    pub black_market: bool,

    /// The commodity this trade order is for. Note that this can both be a ship commodity and an
    /// odyssey item.
    pub commodity: MixedCommodity,

    /// The localized name of the commodity for this trade odder.
    #[serde(rename = "Commodity_Localised")]
    pub commodity_localized: Option<String>,

    /// The type of action for this order.
    #[serde(flatten)]
    pub order: CarrierTradeOrderEventOrder,

    /// The set price for the trade order. For purchase orders it is the price per commodity that
    /// has been reserved. For sale orders, this is the price per commodity the carrier is buying
    /// for.
    pub price: Option<u64>,
}

/// The type of action for a given trade order.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CarrierTradeOrderEventOrder {
    /// For when the fleet carrier is buying commodities from other commanders, with the specified
    /// number of commodities to buy.
    PurchaseOrder(u32),

    /// For when the fleet carrier is selling commodities to other commanders, with a specified
    /// number of commodities to sell.
    SaleOrder(u32),

    /// For when cancelling the given trade order. Is always true.
    CancelTrade(bool),
}

impl CarrierTradeOrderEventOrder {
    /// Whether the given order is a purchase order.
    pub fn is_purchase_order(&self) -> bool {
        matches!(self, CarrierTradeOrderEventOrder::PurchaseOrder(_))
    }

    /// Whether the given order is a sell order.
    pub fn is_sale_order(&self) -> bool {
        matches!(self, CarrierTradeOrderEventOrder::SaleOrder(_))
    }

    /// Whether the given order is a cancellation.
    pub fn is_cancel_trade_order(&self) -> bool {
        matches!(self, CarrierTradeOrderEventOrder::CancelTrade(_))
    }
}

#[cfg(test)]
mod tests {
    use crate::logs::content::log_event_content::carrier_trade_order_event::{
        CarrierTradeOrderEvent, CarrierTradeOrderEventOrder,
    };
    use crate::mixed::MixedCommodity;
    use crate::modules::trading::Commodity;
    use crate::odyssey::Item;

    #[test]
    fn carrier_trade_purchase_order_is_parsed_correctly() {
        let result = serde_json::from_str::<CarrierTradeOrderEvent>(
            r#"
            {
                "CarrierID": 3700005632,
                "BlackMarket": false,
                "Commodity": "mineraloil",
                "Commodity_Localised": "Mineral Oil",
                "PurchaseOrder": 70,
                "Price": 228
            }
        "#,
        );

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            CarrierTradeOrderEvent {
                carrier_id: 3700005632,
                black_market: false,
                commodity: MixedCommodity::ShipCommodity(Commodity::MineralOil),
                commodity_localized: Some("Mineral Oil".to_string()),
                order: CarrierTradeOrderEventOrder::PurchaseOrder(70),
                price: Some(228),
            }
        )
    }

    #[test]
    fn carrier_trade_sale_order_is_parsed_correctly() {
        let result = serde_json::from_str::<CarrierTradeOrderEvent>(
            r#"
            {
                "CarrierID": 3700005632,
                "BlackMarket": false,
                "Commodity": "mineraloil",
                "Commodity_Localised": "Mineral Oil",
                "SaleOrder": 70,
                "Price": 228
            }
        "#,
        );

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            CarrierTradeOrderEvent {
                carrier_id: 3700005632,
                black_market: false,
                commodity: MixedCommodity::ShipCommodity(Commodity::MineralOil),
                commodity_localized: Some("Mineral Oil".to_string()),
                order: CarrierTradeOrderEventOrder::SaleOrder(70),
                price: Some(228),
            }
        )
    }

    #[test]
    fn carrier_trade_cancel_trade_order_is_parsed_correctly() {
        let result = serde_json::from_str::<CarrierTradeOrderEvent>(
            r#"
            {
                "CarrierID": 3700005632,
                "BlackMarket": false,
                "Commodity": "mineraloil",
                "Commodity_Localised": "Mineral Oil",
                "CancelTrade": true,
                "Price": 228
            }
        "#,
        );

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            CarrierTradeOrderEvent {
                carrier_id: 3700005632,
                black_market: false,
                commodity: MixedCommodity::ShipCommodity(Commodity::MineralOil),
                commodity_localized: Some("Mineral Oil".to_string()),
                order: CarrierTradeOrderEventOrder::CancelTrade(true),
                price: Some(228),
            }
        )
    }

    #[test]
    fn carrier_trade_purchase_order_for_odyssey_item_is_parsed_correctly() {
        let result = serde_json::from_str::<CarrierTradeOrderEvent>(
            r#"
            {
                "CarrierID": 3700005632,
                "BlackMarket": false,
                "Commodity": "buildingschematic",
                "PurchaseOrder": 70
            }
        "#,
        );

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            CarrierTradeOrderEvent {
                carrier_id: 3700005632,
                black_market: false,
                commodity: MixedCommodity::OdysseyItem(Item::BuildingSchematic),
                commodity_localized: None,
                order: CarrierTradeOrderEventOrder::PurchaseOrder(70),
                price: None,
            }
        )
    }
}
