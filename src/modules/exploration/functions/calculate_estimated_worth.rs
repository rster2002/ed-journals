use crate::logs::content::log_event_content::scan_event::{ScanEvent, ScanEventKind, ScanEventPlanet, ScanEventStar};
use crate::modules::galaxy::TerraformState;

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
    let base_value = scan.star_type.base_value();
    let mass_factor = f32::max(scan.stellar_mass, 1.0);
    let body_value = base_value + mass_factor * base_value / 66.25;

    if is_first_discovery {
        body_value * 2.6
    } else {
        body_value
    }
}

fn calculate_estimated_planet_worth(
    scan: &ScanEventPlanet,
    is_first_discovery: bool,
    is_first_map: bool,
) -> f32 {
    let base_value = scan.planet_class.base_value();
    let terraformable_bonus = if scan.terraform_state == TerraformState::Terraformable {
        scan.planet_class.terraformable_bonus()
    } else {
        0
    };

    let scan_value = (base_value + terraformable_bonus) as f32;
    let mass_factor = f32::max(scan.mass_em, 1.0);
    let body_value = f32::max(scan_value + scan_value * f32::powf(mass_factor, 0.2) * 0.56591828, 500.0);

    match (is_first_discovery, is_first_map) {
        (true, true) => body_value * 9.6190186404,
        (false, true) => body_value * 8.0956,
        _ => body_value * 3.3333333333,
    }
}
