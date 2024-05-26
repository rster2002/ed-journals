use std::str::FromStr;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::from_str_deserialize_impl;
use crate::modules::odyssey::Item;
use crate::modules::trading::Commodity;

/// Special model for handling scenarios where the input might either be a 'ship' commodity or an
/// Odyssey commodity.
#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum MixedCommodity {
    ShipCommodity(Commodity),
    OdysseyItem(Item),

    #[cfg(not(feature = "strict"))]
    Unknown(String),
}

#[derive(Debug, Error)]
pub enum MixedCommodityError {
    #[error("Unknown mixed commodity: '{0}'")]
    UnknownCommodity(String),
}

impl FromStr for MixedCommodity {
    type Err = MixedCommodityError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ship_commodity = Commodity::from_str(s);

        match ship_commodity {
            #[cfg(not(feature = "strict"))]
            Ok(Commodity::Unknown(_)) => {}
            Ok(commodity) => return Ok(MixedCommodity::ShipCommodity(commodity)),
            Err(_) => {}
        }

        let odyssey_item = Item::from_str(s);

        match odyssey_item {
            #[cfg(not(feature = "strict"))]
            Ok(Item::Unknown(_)) => {}
            Ok(item) => return Ok(MixedCommodity::OdysseyItem(item)),
            Err(_) => {}
        }

        #[cfg(not(feature = "strict"))]
        return Ok(MixedCommodity::Unknown(s.to_string()));

        #[cfg(feature = "strict")]
        return Err(MixedCommodityError::UnknownCommodity(s.to_string()));
    }
}

from_str_deserialize_impl!(MixedCommodity);
