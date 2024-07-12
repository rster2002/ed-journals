use serde::Deserialize;

/// The current flags for the player. These flags are mostly for things that are related to
/// the player's ship.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Flags(u64);

impl Flags {
    /// Whether the player is currently docked at a station.
    pub fn docked(&self) -> bool {
        self.0 & 1 != 0
    }

    /// Whether the player is landed on a planetary surface.
    pub fn landed(&self) -> bool {
        self.0 & 2 != 0
    }

    /// Whether the landing gear is currently down.
    pub fn landing_gear_down(&self) -> bool {
        self.0 & 4 != 0
    }

    // TODO check what counts as up.
    /// Whether the shields are up.
    pub fn shields_up(&self) -> bool {
        self.0 & 8 != 0
    }

    /// Whether player is currently flying in supercruise.
    pub fn supercruise(&self) -> bool {
        self.0 & 16 != 0
    }

    /// Whether flight assist has been turned off.
    pub fn flight_assist_off(&self) -> bool {
        self.0 & 32 != 0
    }

    // TODO check behaviour in SRV
    /// Whether the hardpoints of the ship are currently deployed.
    pub fn hardpoints_deployed(&self) -> bool {
        self.0 & 64 != 0
    }

    /// Whether the player is in a wing.
    pub fn in_wing(&self) -> bool {
        self.0 & 128 != 0
    }

    /// Whether the lights of the current active vehicle are on.
    pub fn lights_on(&self) -> bool {
        self.0 & 256 != 0
    }

    /// Whether the cargo scoop of the current active vehicle is deployed.
    pub fn cargo_scoop_deployed(&self) -> bool {
        self.0 & 512 != 0
    }

    /// Whether silent running is currently active.
    pub fn silent_running(&self) -> bool {
        self.0 & 1024 != 0
    }

    // TODO figure out if this is also set when the tank is full, but the player is still within range.
    /// Whether the player is currently fuel scooping.
    pub fn scooping_fuel(&self) -> bool {
        self.0 & 2048 != 0
    }

    /// Whether the SRV handbreak is currently on.
    pub fn srv_handbreak(&self) -> bool {
        self.0 & 4096 != 0
    }

    /// Whether the player is currently directly piloting the SRV turret.
    pub fn srv_turret_view(&self) -> bool {
        self.0 & 8192 != 0
    }

    /// Whether the turret of the SRV has been retracted. The turret is retracted when the player is
    /// close to their own ship.
    pub fn srv_turret_retracted(&self) -> bool {
        self.0 & 16384 != 0
    }

    /// Whether the player has enabled drive assist in the SRV.
    pub fn srv_drive_assist(&self) -> bool {
        self.0 & 32768 != 0
    }

    /// Whether the FSD is currently masslocked. This prevents the player from charging the FSD.
    pub fn fsd_masslocked(&self) -> bool {
        self.0 & 65536 != 0
    }

    /// Whether the FSD is currently spinning up. This does not differentiate between charging to
    /// jump to supercruise or hyperspace.
    pub fn fsd_charging(&self) -> bool {
        self.0 & 131072 != 0
    }

    /// Whether the FSD is currently cooling down. This prevents the player from charging the FSD.
    pub fn fsd_cooldown(&self) -> bool {
        self.0 & 262144 != 0
    }

    /// Whether the player's current vehicle is low on fuel, which is when the total fuel reserves
    /// of the vehicle drop below 25%.
    pub fn low_fuel(&self) -> bool {
        self.0 & 524288 != 0
    }

    /// Whether the player's current vehicle is overheating, which is when the heat level exceeds
    /// 100% heat.
    pub fn overheating(&self) -> bool {
        self.0 & 1048576 != 0
    }

    /// Whether the lat/long values are available in the status. Set when near a planet.
    pub fn has_lat_long(&self) -> bool {
        self.0 & 2097152 != 0
    }

    /// Whether the player is counted as 'in danger'. If this is true then the player has to wait
    /// 15 seconds before they can log out of the game.
    pub fn in_danger(&self) -> bool {
        self.0 & 4194304 != 0
    }

    /// Whether the player is currently getting interdicted. This doesn't differentiate between an
    /// attempt by an NPC or a player.
    pub fn being_interdicted(&self) -> bool {
        self.0 & 8388608 != 0
    }

    /// Whether the player is currently piloting the main ship (aka the mother ship.)
    pub fn in_main_ship(&self) -> bool {
        self.0 & 16777216 != 0
    }

    /// Whether the player is currently piloting a fighter.
    pub fn in_fighter(&self) -> bool {
        self.0 & 33554432 != 0
    }

    /// Whether the player is currently piloting an SRV.
    pub fn in_srv(&self) -> bool {
        self.0 & 67108864 != 0
    }

    /// Whether the current cockpit mode is currently set to analysis mode.
    pub fn analysis_mode(&self) -> bool {
        self.0 & 134217728 != 0
    }

    /// Whether night vision is currently active.
    pub fn night_vision(&self) -> bool {
        self.0 & 268435456 != 0
    }

    /// Is set when the average radius of the planet is used for the altitude instead of the
    /// altitude to the actual terrain and elevation of the planet's surface.
    pub fn altitude_from_average_radius(&self) -> bool {
        self.0 & 536870912 != 0
    }

    /// Whether the player is currently jumping through hyperspace.
    pub fn fsd_jump(&self) -> bool {
        self.0 & 1073741824 != 0
    }

    pub fn srv_high_beam(&self) -> bool {
        self.0 & 2147483648 != 0
    }
}
