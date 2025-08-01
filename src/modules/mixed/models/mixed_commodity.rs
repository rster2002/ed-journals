use std::str::FromStr;

use serde::Serialize;
use thiserror::Error;

use crate::from_str_deserialize_impl;
use crate::modules::odyssey::Item;
use crate::modules::trading::Commodity;

/// Special model for handling scenarios where the input might either be a 'ship' commodity or an
/// Odyssey commodity.
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum MixedCommodity {
    ShipCommodity(Commodity),
    OdysseyItem(Item),

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
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
            #[cfg(feature = "allow-unknown")]
            Ok(Commodity::Unknown(_)) => {}
            Ok(commodity) => return Ok(MixedCommodity::ShipCommodity(commodity)),
            Err(_) => {}
        }

        let odyssey_item = Item::from_str(s);

        match odyssey_item {
            #[cfg(feature = "allow-unknown")]
            Ok(Item::Unknown(_)) => {}
            Ok(item) => return Ok(MixedCommodity::OdysseyItem(item)),
            Err(_) => {}
        }

        #[cfg(feature = "allow-unknown")]
        return Ok(MixedCommodity::Unknown(s.to_string()));

        #[cfg(not(feature = "allow-unknown"))]
        return Err(MixedCommodityError::UnknownCommodity(s.to_string()));
    }
}

from_str_deserialize_impl!(MixedCommodity);
