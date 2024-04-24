mod approach_body_event;
mod backpack_event;
mod buy_suit_event;
mod cargo_event;
mod carrier_jump_cancelled_event;
mod carrier_jump_request_event;
mod carrier_stats_event;
mod clear_saved_game_event;
mod codex_entry_event;
mod commander_event;
mod create_suit_loadout_event;
mod disembark_event;
mod dock_srv_event;
mod docked_event;
mod docking_cancelled_event;
mod docking_denied_event;
mod docking_granted_event;
mod docking_requested_event;
mod docking_timeout_event;
mod embark_event;
mod engineer_progress_event;
pub mod file_header_event;
mod friends_event;
mod fsd_jump_event;
mod fsd_target_event;
mod fss_all_bodies_found_event;
mod fss_body_signals_event;
mod fss_discovery_scan_event;
mod fss_signal_discovered_event;
mod fuel_scoop_event;
mod launch_srv_event;
mod leave_body_event;
mod liftoff_event;
mod load_game_event;
mod loadout_event;
mod location_event;
mod material_collected_event;
mod materials_event;
mod missions_event;
mod module_store_event;
mod music_event;
mod new_commander_event;
mod outfitting_event;
mod passengers_event;
mod powerplay_event;
mod progress_event;
mod rank_event;
mod receive_text_event;
mod reputation_event;
mod reservoir_replenished_event;
mod scan_bary_centre_event;
mod scan_event;
mod scan_organic_event;
mod shared;
mod ship_locker_event;
mod ship_targeted_event;
mod squadron_startup_event;
mod start_jump_event;
mod statistics_event;
mod stored_modules_event;
mod suit_loadout_event;
mod supercruise_destination_drop_event;
mod supercruise_entry_event;
mod supercruise_exit_event;
mod switch_suit_loadout_event;
mod touchdown_event;
mod undocked_event;
mod uss_drop_event;
mod npc_crew_wage_paid_event;
mod refuel_all_event;
mod set_user_ship_name_event;
mod saa_signals_found_event;
mod saa_scan_complete_event;
mod shipyard_event;
mod stored_ships_event;
mod shipyard_swap_event;
mod module_retrieve_event;
mod launch_drone_event;
mod send_text_event;
mod repair_all_event;
mod buy_ammo_event;
mod repair_event;
mod buy_drones_event;
mod under_attack_event;
mod shield_state_event;
mod scanned_event;
mod market_event;
mod market_buy_event;
mod material_trade_event;
mod module_swap_event;
mod carrier_jump_event;
mod sell_drones_event;
mod material_discovered_event;
mod collect_cargo_event;
mod eject_cargo_event;
mod approach_settlement_event;
mod promotion_event;
mod jet_cone_boost_event;
mod multi_sell_exploration_data_event;
mod datalink_scan_event;
mod interdicted_event;
mod repair_drone_event;
mod module_buy_event;
mod engineer_craft_event;

