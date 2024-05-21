// use std::collections::{HashMap, HashSet};
//
// use serde::Serialize;
// use strum::IntoEnumIterator;
//
// use crate::exobiology::{SpawnSource, SpawnSourceStar, TargetPlanet};
// use crate::exploration::PlanetarySignalType;
// use crate::galaxy::Nebula;
// use crate::logs::content::{LogEvent, LogEventContent};
// use crate::logs::content::log_event_content::fsd_jump_event::FSDJumpEvent;
// use crate::logs::content::log_event_content::fss_body_signals_event::FSSBodySignalsEvent;
// use crate::logs::content::log_event_content::location_event::LocationEvent;
// use crate::logs::content::log_event_content::scan_event::{
//     ScanEvent, ScanEventKind, ScanEventParent, ScanEventPlanet, ScanEventStar,
// };
//
// use super::feed_result::FeedResult;
//
// #[derive(Debug, Serialize)]
// pub struct ExobiologyState {
//     /// Map of body names to spawn sources.
//     pub spawn_sources: HashMap<String, SpawnSource>,
//     planet_scan_events: Vec<(ScanEvent, ScanEventPlanet)>,
//     star_scan_events: Vec<(ScanEvent, ScanEventStar)>,
//     fss_body_signals_events: Vec<FSSBodySignalsEvent>,
//     location_events: Vec<LocationEvent>,
//     fsd_jump_events: Vec<FSDJumpEvent>,
// }
//
// impl ExobiologyState {
//     pub fn new() -> Self {
//         ExobiologyState {
//             spawn_sources: HashMap::new(),
//             planet_scan_events: Vec::new(),
//             star_scan_events: Vec::new(),
//             fss_body_signals_events: Vec::new(),
//             location_events: Vec::new(),
//             fsd_jump_events: Vec::new(),
//         }
//     }
//
//     /// Feeds an event into the ExobiologyState's pool of information to construct a SpawnSource.
//     pub fn feed_event(&mut self, event: &LogEvent) -> FeedResult {
//         match &event.content {
//             LogEventContent::Scan(scan) => match &scan.kind {
//                 ScanEventKind::Planet(planet) => {
//                     self.planet_scan_events.push((scan.clone(), planet.clone()));
//                 }
//                 ScanEventKind::Star(star) => {
//                     self.star_scan_events.push((scan.clone(), star.clone()));
//                 }
//                 _ => return FeedResult::Skipped,
//             },
//             LogEventContent::FSSBodySignals(fss_body_signals) => {
//                 self.fss_body_signals_events.push(fss_body_signals.clone());
//             }
//             LogEventContent::Location(location) => {
//                 self.location_events.push(location.clone());
//             }
//             LogEventContent::FSDJump(fsd_jump) => {
//                 self.fsd_jump_events.push(fsd_jump.clone());
//             }
//             _ => return FeedResult::Skipped,
//         }
//
//         FeedResult::Accepted
//     }
//
//     /// Constructs a SpawnSource from the ExobiologyState's pool of information.
//     pub fn for_body(&self, body_name: impl Into<String>) -> SpawnSource {
//         let body_name = body_name.into();
//
//         let event_is_applicable = |star_system: &String| body_name.starts_with(star_system);
//
//         // Filter events to only include those that are applicable to the body.
//         let star_scan_events = self
//             .star_scan_events
//             .iter()
//             .filter(|(scan, _)| event_is_applicable(&scan.star_system))
//             .collect::<Vec<&(ScanEvent, ScanEventStar)>>();
//
//         let planet_scan_events = self
//             .planet_scan_events
//             .iter()
//             .filter(|(scan, _)| event_is_applicable(&scan.star_system))
//             .collect::<Vec<&(ScanEvent, ScanEventPlanet)>>();
//
//         let target_body = planet_scan_events
//             .iter()
//             .find(|(scan, _)| scan.body_name == body_name)
//             .map(|(scan, planet)| (scan, planet));
//
//         let fss_body_signal = self
//             .fss_body_signals_events
//             .iter()
//             .find(|fss_body_signals| fss_body_signals.body_name == body_name);
//
//         let star_pos_from_location = self
//             .location_events
//             .iter()
//             .filter(|location| event_is_applicable(&location.location_info.star_system))
//             .map(|event| event.location_info.star_pos)
//             .next();
//
//         let star_pos_from_jump = self
//             .fsd_jump_events
//             .iter()
//             .filter(|fsd_jump| event_is_applicable(&fsd_jump.system_info.star_system))
//             .map(|event| event.system_info.star_pos)
//             .next();
//
//         // Construct the SpawnSource
//         SpawnSource {
//             body_name: body_name.clone(),
//             star_system_position: star_pos_from_location.or(star_pos_from_jump),
//             parent_stars: target_body
//                 .map(|(scan, _)| {
//                     let parent_ids = scan
//                         .parents
//                         .iter()
//                         .filter_map(|parent| match parent {
//                             ScanEventParent::Star(star) => Some(*star),
//                             _ => None,
//                         })
//                         .collect::<Vec<u8>>();
//
//                     star_scan_events
//                         .iter()
//                         .filter(|(scan, _)| parent_ids.contains(&scan.body_id))
//                         .map(|(scan, star)| SpawnSourceStar {
//                             class: star.star_type.clone(),
//                             body_id: scan.body_id,
//                             luminosity: star.luminosity.clone(),
//                         })
//                         .collect::<Vec<SpawnSourceStar>>()
//                 })
//                 .unwrap_or_default(),
//             target_planet: target_body.map(|(_, planet)| TargetPlanet {
//                 atmosphere: planet.atmosphere.clone(),
//                 gravity: planet.surface_gravity.clone(),
//                 class: planet.planet_class.clone(),
//                 surface_temperature: planet.surface_temperature,
//                 volcanism: planet.volcanism.clone(),
//                 materials: HashSet::from_iter(planet.materials.iter().map(|m| m.name.clone())),
//                 composition: planet.composition.clone().unwrap_or_default(),
//             }),
//             geological_signals_present: fss_body_signal.map(|fss_body_signal| {
//                 fss_body_signal
//                     .signals
//                     .iter()
//                     .any(|signal| signal.kind == PlanetarySignalType::Geological)
//             }),
//             distance_from_star: target_body.map(|body| body.0.distance_from_arrival.clone()),
//             distance_from_nebula: star_pos_from_location
//                 .or(star_pos_from_jump)
//                 .map(|pos| Nebula::closest_to(pos).1),
//
//             planet_classes_in_system: HashSet::from_iter(
//                 planet_scan_events
//                     .iter()
//                     .map(|(_, planet)| planet.planet_class.clone()),
//             ),
//             stars_in_system: HashSet::from_iter(star_scan_events.iter().map(|(scan, star)| {
//                 SpawnSourceStar {
//                     class: star.star_type.clone(),
//                     body_id: scan.body_id,
//                     luminosity: star.luminosity.clone(),
//                 }
//             })),
//         }
//     }
// }
//
// impl Default for ExobiologyState {
//     fn default() -> Self {
//         ExobiologyState::new()
//     }
// }
//
