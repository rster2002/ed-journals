use serde::{Deserialize, Serialize};
use crate::mixed::MixedCommodity;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CarrierTradeOrderEvent {
    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,
    pub black_market: bool,
    pub commodity: MixedCommodity,

    #[serde(rename = "Commodity_Localised")]
    pub commodity_localized: Option<String>,

    #[serde(flatten)]
    pub order: CarrierTradeOrderEventOrder,
    pub price: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CarrierTradeOrderEventOrder {
    PurchaseOrder(u32),
    SaleOrder(u32),
    CancelTrade(bool),
}

impl CarrierTradeOrderEventOrder {
    pub fn is_purchase_order(&self) -> bool {
        matches!(self, CarrierTradeOrderEventOrder::PurchaseOrder(_))
    }

    pub fn is_sale_order(&self) -> bool {
        matches!(self, CarrierTradeOrderEventOrder::SaleOrder(_))
    }

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