use crate::models::journal_event_kind::approach_body_event::ApproachBodyEvent;
use crate::models::journal_event_kind::backpack_event::BackpackEvent;
use crate::models::journal_event_kind::buy_suit_event::BuySuitEvent;
use crate::models::journal_event_kind::cargo_event::CargoEvent;
use crate::models::journal_event_kind::carrier_jump_cancelled_event::CarrierJumpCancelled;
use crate::models::journal_event_kind::carrier_jump_request_event::CarrierJumpRequestEvent;
use crate::models::journal_event_kind::carrier_stats_event::CarrierStatsEvent;
use crate::models::journal_event_kind::clear_saved_game_event::ClearSavedGameEvent;
use crate::models::journal_event_kind::codex_entry_event::CodexEntryEvent;
use crate::models::journal_event_kind::commander_event::CommanderEvent;
use crate::models::journal_event_kind::create_suit_loadout_event::CreateSuitLoadoutEvent;
use crate::models::journal_event_kind::disembark_event::DisembarkEvent;
use crate::models::journal_event_kind::dock_srv_event::DockSRVEvent;
use crate::models::journal_event_kind::docked_event::DockedEvent;
use crate::models::journal_event_kind::docking_cancelled_event::DockingCancelled;
use crate::models::journal_event_kind::docking_denied_event::DockingDeniedEvent;
use crate::models::journal_event_kind::docking_granted_event::DockingGrantedEvent;
use crate::models::journal_event_kind::docking_requested_event::DockingRequestedEvent;
use crate::models::journal_event_kind::docking_timeout_event::DockingTimeoutEvent;
use crate::models::journal_event_kind::embark_event::EmbarkEvent;
use crate::models::journal_event_kind::engineer_progress_event::EngineerProgressEvent;
use crate::models::journal_event_kind::file_header_event::FileHeaderEvent;
use crate::models::journal_event_kind::friends_event::FriendsEvent;
use crate::models::journal_event_kind::fsd_jump_event::FSDJumpEvent;
use crate::models::journal_event_kind::fsd_target_event::FSDTargetEvent;
use crate::models::journal_event_kind::fss_all_bodies_found_event::FSSAllBodiesFoundEvent;
use crate::models::journal_event_kind::fss_body_signals_event::FSSBodySignalsEvent;
use crate::models::journal_event_kind::fss_discovery_scan_event::FSSDiscoveryScan;
use crate::models::journal_event_kind::fss_signal_discovered_event::FSSSignalDiscoveredEvent;
use crate::models::journal_event_kind::fuel_scoop_event::FuelScoopEvent;
use crate::models::journal_event_kind::launch_srv_event::LaunchSRVEvent;
use crate::models::journal_event_kind::leave_body_event::LeaveBodyEvent;
use crate::models::journal_event_kind::liftoff_event::LiftoffEvent;
use crate::models::journal_event_kind::load_game_event::LoadGameEvent;
use crate::models::journal_event_kind::loadout_event::LoadoutEvent;
use crate::models::journal_event_kind::location_event::LocationEvent;
use crate::models::journal_event_kind::material_collected_event::MaterialCollectedEvent;
use crate::models::journal_event_kind::materials_event::MaterialsEvent;
use crate::models::journal_event_kind::missions_event::MissionsEvent;
use crate::models::journal_event_kind::module_store_event::ModuleStoreEvent;
use crate::models::journal_event_kind::music_event::MusicEvent;
use crate::models::journal_event_kind::new_commander_event::NewCommanderEvent;
use crate::models::journal_event_kind::outfitting_event::OutfittingEvent;
use crate::models::journal_event_kind::passengers_event::PassengersEvent;
use crate::models::journal_event_kind::powerplay_event::PowerplayEvent;
use crate::models::journal_event_kind::progress_event::ProgressEvent;
use crate::models::journal_event_kind::rank_event::RankEvent;
use crate::models::journal_event_kind::receive_text_event::ReceiveTextEvent;
use crate::models::journal_event_kind::reputation_event::ReputationEvent;
use crate::models::journal_event_kind::reservoir_replenished_event::ReservoirReplenishedEvent;
use crate::models::journal_event_kind::scan_bary_centre_event::ScanBaryCentreEvent;
use crate::models::journal_event_kind::scan_event::ScanEvent;
use crate::models::journal_event_kind::scan_organic_event::ScanOrganicEvent;
use crate::models::journal_event_kind::ship_locker_event::ShipLockerEvent;
use crate::models::journal_event_kind::ship_targeted_event::ShipTargetedEvent;
use crate::models::journal_event_kind::squadron_startup_event::SquadronStartupEvent;
use crate::models::journal_event_kind::start_jump_event::StartJumpEvent;
use crate::models::journal_event_kind::statistics_event::StatisticsEvent;
use crate::models::journal_event_kind::stored_modules_event::StoredModulesEvent;
use crate::models::journal_event_kind::suit_loadout_event::SuitLoadoutEvent;
use crate::models::journal_event_kind::supercruise_destination_drop_event::SupercruiseDestinationDropEvent;
use crate::models::journal_event_kind::supercruise_entry_event::SupercruiseEntryEvent;
use crate::models::journal_event_kind::supercruise_exit_event::SupercruiseExitEvent;
use crate::models::journal_event_kind::switch_suit_loadout_event::SwitchSuitLoadoutEvent;
use crate::models::journal_event_kind::touchdown_event::TouchdownEvent;
use crate::models::journal_event_kind::undocked_event::UndockedEvent;
use crate::models::journal_event_kind::uss_drop_event::USSDropEvent;
use serde::Deserialize;
use crate::models::journal_event_kind::approach_settlement_event::ApproachSettlementEvent;
use crate::models::journal_event_kind::module_retrieve_event::ModuleRetrieveEvent;
use crate::models::journal_event_kind::module_swap_event::ModuleSwapEvent;
use crate::models::journal_event_kind::buy_ammo_event::BuyAmmoEvent;
use crate::models::journal_event_kind::buy_drones_event::BuyDronesEvent;
use crate::models::journal_event_kind::carrier_jump_event::CarrierJumpEvent;
use crate::models::journal_event_kind::collect_cargo_event::CollectCargoEvent;
use crate::models::journal_event_kind::datalink_scan_event::DatalinkScanEvent;
use crate::models::journal_event_kind::eject_cargo_event::EjectCargoEvent;
use crate::models::journal_event_kind::engineer_craft_event::EngineerCraftEvent;
use crate::models::journal_event_kind::interdicted_event::InterdictedEvent;
use crate::models::journal_event_kind::jet_cone_boost_event::JetConeBoostEvent;
use crate::models::journal_event_kind::launch_drone_event::LaunchDroneEvent;
use crate::models::journal_event_kind::market_buy_event::MarketBuyEvent;
use crate::models::journal_event_kind::market_event::MarketEvent;
use crate::models::journal_event_kind::material_discovered_event::MaterialDiscoveredEvent;
use crate::models::journal_event_kind::material_trade_event::MaterialTradeEvent;
use crate::models::journal_event_kind::module_buy_event::ModuleBuyEvent;
use crate::models::journal_event_kind::multi_sell_exploration_data_event::MultiSellExplorationDataEvent;
use crate::models::journal_event_kind::npc_crew_wage_paid_event::NPCCrewWagePaidEvent;
use crate::models::journal_event_kind::promotion_event::PromotionEvent;
use crate::models::journal_event_kind::refuel_all_event::RefuelAllEvent;
use crate::models::journal_event_kind::repair_all_event::RepairAllEvent;
use crate::models::journal_event_kind::repair_drone_event::RepairDroneEvent;
use crate::models::journal_event_kind::repair_event::RepairEvent;
use crate::models::journal_event_kind::saa_scan_complete_event::SAAScanCompleteEvent;
use crate::models::journal_event_kind::saa_signals_found_event::SAASignalsFoundEvent;
use crate::models::journal_event_kind::scanned_event::ScannedEvent;
use crate::models::journal_event_kind::sell_drones_event::SellDronesEvent;
use crate::models::journal_event_kind::send_text_event::SendTextEvent;
use crate::models::journal_event_kind::set_user_ship_name_event::SetUserShipNameEvent;
use crate::models::journal_event_kind::shield_state_event::ShieldStateEvent;
use crate::models::journal_event_kind::shipyard_event::ShipyardEvent;
use crate::models::journal_event_kind::shipyard_swap_event::ShipyardSwapEvent;
use crate::models::journal_event_kind::stored_ships_event::StoredShipsEvent;
use crate::models::journal_event_kind::under_attack_event::UnderAttackEvent;

