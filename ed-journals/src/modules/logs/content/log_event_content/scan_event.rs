use serde::{Deserialize, Deserializer};
use serde_json::Value;

use crate::modules::shared::galaxy::atmosphere::Atmosphere;
use crate::modules::shared::galaxy::atmosphere_element::AtmosphereElement;
use crate::modules::shared::galaxy::atmosphere_type::AtmosphereType;
use crate::modules::shared::galaxy::orbit_info::OrbitInfo;
use crate::modules::shared::galaxy::planet_class::PlanetClass;
use crate::modules::shared::galaxy::ring_class::RingClass;
use crate::modules::shared::galaxy::star_class::StarClass;
use crate::modules::shared::galaxy::star_luminosity::StarLuminosity;
use crate::modules::shared::galaxy::terraform_state::TerraformState;
use crate::modules::shared::galaxy::volcanism::Volcanism;
use crate::modules::shared::materials::material::Material;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ScanEvent {
    pub scan_type: ScanEventScanType,
    pub body_name: String,

    #[serde(rename = "BodyID")]
    pub body_id: u8,

    #[serde(default)]
    pub parents: Vec<ScanEventParent>,
    pub star_system: String,
    pub system_address: u64,

    #[serde(rename = "DistanceFromArrivalLS")]
    pub distance_from_arrival_ls: f32,
    pub was_discovered: bool,
    pub was_mapped: bool,

    /// [None] value should be considered a belt cluster
    #[serde(flatten)]
    pub kind: ScanEventKind,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum ScanEventScanType {
    Basic,
    NavBeaconDetail,
    AutoScan,
    Detailed,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ScanEventKind {
    Star(ScanEventStar),
    Planet(ScanEventPlanet),
    BeltCluster(ScanEventBeltCluster),
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
                    .map_err(|e| serde::de::Error::custom(format!("{}", e)))?,
            ));
        }

        // If the 'TidalLock' key is present, then the whole object should be parsed as a planet
        // variant or fail.
        if map.get("TidalLock").is_some() {
            return Ok(ScanEventKind::Planet(
                serde_json::from_value(value)
                    .map_err(|e| serde::de::Error::custom(format!("{}", e)))?,
            ));
        }

        // It none of the above match only then should it be considered a belt cluster.
        Ok(ScanEventKind::BeltCluster(
            serde_json::from_value(value)
                .map_err(|e| serde::de::Error::custom(format!("{}", e)))?,
        ))
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ScanEventStar {
    pub star_type: StarClass,
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

#[derive(Debug, Deserialize, Clone, PartialEq)]
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
    pub radius: f32,
    pub surface_gravity: f32,
    pub surface_temperature: f32,
    pub surface_pressure: f32,
    pub landable: bool,

    #[serde(default)]
    pub materials: Vec<ScanEventPlanetMaterial>,
    pub composition: Option<ScanEventPlanetComposition>,

    #[serde(flatten)]
    pub orbit_info: OrbitInfo,
    pub rotation_period: f32,
    pub axial_tilt: f32,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ScanEventPlanetAtmosphereComposition {
    pub name: AtmosphereElement,
    pub percent: f32,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ScanEventPlanetMaterial {
    pub name: Material,
    pub percent: f32,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ScanEventPlanetComposition {
    pub ice: f32,
    pub rock: f32,
    pub metal: f32,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum ScanEventParent {
    Null(u8),
    Star(u8),
    Ring(u8),
    Planet(u8),
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
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
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ScanEventBeltCluster {}

#[cfg(test)]
mod tests {
    use crate::modules::logs::content::log_event_content::scan_event::ScanEvent;

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
}
