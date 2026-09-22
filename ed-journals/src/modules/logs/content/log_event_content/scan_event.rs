use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

use crate::galaxy::{Gravity, LocalDistance, PlanetComposition};
use crate::modules::galaxy::{
    Atmosphere, AtmosphereElement, AtmosphereType, OrbitInfo, PlanetClass, RingClass, StarClass,
    StarLuminosity, TerraformState, Volcanism,
};
use crate::modules::materials::Material;

/// Fired when information about a body within a system is scanned.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ScanEvent {
    /// The kind of scan that was performed.
    pub scan_type: ScanEventScanType,

    /// The name of the body.
    pub body_name: String,

    /// The id of the body within the system.
    #[serde(rename = "BodyID")]
    pub body_id: u8,

    /// Parents the body orbits, going from the most direct parent to the least direct parent.
    #[serde(default)]
    pub parents: Vec<ScanEventParent>,

    /// The name of the system the body is in.
    pub star_system: String,

    /// The address of the system the body is in.
    pub system_address: u64,

    /// The distance from the jump-in point.
    #[serde(rename = "DistanceFromArrivalLS")]
    pub distance_from_arrival: LocalDistance,

    /// Whether the body has been discovered before.
    #[serde(default)]
    pub was_discovered: bool,

    /// Whether the body has been mapped before.
    #[serde(default)]
    pub was_mapped: bool,

    /// Detailed information about the body per type of body.
    #[serde(flatten)]
    pub kind: ScanEventKind,
}

/// The type of scan that was performed.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum ScanEventScanType {
    Basic,
    NavBeaconDetail,
    AutoScan,
    Detailed,
}

impl ScanEventScanType {
    pub fn is_basic(&self) -> bool {
        matches!(self, ScanEventScanType::Basic)
    }

    pub fn is_nav_beacon_detail(&self) -> bool {
        matches!(self, ScanEventScanType::NavBeaconDetail)
    }

    pub fn is_auto_scan(&self) -> bool {
        matches!(self, ScanEventScanType::AutoScan)
    }

    pub fn is_detailed(&self) -> bool {
        matches!(self, ScanEventScanType::Detailed)
    }
}

#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum ScanEventKind {
    Star(ScanEventStar),
    Planet(ScanEventPlanet),
    BeltCluster(ScanEventBeltCluster),
}

impl ScanEventKind {
    pub fn is_star(&self) -> bool {
        matches!(self, ScanEventKind::Star(_))
    }

    /// If the body is a star, return the star-specific details, otherwise [None].
    pub fn star(&self) -> Option<&ScanEventStar> {
        if let ScanEventKind::Star(star) = self {
            return Some(star);
        }

        None
    }

    pub fn is_planet(&self) -> bool {
        matches!(self, ScanEventKind::Planet(_))
    }

    /// If the body is a star, return the planet-specific details, otherwise [None].
    pub fn planet(&self) -> Option<&ScanEventPlanet> {
        if let ScanEventKind::Planet(planet) = self {
            return Some(planet);
        }

        None
    }

    pub fn is_belt_cluster(&self) -> bool {
        matches!(self, ScanEventKind::BeltCluster(_))
    }

    /// If the body is a belt cluster, return the belt cluster-specific details, otherwise [None].
    pub fn belt_cluster(&self) -> Option<&ScanEventBeltCluster> {
        if let ScanEventKind::BeltCluster(cluster) = self {
            return Some(cluster);
        }

        None
    }
}

