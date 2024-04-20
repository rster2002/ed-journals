use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct ScanEvent {
    // TODO turn this into an enum
    pub scan_type: String,
    pub body_name: String,

    #[serde(rename = "BodyID")]
    pub body_id: u8,
    pub parents: Vec<ScanEventParent>,
    pub star_system: String,
    pub system_address: u64,

    #[serde(rename = "DistanceFromArrivalLS")]
    pub distance_from_arrival_ls: f32,
    pub was_discovered: bool,
    pub was_mapped: bool,

    #[serde(flatten)]
    pub kind: ScanEventKind,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase", untagged)]
pub enum ScanEventKind {
    BeltCluster,
    Star(ScanEventStar),
    Planet(ScanEventPlanet),
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct ScanEventStar {
    // TODO turn this into an enum
    pub star_type: String,

    // TODO maybe include this in the star type
    pub subclass: u8,
    pub stellar_mass: f32,
    pub radius: f32,
    pub absolute_magnitude: f32,

    #[serde(rename = "Age_MY")]
    pub age_my: u32,
    pub surface_temperature: f32,

    // TODO turn this into an enum
    pub luminosity: String,
    pub semi_major_axis: f32,
    pub eccentricity: f32,
    pub orbital_inclination: f32,
    pub periapsis: f32,
    pub orbital_period: f32,
    pub ascending_node: f32,
    pub mean_anomaly: f32,
    pub rotation_period: f32,
    pub axial_tilt: f32,

    #[serde(default)]
    pub rings: Vec<ScanEventRing>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct ScanEventPlanet {
    pub tidal_lock: bool,

    // TODO turn this into an enum
    pub terraform_state: String,

    // TODO turn this into an enum
    pub planet_class: String,

    // TODO turn this into an enum
    pub atmosphere: String,

    // TODO turn this into an enum
    pub atmosphere_type: String,

    // TODO turn this into an enum
    pub volcanism: String,

    #[serde(rename = "MassEM")]
    pub mass_em: f32,

    pub radius: f32,
    pub surface_gravity: f32,
    pub surface_temperature: f32,
    pub surface_pressure: f32,
    pub landable: bool,
    pub materials: Vec<ScanEventPlanetMaterial>,
    pub composition: ScanEventPlanetComposition,

    pub semi_major_axis: f32,
    pub eccentricity: f32,
    pub orbital_inclination: f32,
    pub periapsis: f32,
    pub orbital_period: f32,
    pub ascending_node: f32,
    pub mean_anomaly: f32,
    pub rotation_period: f32,
    pub axial_tilt: f32,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct ScanEventPlanetMaterial {
    // TODO turn this into an enum
    pub name: String,
    pub percent: f32,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct ScanEventPlanetComposition {
    pub ice: f32,
    pub rock: f32,
    pub metal: f32,
}


// #[derive(Debug, Deserialize)]
// #[cfg_attr(test, derive(PartialEq))]
// #[serde(rename_all = "PascalCase")]
// pub struct ScanEvent {
//     // TODO turn this into an enum
//     pub scan_type: String,
//     pub body_name: String,
//
//     #[serde(rename = "BodyID")]
//     pub parents: Vec<ScanEventParent>,

//
//     #[serde(rename = "DistanceFromArrivalLS")]
//     pub distance_from_arrival_ls: f32,
//

//
//     pub was_discovered: bool,
//     pub was_mapped: bool,
// }

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub enum ScanEventParent {
    Null(u8),
    Star(u8),
    Ring(u8),
    Planet(u8),
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct ScanEventRing {
    pub name: String,

    // TODO turn this into an enum
    pub ring_class: String,

    #[serde(rename = "MassMT")]
    pub mass_mt: f32,
    pub inner_rad: f32,
    pub outer_rad: f32,
}
