pub mod afmu_repairs_event;
pub mod applied_to_squadron_event;
pub mod approach_body_event;
pub mod approach_settlement_event;
pub mod asteroid_cracked_event;
pub mod backpack_change_event;
pub mod backpack_event;
pub mod bounty_event;
pub mod buy_ammo_event;
pub mod buy_drones_event;
pub mod buy_exploration_data_event;
pub mod buy_micro_resource_event;
pub mod buy_suit_event;
pub mod buy_weapon_event;
pub mod cargo_depot_event;
pub mod cargo_event;
pub mod cargo_transfer_event;
pub mod carrier_bank_transfer_event;
pub mod carrier_deposit_fuel_event;
pub mod carrier_jump_cancelled_event;
pub mod carrier_jump_event;
pub mod carrier_jump_request_event;
pub mod carrier_stats_event;
pub mod clear_saved_game_event;
pub mod codex_entry_event;
pub mod collect_cargo_event;
pub mod collect_items_event;
pub mod commander_event;
pub mod commit_crime_event;
pub mod create_suit_loadout_event;
pub mod crew_assign_event;
pub mod crime_victim_event;
pub mod data_scanned_event;
pub mod datalink_scan_event;
pub mod died_event;
pub mod discovery_scan_event;
pub mod disembark_event;
pub mod dock_fighter_event;
pub mod dock_srv_event;
pub mod docked_event;
pub mod docking_cancelled_event;
pub mod docking_denied_event;
pub mod docking_granted_event;
pub mod docking_requested_event;
pub mod docking_timeout_event;
pub mod drop_items_event;
pub mod eject_cargo_event;
pub mod embark_event;
pub mod engineer_craft_event;
pub mod engineer_progress_event;
pub mod escape_interdiction_event;
pub mod fetch_remote_module_event;
pub mod fighter_destroyed_event;
pub mod fighter_rebuilt_event;
pub mod file_header_event;
pub mod friends_event;
pub mod fsd_jump_event;
pub mod fsd_target_event;
pub mod fss_all_bodies_found_event;
pub mod fss_body_signals_event;
pub mod fss_discovery_scan_event;
pub mod fss_signal_discovered_event;
pub mod fuel_scoop_event;
pub mod hull_damage_event;
pub mod interdicted_event;
pub mod jet_cone_boost_event;
pub mod launch_drone_event;
pub mod launch_fighter_event;
pub mod launch_srv_event;
pub mod leave_body_event;
pub mod liftoff_event;
pub mod load_game_event;
pub mod loadout_equip_module_event;
pub mod loadout_event;
pub mod location_event;
pub mod market_buy_event;
pub mod market_event;
pub mod market_sell_event;
pub mod material_collected_event;
pub mod material_discovered_event;
pub mod material_trade_event;
pub mod materials_event;
pub mod mining_refined;
pub mod mission_abandoned_event;
pub mod mission_accepted_event;
pub mod mission_completed_event;
pub mod mission_failed_event;
pub mod mission_redirected_event;
pub mod missions_event;
pub mod module_buy_event;
pub mod module_retrieve_event;
pub mod module_sell_event;
pub mod module_store_event;
pub mod module_swap_event;
pub mod multi_sell_exploration_data_event;
pub mod music_event;
pub mod nav_beacon_scan_event;
pub mod new_commander_event;
pub mod npc_crew_rank_event;
pub mod npc_crew_wage_paid_event;
pub mod outfitting_event;
pub mod passengers_event;
pub mod pay_bounties_event;
pub mod pay_fines_event;
pub mod powerplay_event;
pub mod progress_event;
pub mod promotion_event;
pub mod prospected_asteroid_event;
pub mod rank_event;
pub mod receive_text_event;
pub mod redeem_voucher_event;
pub mod refuel_all_event;
pub mod repair_all_event;
pub mod repair_drone_event;
pub mod repair_event;
pub mod reputation_event;
pub mod reservoir_replenished_event;
pub mod restock_vehicle_event;
pub mod resurrect_event;
pub mod saa_scan_complete_event;
pub mod saa_signals_found_event;
pub mod scan_bary_centre_event;
pub mod scan_event;
pub mod scan_organic_event;
pub mod scanned_event;
pub mod sell_drones_event;
pub mod sell_organic_data;
pub mod send_text_event;
pub mod set_user_ship_name_event;
pub mod shared;
pub mod shield_state_event;
pub mod ship_locker_event;
pub mod ship_targeted_event;
pub mod shipyard_buy_event;
pub mod shipyard_event;
pub mod shipyard_new_event;
pub mod shipyard_swap_event;
pub mod shipyard_transfer_event;
pub mod squadron_startup_event;
pub mod start_jump_event;
pub mod statistics_event;
pub mod stored_modules_event;
pub mod stored_ships_event;
pub mod suit_loadout_event;
pub mod supercruise_destination_drop_event;
pub mod supercruise_entry_event;
pub mod supercruise_exit_event;
pub mod switch_suit_loadout_event;
pub mod synthasis_event;
pub mod technology_broker_event;
pub mod touchdown_event;
pub mod under_attack_event;
pub mod undocked_event;
pub mod use_consumable_event;
pub mod uss_drop_event;
pub mod vehicle_switch_event;

