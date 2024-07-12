use serde::Deserialize;

/// A second flags field which includes flags for the on-foot status of the player.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Flags2(u64);

impl Flags2 {
    /// Whether the current player is currently on-foot.
    pub fn on_foot(&self) -> bool {
        self.0 & 1 != 0
    }

    /// Whether the current player is currently traveling by taxi.
    pub fn in_taxi(&self) -> bool {
        self.0 & 2 != 0
    }

    /// Whether the current player is currently in a multicrew session.
    pub fn in_multicrew(&self) -> bool {
        self.0 & 4 != 0
    }

    /// Whether the player is currently on-foot in a space station.
    pub fn on_foot_in_station(&self) -> bool {
        self.0 & 8 != 0
    }

    /// Whether the player is currently on-foot on a planet.
    pub fn on_foot_on_planet(&self) -> bool {
        self.0 & 16 != 0
    }

    /// Whether the player is currently aiming down the sight of a weapon.
    pub fn aim_down_sight(&self) -> bool {
        self.0 & 32 != 0
    }

    /// Whether the player currently has a low oxygen warning.
    pub fn low_oxygen(&self) -> bool {
        self.0 & 64 != 0
    }

    /// Whether the player currently has a low health.
    pub fn low_health(&self) -> bool {
        self.0 & 128 != 0
    }

    /// Whether the player currently has a cold warning.
    pub fn cold(&self) -> bool {
        self.0 & 256 != 0
    }

    /// Whether the player currently has a heat warning.
    pub fn hot(&self) -> bool {
        self.0 & 512 != 0
    }

    /// Whether the player currently has a very cold warning.
    pub fn very_cold(&self) -> bool {
        self.0 & 1024 != 0
    }

    /// Whether the player currently has an extreme heat warning.
    pub fn very_hot(&self) -> bool {
        self.0 & 2048 != 0
    }

    pub fn glide_mode(&self) -> bool {
        self.0 & 4096 != 0
    }

    /// Whether the player is currently on-foot in the ship hangar.
    pub fn on_foot_in_hangar(&self) -> bool {
        self.0 & 8192 != 0
    }

    /// Whether the player is currently on-foot in the social space in a space station.
    pub fn on_foot_social_space(&self) -> bool {
        self.0 & 16384 != 0
    }

    pub fn on_foot_exterior(&self) -> bool {
        self.0 & 32768 != 0
    }

    /// Whether there is a breathable atmosphere at the current location of the player.
    pub fn breathable_atmosphere(&self) -> bool {
        self.0 & 65536 != 0
    }

    pub fn telepresence_multicrew(&self) -> bool {
        self.0 & 131072 != 0
    }

    pub fn physical_multicrew(&self) -> bool {
        self.0 & 262144 != 0
    }

    /// Whether the FSD of the current ship is charging.
    pub fn fsd_hyperdrive_charging(&self) -> bool {
        self.0 & 524288 != 0
    }
}