impl<'de> Deserialize<'de> for ScanEventKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;

        let Value::Object(map) = &value else {
            return Err(serde::de::Error::custom("Failed to parse scan event kind"));
        };

        // If the 'StarType' key is present, then the whole object should be parsed as a star
        // variant or fail.
        if map.get("StarType").is_some() {
            return Ok(ScanEventKind::Star(
                serde_json::from_value(value)
                    .map_err(|e| serde::de::Error::custom(format!("{e}")))?,
            ));
        }

        // If the 'TidalLock' key is present, then the whole object should be parsed as a planet
        // variant or fail.
        if map.get("TidalLock").is_some() {
            return Ok(ScanEventKind::Planet(
                serde_json::from_value(value)
                    .map_err(|e| serde::de::Error::custom(format!("{e}")))?,
            ));
        }

        // It none of the above match only then should it be considered a belt cluster.
        Ok(ScanEventKind::BeltCluster(
            serde_json::from_value(value).map_err(|e| serde::de::Error::custom(format!("{e}")))?,
        ))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ScanEventStar {
    pub star_type: StarClass,

    #[serde(default)]
    pub subclass: u8,
    pub stellar_mass: f32,
    pub radius: f32,
    pub absolute_magnitude: f32,

    #[serde(rename = "Age_MY")]
    pub age_my: u32,
    pub surface_temperature: f32,
    pub luminosity: StarLuminosity,

    /// Missing if it's a single primary star instead of a binary or star system or more stars.
    pub orbit_info: Option<OrbitInfo>,
    pub rotation_period: f32,
    pub axial_tilt: f32,

    #[serde(default)]
    pub rings: Vec<ScanEventRing>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ScanEventPlanet {
    pub tidal_lock: bool,
    pub terraform_state: TerraformState,
    pub planet_class: PlanetClass,
    pub atmosphere: Atmosphere,
    pub atmosphere_type: Option<AtmosphereType>,

    #[serde(default)]
    pub atmosphere_composition: Vec<ScanEventPlanetAtmosphereComposition>,
    pub volcanism: Volcanism,

    #[serde(rename = "MassEM")]
    pub mass_em: f32,

    /// Radius of the planet in meters.
    pub radius: f32,
    pub surface_gravity: Gravity,
    pub surface_temperature: f32,
    pub surface_pressure: f32,
    pub landable: bool,

    #[serde(default)]
    pub materials: Vec<ScanEventPlanetMaterial>,
    pub composition: Option<PlanetComposition>,

    #[serde(flatten)]
    pub orbit_info: OrbitInfo,
    pub rotation_period: f32,
    pub axial_tilt: f32,

    #[serde(default)]
    pub rings: Vec<ScanEventRing>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ScanEventPlanetAtmosphereComposition {
    pub name: AtmosphereElement,
    pub percent: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ScanEventPlanetMaterial {
    pub name: Material,
    pub percent: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(rename_all = "PascalCase")]
pub enum ScanEventParent {
    /// The body orbits around a null-point with the given body id. This is used when multiple
    /// bodies orbit each other.
    Null(u8),

    /// The body orbits around a star with the given body id.
    Star(u8),

    /// The body orbits around a ring with the given body id.
    Ring(u8),

    /// The body orbits around a planet with the given body id.
    Planet(u8),
}

impl ScanEventParent {
    pub fn body_id(&self) -> u8 {
        match self {
            ScanEventParent::Null(id) => *id,
            ScanEventParent::Star(id) => *id,
            ScanEventParent::Ring(id) => *id,
            ScanEventParent::Planet(id) => *id,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ScanEventRing {
    pub name: String,
    pub ring_class: RingClass,

    #[serde(rename = "MassMT")]
    pub mass_mt: f32,
    pub inner_rad: f32,
    pub outer_rad: f32,
}

/// This struct is always empty and is just here to make sure [serde] recognizes the empty variant.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ScanEventBeltCluster {}

#[cfg(test)]
mod tests {
    use crate::galaxy::LocalDistance;
    use crate::logs::scan_event::ScanEventParent;
    use crate::modules::logs::content::log_event_content::scan_event::ScanEvent;
    use std::cmp::Ordering;

    #[test]
    fn scan_event_is_parsed_correctly() {
        let value = serde_json::from_str::<ScanEvent>(
            r#"
            {
                "timestamp": "2022-10-11T19:59:10Z",
                "event": "Scan",
                "ScanType": "AutoScan",
                "BodyName": "Etain A Belt Cluster 1",
                "BodyID": 2,
                "Parents": [
                    {
                        "Ring": 1
                    },
                    {
                        "Star": 0
                    }
                ],
                "StarSystem": "Etain",
                "SystemAddress": 2869977884057,
                "DistanceFromArrivalLS": 4.884683,
                "WasDiscovered": true,
                "WasMapped": false
            }
        "#,
        );

        assert!(value.is_ok());
    }

    #[test]
    fn distance_is_converted_correctly() {
        fn assert_roughly_eq(a: f32, b: f32) {
            assert!((a - b).abs() < 0.0001);
        }

        let distance = LocalDistance(1000.0);
        assert_roughly_eq(distance.as_au(), 2.0);

        let distance = LocalDistance::from_au(2.0);
        assert_roughly_eq(distance.as_ls(), 1000.0);
    }

    #[test]
    fn parent_vecs_are_ordered_correctly() {
        let test_cases = &[
            (
                vec![ScanEventParent::Null(1)],
                vec![ScanEventParent::Null(0)],
                Ordering::Greater,
            ),
            (
                vec![ScanEventParent::Planet(1), ScanEventParent::Null(0)],
                vec![ScanEventParent::Null(0)],
                Ordering::Greater,
            ),
        ];

        for (a, b, expected) in test_cases {
            let result = a.cmp(&b);
            assert_eq!(result, *expected);
        }
    }
}