#[derive(Debug, Deserialize)]
#[serde(tag = "event")]
pub enum JournalEventKind {
    // Startup
    Cargo(CargoEvent),

    #[serde(rename = "Fileheader")]
    FileHeader(FileHeaderEvent),
    Commander(CommanderEvent),
    Materials(MaterialsEvent),
    Rank(RankEvent),
    Progress(ProgressEvent),
    Reputation(ReputationEvent),
    EngineerProgress(EngineerProgressEvent),
    LoadGame(LoadGameEvent),
    Powerplay(PowerplayEvent),
    Passengers(PassengersEvent),
    Statistics(StatisticsEvent),
    NewCommander(NewCommanderEvent),
    Loadout(LoadoutEvent),
    ClearSavedGame(ClearSavedGameEvent),
    Missions(MissionsEvent),

    // Travel
    ApproachBody(ApproachBodyEvent),
    Docked(DockedEvent),
    DockingCancelled(DockingCancelled),
    DockingDenied(DockingDeniedEvent),
    DockingGranted(DockingGrantedEvent),
    DockingRequested(DockingRequestedEvent),
    DockingTimeout(DockingTimeoutEvent),
    FSDJump(FSDJumpEvent),
    FSDTarget(FSDTargetEvent),
    LeaveBody(LeaveBodyEvent),
    Liftoff(LiftoffEvent),
    Location(LocationEvent),
    StartJump(StartJumpEvent),
    SupercruiseEntry(SupercruiseEntryEvent),
    SupercruiseExit(SupercruiseExitEvent),
    Touchdown(TouchdownEvent),
    Undocked(UndockedEvent),

