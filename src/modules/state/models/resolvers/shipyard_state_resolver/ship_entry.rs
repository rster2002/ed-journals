use crate::ship::ShipType;
use chrono::{DateTime, TimeDelta, Utc};
use serde::Serialize;
use std::ops::Add;

#[derive(Debug, Serialize)]
pub struct ShipEntry {
    pub ship_type: ShipType,
    pub status: ShipStatus,
}

/// The status of the ship.
#[derive(Debug, Serialize)]
pub enum ShipStatus {
    /// The current status of the ship is not known.
    Unknown,

    /// The ship is the current active ship for the player.
    Active,

    /// The ship is currently stored.
    Stored(u64),

    /// The ship is currently being transferred between two locations.
    Transferring(TransferringShipStatus),
}

/// Information about the current transfer status.
#[derive(Debug, Serialize)]
pub struct TransferringShipStatus {
    /// The timestamp when the transfer was initiated.
    pub started_at: DateTime<Utc>,

    /// The total transfer time duration in seconds.
    pub transfer_time: u64,

    /// The market id the ship is transferred from.
    pub from_market_id: u64,

    /// The market id the ship is transferred to.
    pub to_market_id: u64,
}

impl TransferringShipStatus {
    pub fn completed_at(&self) -> DateTime<Utc> {
        self.started_at
            .add(TimeDelta::new((self.transfer_time as i64).clamp(0, 10000000), 0).unwrap())
    }

    pub fn done_transferring(&self, timestamp: &DateTime<Utc>) -> bool {
        &self.completed_at() <= timestamp
    }
}
