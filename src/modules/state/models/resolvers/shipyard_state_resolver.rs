pub mod ship_entry;

use crate::logs::shipyard_buy_event::ShipyardBuyEventOldShipAction;
use crate::logs::{LogEvent, LogEventContent};
use crate::ship::ShipType;
use crate::state::models::feed_result::FeedResult;
use crate::state::models::resolvers::shipyard_state_resolver::ship_entry::{
    ShipEntry, ShipStatus, TransferringShipStatus,
};
use crate::state::traits::state_resolver::StateResolver;
use serde::Serialize;
use std::collections::HashMap;

/// Keeps track of the player's stored ships.
#[derive(Debug, Default, Serialize)]
pub struct ShipyardStateResolver {
    pub ships: HashMap<u64, ShipEntry>,
}

impl StateResolver<LogEvent> for ShipyardStateResolver {
    fn feed(&mut self, input: &LogEvent) -> FeedResult {
        // Update any transferring ship statuses when the transfer time has been completed.
        for ship in self.ships.values_mut() {
            if let ShipStatus::Transferring(transfer_status) = &ship.status {
                if transfer_status.done_transferring(&input.timestamp) {
                    ship.status = ShipStatus::Stored(transfer_status.to_market_id);
                }
            }
        }

        match &input.content {
            LogEventContent::ShipyardBuy(buy_event) => match &buy_event.old_ship_action {
                ShipyardBuyEventOldShipAction::Store(store_action) => {
                    let ship_entry = self.track_ship(
                        store_action.store_ship_id,
                        store_action.store_old_ship.clone(),
                    );

                    ship_entry.status = ShipStatus::Stored(buy_event.market_id);
                }
                ShipyardBuyEventOldShipAction::Sell(sell_action) => {
                    self.ships.remove(&sell_action.sell_ship_id);
                }
            },
            LogEventContent::ShipyardNew(new_event) => {
                let ship = self.track_ship(new_event.new_ship_id, new_event.ship_type.clone());

                ship.status = ShipStatus::Active;
            }
            LogEventContent::ShipyardSell(sell_event) => {
                self.ships.remove(&sell_event.sell_ship_id);
            }
            LogEventContent::ShipyardSwap(swap_event) => {
                let new_ship = self.track_ship(swap_event.ship_id, swap_event.ship_type.clone());

                new_ship.status = ShipStatus::Active;

                let old_ship =
                    self.track_ship(swap_event.store_ship_id, swap_event.store_old_ship.clone());

                old_ship.status = ShipStatus::Stored(swap_event.market_id);
            }
            LogEventContent::ShipyardTransfer(transfer_event) => {
                let ship =
                    self.track_ship(transfer_event.ship_id, transfer_event.ship_type.clone());

                ship.status = ShipStatus::Transferring(TransferringShipStatus {
                    started_at: input.timestamp,
                    transfer_time: transfer_event.transfer_time,
                    from_market_id: transfer_event.ship_market_id,
                    to_market_id: transfer_event.market_id,
                })
            }
            LogEventContent::StoredShips(stored_ships) => {
                for local_ship in &stored_ships.ships_here {
                    let ship_entry =
                        self.track_ship(local_ship.ship_id, local_ship.ship_type.clone());

                    ship_entry.status = ShipStatus::Stored(stored_ships.market_id);
                }

                for remote_ship in &stored_ships.ships_remote {
                    let ship_entry =
                        self.track_ship(remote_ship.ship_id, remote_ship.ship_type.clone());

                    if let Some(location_information) = &remote_ship.storage_location {
                        ship_entry.status = ShipStatus::Stored(location_information.ship_market_id);
                    }
                }
            }

            _ => return FeedResult::Skipped,
        }

        FeedResult::Accepted
    }
}

impl ShipyardStateResolver {
    fn track_ship(&mut self, ship_id: u64, ship_type: ShipType) -> &mut ShipEntry {
        self.ships.entry(ship_id).or_insert_with(|| ShipEntry {
            ship_type,
            status: ShipStatus::Unknown,
        })
    }
}