    /// This event is fired when something changes in the `NavRoute.json` file and does not contain
    /// the route in the event.
    NavRoute,
    NavRouteClear,

    // Combat
    HeatWarning,
    Interdicted(InterdictedEvent),
    ShipTargeted(ShipTargetedEvent),
    ShieldState(ShieldStateEvent),
    UnderAttack(UnderAttackEvent),

    // Exploration
    CodexEntry(CodexEntryEvent),
    Scan(ScanEvent),
    FSSAllBodiesFound(FSSAllBodiesFoundEvent),
    FSSBodySignals(FSSBodySignalsEvent),
    FSSDiscoveryScan(FSSDiscoveryScan),
    FSSSignalDiscovered(FSSSignalDiscoveredEvent),
    MaterialCollected(MaterialCollectedEvent),
    MaterialDiscovered(MaterialDiscoveredEvent),
    MultiSellExplorationData(MultiSellExplorationDataEvent),
    SAAScanComplete(SAAScanCompleteEvent),
    SAASignalsFound(SAASignalsFoundEvent),
    ScanBaryCentre(ScanBaryCentreEvent),

    // Trade
    CollectCargo(CollectCargoEvent),
    EjectCargo(EjectCargoEvent),
    MarketBuy(MarketBuyEvent),

    // Station services
    BuyAmmo(BuyAmmoEvent),
    BuyDrones(BuyDronesEvent),
    Market(MarketEvent),
    MaterialTrade(MaterialTradeEvent),
    ModuleRetrieve(ModuleRetrieveEvent),
    ModuleSwap(ModuleSwapEvent),
    Outfitting(OutfittingEvent),
    ModuleBuy(ModuleBuyEvent),
    ModuleStore(ModuleStoreEvent),
    RefuelAll(RefuelAllEvent),
    Repair(RepairEvent),
    RepairAll(RepairAllEvent),
    SellDrones(SellDronesEvent),
    SetUserShipName(SetUserShipNameEvent),
    Shipyard(ShipyardEvent),
    ShipyardSwap(ShipyardSwapEvent),
    StoredModules(StoredModulesEvent),
    StoredShips(StoredShipsEvent),

    // Squadrons
    SquadronStartup(SquadronStartupEvent),

    // Fleet carriers
    CarrierJump(CarrierJumpEvent),
    CarrierStats(CarrierStatsEvent),
    CarrierJumpRequest(CarrierJumpRequestEvent),
    CarrierJumpCancelled(CarrierJumpCancelled),

    // Odyssey
    Backpack(BackpackEvent),
    BuySuit(BuySuitEvent),
    CreateSuitLoadout(CreateSuitLoadoutEvent),
    Disembark(DisembarkEvent),
    Embark(EmbarkEvent),
    ScanOrganic(ScanOrganicEvent),
    ShipLocker(ShipLockerEvent),
    SwitchSuitLoadout(SwitchSuitLoadoutEvent),
    SuitLoadout(SuitLoadoutEvent),

    // Other
    ApproachSettlement(ApproachSettlementEvent),
    DatalinkScan(DatalinkScanEvent),
    DockSRV(DockSRVEvent),
    EngineerCraft(EngineerCraftEvent),
    FuelScoop(FuelScoopEvent),
    Friends(FriendsEvent),
    JetConeBoost(JetConeBoostEvent),
    LaunchDrone(LaunchDroneEvent),
    LaunchSRV(LaunchSRVEvent),

    /// This event is fired when something changes in the `ModulesInfo.json` file and does not
    /// contain any data itself.
    ModuleInfo,
    Music(MusicEvent),

    #[serde(rename = "NpcCrewPaidWage")]
    NPCCrewPaidWage(NPCCrewWagePaidEvent),
    Promotion(PromotionEvent),
    ReceiveText(ReceiveTextEvent),
    RepairDrone(RepairDroneEvent),
    ReservoirReplenished(ReservoirReplenishedEvent),
    Scanned(ScannedEvent),
    SendText(SendTextEvent),
    Shutdown,
    USSDrop(USSDropEvent),
    SupercruiseDestinationDrop(SupercruiseDestinationDropEvent),
}