use crate::models::journal_event_kind::afmu_repairs_event::AFMURepairsEvent;
use crate::models::journal_event_kind::applied_to_squadron_event::AppliedToSquadronEvent;
use crate::models::journal_event_kind::approach_body_event::ApproachBodyEvent;
use crate::models::journal_event_kind::approach_settlement_event::ApproachSettlementEvent;
use crate::models::journal_event_kind::asteroid_cracked_event::AsteroidCrackedEvent;
use crate::models::journal_event_kind::backpack_change_event::BackpackChangeEvent;
use crate::models::journal_event_kind::backpack_event::BackpackEvent;
use crate::models::journal_event_kind::bounty_event::BountyEvent;
use crate::models::journal_event_kind::buy_ammo_event::BuyAmmoEvent;
use crate::models::journal_event_kind::buy_drones_event::BuyDronesEvent;
use crate::models::journal_event_kind::buy_exploration_data_event::BuyExplorationDataEvent;
use crate::models::journal_event_kind::buy_micro_resource_event::BuyMicroResourceEvent;
use crate::models::journal_event_kind::buy_suit_event::BuySuitEvent;
use crate::models::journal_event_kind::buy_weapon_event::BuyWeaponEvent;
use crate::models::journal_event_kind::cargo_depot_event::CargoDepotEvent;
use crate::models::journal_event_kind::cargo_event::CargoEvent;
use crate::models::journal_event_kind::cargo_transfer_event::CargoTransferEvent;
use crate::models::journal_event_kind::carrier_bank_transfer_event::CarrierBankTransferEvent;
use crate::models::journal_event_kind::carrier_deposit_fuel_event::CarrierDepositFuelEvent;
use crate::models::journal_event_kind::carrier_jump_cancelled_event::CarrierJumpCancelled;
use crate::models::journal_event_kind::carrier_jump_event::CarrierJumpEvent;
use crate::models::journal_event_kind::carrier_jump_request_event::CarrierJumpRequestEvent;
use crate::models::journal_event_kind::carrier_stats_event::CarrierStatsEvent;
use crate::models::journal_event_kind::clear_saved_game_event::ClearSavedGameEvent;
use crate::models::journal_event_kind::codex_entry_event::CodexEntryEvent;
use crate::models::journal_event_kind::collect_cargo_event::CollectCargoEvent;
use crate::models::journal_event_kind::collect_items_event::CollectItemsEvent;
use crate::models::journal_event_kind::commander_event::CommanderEvent;
use crate::models::journal_event_kind::commit_crime_event::CommitCrimeEvent;
use crate::models::journal_event_kind::create_suit_loadout_event::CreateSuitLoadoutEvent;
use crate::models::journal_event_kind::crew_assign_event::CrewAssignEvent;
use crate::models::journal_event_kind::crime_victim_event::CrimeVictimEvent;
use crate::models::journal_event_kind::data_scanned_event::DataScannedEvent;
use crate::models::journal_event_kind::datalink_scan_event::DatalinkScanEvent;
use crate::models::journal_event_kind::died_event::DiedEvent;
use crate::models::journal_event_kind::discovery_scan_event::DiscoveryScanEvent;
use crate::models::journal_event_kind::disembark_event::DisembarkEvent;
use crate::models::journal_event_kind::dock_fighter_event::DockFighterEvent;
use crate::models::journal_event_kind::dock_srv_event::DockSRVEvent;
use crate::models::journal_event_kind::docked_event::DockedEvent;
use crate::models::journal_event_kind::docking_cancelled_event::DockingCancelled;
use crate::models::journal_event_kind::docking_denied_event::DockingDeniedEvent;
use crate::models::journal_event_kind::docking_granted_event::DockingGrantedEvent;
use crate::models::journal_event_kind::docking_requested_event::DockingRequestedEvent;
use crate::models::journal_event_kind::docking_timeout_event::DockingTimeoutEvent;
use crate::models::journal_event_kind::drop_items_event::DropItemsEvent;
use crate::models::journal_event_kind::eject_cargo_event::EjectCargoEvent;
use crate::models::journal_event_kind::embark_event::EmbarkEvent;
use crate::models::journal_event_kind::engineer_craft_event::EngineerCraftEvent;
use crate::models::journal_event_kind::engineer_progress_event::EngineerProgressEvent;
use crate::models::journal_event_kind::escape_interdiction_event::EscapeInterdictionEvent;
use crate::models::journal_event_kind::fetch_remote_module_event::FetchRemoteModuleEvent;
use crate::models::journal_event_kind::fighter_destroyed_event::FighterDestroyedEvent;
use crate::models::journal_event_kind::fighter_rebuilt_event::FighterRebuiltEvent;
use crate::models::journal_event_kind::file_header_event::FileHeaderEvent;
use crate::models::journal_event_kind::friends_event::FriendsEvent;
use crate::models::journal_event_kind::fsd_jump_event::FSDJumpEvent;
use crate::models::journal_event_kind::fsd_target_event::FSDTargetEvent;
use crate::models::journal_event_kind::fss_all_bodies_found_event::FSSAllBodiesFoundEvent;
use crate::models::journal_event_kind::fss_body_signals_event::FSSBodySignalsEvent;
use crate::models::journal_event_kind::fss_discovery_scan_event::FSSDiscoveryScan;
use crate::models::journal_event_kind::fss_signal_discovered_event::FSSSignalDiscoveredEvent;
use crate::models::journal_event_kind::fuel_scoop_event::FuelScoopEvent;
use crate::models::journal_event_kind::hull_damage_event::HullDamageEvent;
use crate::models::journal_event_kind::interdicted_event::InterdictedEvent;
use crate::models::journal_event_kind::jet_cone_boost_event::JetConeBoostEvent;
use crate::models::journal_event_kind::launch_drone_event::LaunchDroneEvent;
use crate::models::journal_event_kind::launch_fighter_event::LaunchFighterEvent;
use crate::models::journal_event_kind::launch_srv_event::LaunchSRVEvent;
use crate::models::journal_event_kind::leave_body_event::LeaveBodyEvent;
use crate::models::journal_event_kind::liftoff_event::LiftoffEvent;
use crate::models::journal_event_kind::load_game_event::LoadGameEvent;
use crate::models::journal_event_kind::loadout_equip_module_event::LoadoutEquipModuleEvent;
use crate::models::journal_event_kind::loadout_event::LoadoutEvent;
use crate::models::journal_event_kind::location_event::LocationEvent;
use crate::models::journal_event_kind::market_buy_event::MarketBuyEvent;
use crate::models::journal_event_kind::market_event::MarketEvent;
use crate::models::journal_event_kind::market_sell_event::MarketSellEvent;
use crate::models::journal_event_kind::material_collected_event::MaterialCollectedEvent;
use crate::models::journal_event_kind::material_discovered_event::MaterialDiscoveredEvent;
use crate::models::journal_event_kind::material_trade_event::MaterialTradeEvent;
use crate::models::journal_event_kind::materials_event::MaterialsEvent;
use crate::models::journal_event_kind::mining_refined::MiningRefinedEvent;
use crate::models::journal_event_kind::mission_abandoned_event::MissionAbandonedEvent;
use crate::models::journal_event_kind::mission_accepted_event::MissionAcceptedEvent;
use crate::models::journal_event_kind::mission_completed_event::MissionCompletedEvent;
use crate::models::journal_event_kind::mission_failed_event::MissionFailedEvent;
use crate::models::journal_event_kind::mission_redirected_event::MissionRedirectedEvent;
use crate::models::journal_event_kind::missions_event::MissionsEvent;
use crate::models::journal_event_kind::module_buy_event::ModuleBuyEvent;
use crate::models::journal_event_kind::module_retrieve_event::ModuleRetrieveEvent;
use crate::models::journal_event_kind::module_sell_event::ModuleSellEvent;
use crate::models::journal_event_kind::module_store_event::ModuleStoreEvent;
use crate::models::journal_event_kind::module_swap_event::ModuleSwapEvent;
use crate::models::journal_event_kind::multi_sell_exploration_data_event::MultiSellExplorationDataEvent;
use crate::models::journal_event_kind::music_event::MusicEvent;
use crate::models::journal_event_kind::nav_beacon_scan_event::NavBeaconScanEvent;
use crate::models::journal_event_kind::new_commander_event::NewCommanderEvent;
use crate::models::journal_event_kind::npc_crew_rank_event::NPCCrewRankEvent;
use crate::models::journal_event_kind::npc_crew_wage_paid_event::NPCCrewWagePaidEvent;
use crate::models::journal_event_kind::outfitting_event::OutfittingEvent;
use crate::models::journal_event_kind::passengers_event::PassengersEvent;
use crate::models::journal_event_kind::pay_bounties_event::PayBountiesEvent;
use crate::models::journal_event_kind::pay_fines_event::PayFinesEvent;
use crate::models::journal_event_kind::powerplay_event::PowerplayEvent;
use crate::models::journal_event_kind::progress_event::ProgressEvent;
use crate::models::journal_event_kind::promotion_event::PromotionEvent;
use crate::models::journal_event_kind::prospected_asteroid_event::ProspectedAsteroidEvent;
use crate::models::journal_event_kind::rank_event::RankEvent;
use crate::models::journal_event_kind::receive_text_event::ReceiveTextEvent;
use crate::models::journal_event_kind::redeem_voucher_event::RedeemVoucherEvent;
use crate::models::journal_event_kind::refuel_all_event::RefuelAllEvent;
use crate::models::journal_event_kind::repair_all_event::RepairAllEvent;
use crate::models::journal_event_kind::repair_drone_event::RepairDroneEvent;
use crate::models::journal_event_kind::repair_event::RepairEvent;
use crate::models::journal_event_kind::reputation_event::ReputationEvent;
use crate::models::journal_event_kind::reservoir_replenished_event::ReservoirReplenishedEvent;
use crate::models::journal_event_kind::restock_vehicle_event::RestockVehicleEvent;
use crate::models::journal_event_kind::resurrect_event::ResurrectEvent;
use crate::models::journal_event_kind::saa_scan_complete_event::SAAScanCompleteEvent;
use crate::models::journal_event_kind::saa_signals_found_event::SAASignalsFoundEvent;
use crate::models::journal_event_kind::scan_bary_centre_event::ScanBaryCentreEvent;
use crate::models::journal_event_kind::scan_event::ScanEvent;
use crate::models::journal_event_kind::scan_organic_event::ScanOrganicEvent;
use crate::models::journal_event_kind::scanned_event::ScannedEvent;
use crate::models::journal_event_kind::sell_drones_event::SellDronesEvent;
use crate::models::journal_event_kind::sell_organic_data::SellOrganicDataEvent;
use crate::models::journal_event_kind::send_text_event::SendTextEvent;
use crate::models::journal_event_kind::set_user_ship_name_event::SetUserShipNameEvent;
use crate::models::journal_event_kind::shield_state_event::ShieldStateEvent;
use crate::models::journal_event_kind::ship_locker_event::ShipLockerEvent;
use crate::models::journal_event_kind::ship_targeted_event::ShipTargetedEvent;
use crate::models::journal_event_kind::shipyard_buy_event::ShipyardBuyEvent;
use crate::models::journal_event_kind::shipyard_event::ShipyardEvent;
use crate::models::journal_event_kind::shipyard_new_event::ShipyardNewEvent;
use crate::models::journal_event_kind::shipyard_swap_event::ShipyardSwapEvent;
use crate::models::journal_event_kind::shipyard_transfer_event::ShipyardTransferEvent;
use crate::models::journal_event_kind::squadron_startup_event::SquadronStartupEvent;
use crate::models::journal_event_kind::start_jump_event::StartJumpEvent;
use crate::models::journal_event_kind::statistics_event::StatisticsEvent;
use crate::models::journal_event_kind::stored_modules_event::StoredModulesEvent;
use crate::models::journal_event_kind::stored_ships_event::StoredShipsEvent;
use crate::models::journal_event_kind::suit_loadout_event::SuitLoadoutEvent;
use crate::models::journal_event_kind::supercruise_destination_drop_event::SupercruiseDestinationDropEvent;
use crate::models::journal_event_kind::supercruise_entry_event::SupercruiseEntryEvent;
use crate::models::journal_event_kind::supercruise_exit_event::SupercruiseExitEvent;
use crate::models::journal_event_kind::switch_suit_loadout_event::SwitchSuitLoadoutEvent;
use crate::models::journal_event_kind::synthasis_event::SynthesisEvent;
use crate::models::journal_event_kind::technology_broker_event::TechnologyBrokerEvent;
use crate::models::journal_event_kind::touchdown_event::TouchdownEvent;
use crate::models::journal_event_kind::under_attack_event::UnderAttackEvent;
use crate::models::journal_event_kind::undocked_event::UndockedEvent;
use crate::models::journal_event_kind::use_consumable_event::UseConsumableEvent;
use crate::models::journal_event_kind::uss_drop_event::USSDropEvent;
use crate::models::journal_event_kind::vehicle_switch_event::VehicleSwitchEvent;
use serde::Deserialize;
use serde_json::Value;

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
    Bounty(BountyEvent),
    Died(DiedEvent),
    EscapeInterdiction(EscapeInterdictionEvent),
    FighterDestroyed(FighterDestroyedEvent),
    HeatDamage,
    HeatWarning,
    HullDamage(HullDamageEvent),
    Interdicted(InterdictedEvent),
    ShipTargeted(ShipTargetedEvent),
    ShieldState(ShieldStateEvent),
    UnderAttack(UnderAttackEvent),

    // Exploration
    CodexEntry(CodexEntryEvent),
    DiscoveryScan(DiscoveryScanEvent),
    Scan(ScanEvent),
    FSSAllBodiesFound(FSSAllBodiesFoundEvent),
    FSSBodySignals(FSSBodySignalsEvent),
    FSSDiscoveryScan(FSSDiscoveryScan),
    FSSSignalDiscovered(FSSSignalDiscoveredEvent),
    MaterialCollected(MaterialCollectedEvent),
    MaterialDiscovered(MaterialDiscoveredEvent),
    MultiSellExplorationData(MultiSellExplorationDataEvent),
    NavBeaconScan(NavBeaconScanEvent),
    BuyExplorationData(BuyExplorationDataEvent),
    SAAScanComplete(SAAScanCompleteEvent),
    SAASignalsFound(SAASignalsFoundEvent),
    ScanBaryCentre(ScanBaryCentreEvent),

    // Trade
    AsteroidCracked(AsteroidCrackedEvent),
    CollectCargo(CollectCargoEvent),
    EjectCargo(EjectCargoEvent),
    MarketBuy(MarketBuyEvent),
    MarketSell(MarketSellEvent),
    MiningRefined(MiningRefinedEvent),

    // Station services
    BuyAmmo(BuyAmmoEvent),
    BuyDrones(BuyDronesEvent),
    CargoDepot(CargoDepotEvent),
    CrewAssign(CrewAssignEvent),
    FetchRemoteModule(FetchRemoteModuleEvent),
    Market(MarketEvent),
    MaterialTrade(MaterialTradeEvent),
    MissionAbandoned(MissionAbandonedEvent),
    MissionAccepted(MissionAcceptedEvent),
    MissionCompleted(MissionCompletedEvent),
    MissionFailed(MissionFailedEvent),
    MissionRedirected(MissionRedirectedEvent),
    ModuleRetrieve(ModuleRetrieveEvent),
    ModuleSell(ModuleSellEvent),
    ModuleSwap(ModuleSwapEvent),
    Outfitting(OutfittingEvent),
    PayBounties(PayBountiesEvent),
    PayFines(PayFinesEvent),
    ModuleBuy(ModuleBuyEvent),
    ModuleStore(ModuleStoreEvent),
    RedeemVoucher(RedeemVoucherEvent),
    RefuelAll(RefuelAllEvent),
    Repair(RepairEvent),
    RepairAll(RepairAllEvent),
    RestockVehicle(RestockVehicleEvent),
    SellDrones(SellDronesEvent),
    SetUserShipName(SetUserShipNameEvent),
    Shipyard(ShipyardEvent),
    ShipyardBuy(ShipyardBuyEvent),
    ShipyardNew(ShipyardNewEvent),
    ShipyardTransfer(ShipyardTransferEvent),
    ShipyardSwap(ShipyardSwapEvent),
    StoredModules(StoredModulesEvent),
    StoredShips(StoredShipsEvent),
    TechnologyBroker(TechnologyBrokerEvent),

    // Squadrons
    AppliedToSquadron(AppliedToSquadronEvent),
    SquadronStartup(SquadronStartupEvent),

    // Fleet carriers
    CarrierJump(CarrierJumpEvent),
    CarrierStats(CarrierStatsEvent),
    CarrierJumpRequest(CarrierJumpRequestEvent),
    CarrierBankTransfer(CarrierBankTransferEvent),
    CarrierDepositFuel(CarrierDepositFuelEvent),
    CarrierJumpCancelled(CarrierJumpCancelled),

    // Odyssey
    Backpack(BackpackEvent),
    BackpackChange(BackpackChangeEvent),
    BuyMicroResources(BuyMicroResourceEvent),
    BuySuit(BuySuitEvent),
    BuyWeapon(BuyWeaponEvent),
    CollectItems(CollectItemsEvent),
    CreateSuitLoadout(CreateSuitLoadoutEvent),
    Disembark(DisembarkEvent),
    DropItems(DropItemsEvent),
    Embark(EmbarkEvent),
    LoadoutEquipModule(LoadoutEquipModuleEvent),
    ScanOrganic(ScanOrganicEvent),
    SellOrganicData(SellOrganicDataEvent),
    ShipLocker(ShipLockerEvent),
    SwitchSuitLoadout(SwitchSuitLoadoutEvent),
    SuitLoadout(SuitLoadoutEvent),
    UseConsumable(UseConsumableEvent),

    // Other
    #[serde(rename = "AfmuRepairs")]
    AFMURepairs(AFMURepairsEvent),
    ApproachSettlement(ApproachSettlementEvent),
    CommitCrime(CommitCrimeEvent),
    CargoTransfer(CargoTransferEvent),
    CockpitBreached,
    CrimeVictim(CrimeVictimEvent),
    DatalinkScan(DatalinkScanEvent),
    DataScanned(DataScannedEvent),
    DockFighter(DockFighterEvent),
    DockSRV(DockSRVEvent),
    EngineerCraft(EngineerCraftEvent),
    FighterRebuilt(FighterRebuiltEvent),
    FuelScoop(FuelScoopEvent),
    Friends(FriendsEvent),
    JetConeBoost(JetConeBoostEvent),
    LaunchDrone(LaunchDroneEvent),
    LaunchFighter(LaunchFighterEvent),
    LaunchSRV(LaunchSRVEvent),

    /// This event is fired when something changes in the `ModulesInfo.json` file and does not
    /// contain any data itself.
    ModuleInfo,
    Music(MusicEvent),

    #[serde(rename = "NpcCrewPaidWage")]
    NPCCrewPaidWage(NPCCrewWagePaidEvent),

    #[serde(rename = "NpcCrewRank")]
    NPCCrewRank(NPCCrewRankEvent),
    Promotion(PromotionEvent),
    ProspectedAsteroid(ProspectedAsteroidEvent),
    ReceiveText(ReceiveTextEvent),
    RepairDrone(RepairDroneEvent),
    ReservoirReplenished(ReservoirReplenishedEvent),
    Resurrect(ResurrectEvent),
    Scanned(ScannedEvent),
    SelfDestruct,
    SendText(SendTextEvent),
    Shutdown,
    Synthesis(SynthesisEvent),
    SystemsShutdown,
    USSDrop(USSDropEvent),
    VehicleSwitch(VehicleSwitchEvent),
    SupercruiseDestinationDrop(SupercruiseDestinationDropEvent),

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(Value),
}
