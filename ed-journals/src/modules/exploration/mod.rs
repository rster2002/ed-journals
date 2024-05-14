use crate::{
    logs::content::log_event_content::scan_event::{
        ScanEvent, ScanEventKind, ScanEventPlanet, ScanEventStar,
    },
    shared::galaxy::terraform_state::TerraformState,
};

use crate::shared::galaxy::planet_class::PlanetClass;
use crate::shared::galaxy::star_class::StarClass;

pub fn calculate_estimated_worth(scan: &ScanEvent) -> f32 {
    match &scan.kind {
        ScanEventKind::Star(star_scan) => {
            calculate_estimated_star_worth(&star_scan, !scan.was_discovered)
        }
        ScanEventKind::Planet(planet_scan) => {
            calculate_estimated_planet_worth(&planet_scan, !scan.was_discovered, !scan.was_mapped)
        }
        _ => 0.0,
    }
}

fn calculate_estimated_star_worth(scan: &ScanEventStar, is_first_discovery: bool) -> f32 {
    let k: f32 = match scan.star_type {
        StarClass::D => 14_057.0,
        StarClass::DA => 14_057.0,
        StarClass::DAB => 14_057.0,
        StarClass::DAO => 14_057.0,
        StarClass::DAZ => 14_057.0,
        StarClass::DAV => 14_057.0,
        StarClass::DB => 14_057.0,
        StarClass::DBZ => 14_057.0,
        StarClass::DBV => 14_057.0,
        StarClass::DO => 14_057.0,
        StarClass::DOV => 14_057.0,
        StarClass::DQ => 14_057.0,
        StarClass::DC => 14_057.0,
        StarClass::DCV => 14_057.0,
        StarClass::DX => 14_057.0,
        StarClass::N => 22_628.0,
        StarClass::H => 22_628.0,
        StarClass::SupermassiveBlackHole => 33.5678,
        _ => 1_200.0,
    };

    let m = f32::max(scan.stellar_mass, 1.0);

    let base_value = k + m * k / 66.25;

    if is_first_discovery {
        base_value * 2.6
    } else {
        base_value
    }
}

fn calculate_estimated_planet_worth(
    scan: &ScanEventPlanet,
    is_first_discovery: bool,
    is_first_map: bool,
) -> f32 {
    let t = scan.terraform_state == TerraformState::Terraformable;

    let k = match scan.planet_class {
        PlanetClass::MetalRichBody => 21_790.0 + (if t { 65_631.0 } else { 0.0 }),
        PlanetClass::AmmoniaWorld => 96_932.0,
        PlanetClass::SudarskyClassIGasGiant => 1_656.0,
        PlanetClass::SudarskyClassIiGasGiant => 9_654.0 + (if t { 100_677.0 } else { 0.0 }),
        PlanetClass::HighMetalContentBody => 9_654.0 + (if t { 100_677.0 } else { 0.0 }),
        PlanetClass::WaterWorld => 64_831.0 + (if t { 116_295.0 } else { 0.0 }),
        PlanetClass::EarthlikeBody => 64_831.0 + (if t { 116_295.0 } else { 0.0 }),
        _ => 300.0 + (if t { 93_328.0 } else { 0.0 }),
    };

    let m = f32::max(scan.mass_em, 1.0);

    let base_value = f32::max(k + k * f32::powf(m, 0.2) * 0.56591828, 500.0);

    match (is_first_discovery, is_first_map) {
        (true, true) => base_value * 9.6190186404,
        (false, true) => base_value * 8.0956,
        _ => base_value * 3.3333333333,
    }
}
