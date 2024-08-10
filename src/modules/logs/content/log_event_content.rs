use kinded::Kinded;
use serde::{Deserialize, Serialize};
#[cfg(not(feature = "strict"))]
use serde_json::Value;

use afmu_repairs_event::AFMURepairsEvent;
use applied_to_squadron_event::AppliedToSquadronEvent;
use approach_body_event::ApproachBodyEvent;
use approach_settlement_event::ApproachSettlementEvent;
use asteroid_cracked_event::AsteroidCrackedEvent;
use backpack_change_event::BackpackChangeEvent;
use backpack_event::BackpackEvent;
use book_dropship_event::BookDropshipEvent;
use book_taxi_event::BookTaxiEvent;
use bounty_event::BountyEvent;
use buy_ammo_event::BuyAmmoEvent;
use buy_drones_event::BuyDronesEvent;
use buy_exploration_data_event::BuyExplorationDataEvent;
use buy_micro_resource_event::BuyMicroResourceEvent;
use buy_suit_event::BuySuitEvent;
use buy_trade_data_event::BuyTradeDateEvent;
use buy_weapon_event::BuyWeaponEvent;
use cancel_dropship_event::CancelDropshipEvent;
use cancel_taxi_event::CancelTaxiEvent;
use cap_ship_bond_event::CapShipBondEvent;
use cargo_depot_event::CargoDepotEvent;
use cargo_event::CargoEvent;
use cargo_transfer_event::CargoTransferEvent;
use carrier_bank_transfer_event::CarrierBankTransferEvent;
use carrier_buy_event::CarrierBuyEvent;
use carrier_cancel_decommission_event::CarrierCancelDecommissionEvent;
use carrier_crew_services_event::CarrierCrewServicesEvent;
use carrier_decommision_event::CarrierDecommissionEvent;
use carrier_deposit_fuel_event::CarrierDepositFuelEvent;
use carrier_docking_permission_event::CarrierDockingPermissionEvent;
use carrier_finance_event::CarrierFinanceEvent;
use carrier_jump_cancelled_event::CarrierJumpCancelled;
use carrier_jump_event::CarrierJumpEvent;
use carrier_jump_request_event::CarrierJumpRequestEvent;
use carrier_module_pack_event::CarrierModulePackEvent;
use carrier_name_changed_event::CarrierNameChangeEvent;
use carrier_ship_pack_event::CarrierShipPackEvent;
use carrier_stats_event::CarrierStatsEvent;
use carrier_trade_order_event::CarrierTradeOrderEvent;
use change_crew_role_event::ChangeCrewRoleEvent;
use clear_impound_event::ClearImpoundEvent;
use clear_saved_game_event::ClearSavedGameEvent;
use codex_entry_event::CodexEntryEvent;
use collect_cargo_event::CollectCargoEvent;
use collect_items_event::CollectItemsEvent;
use commander_event::CommanderEvent;
use commit_crime_event::CommitCrimeEvent;
use community_goal_discard_event::CommunityGoalDiscardEvent;
use community_goal_join_event::CommunityGoalJoinEvent;
use community_goal_reward_event::CommunityGoalRewardEvent;
use continued_event::ContinuedEvent;
use create_suit_loadout_event::CreateSuitLoadoutEvent;
use crew_assign_event::CrewAssignEvent;
use crew_fire_event::CrewFireEvent;
use crew_hire_event::CrewHireEvent;
use crew_launch_fighter_event::CrewLaunchFighterEvent;
use crew_member_joins_event::CrewMemberJoinsEvent;
use crew_member_quits_event::CrewMemberQuitsEvent;
use crew_member_role_change_event::CrewMemberRoleChangeEvent;
use crime_victim_event::CrimeVictimEvent;
use data_scanned_event::DataScannedEvent;
use datalink_scan_event::DatalinkScanEvent;
use datalink_voucher_event::DatalinkVoucherEvent;
use delete_suit_loadout_event::DeleteSuitLoadoutEvent;
use died_event::DiedEvent;
use disbanded_squadron_event::DisbandedSquadronEvent;
use discovery_scan_event::DiscoveryScanEvent;
use disembark_event::DisembarkEvent;
use dock_fighter_event::DockFighterEvent;
use dock_srv_event::DockSRVEvent;
use docked_event::DockedEvent;
use docking_cancelled_event::DockingCancelled;
use docking_denied_event::DockingDeniedEvent;
use docking_granted_event::DockingGrantedEvent;
use docking_requested_event::DockingRequestedEvent;
use docking_timeout_event::DockingTimeoutEvent;
use drop_items_event::DropItemsEvent;
use drop_ship_deploy_event::DropshipDeployEvent;
use eject_cargo_event::EjectCargoEvent;
use embark_event::EmbarkEvent;
use end_crew_session_event::EndCrewSessionEvent;
use engineer_contribution_event::EngineerContributionEvent;
use engineer_craft_event::EngineerCraftEvent;
use engineer_progress_event::EngineerProgressEvent;
use escape_interdiction_event::EscapeInterdictionEvent;
use faction_kill_bond_event::FactionKillBondEvent;
use fetch_remote_module_event::FetchRemoteModuleEvent;
use fighter_destroyed_event::FighterDestroyedEvent;
use fighter_rebuilt_event::FighterRebuiltEvent;
use file_header_event::FileHeaderEvent;
use friends_event::FriendsEvent;
use fs_embark_event::FCMaterialsEvent;
use fsd_jump_event::FSDJumpEvent;
use fsd_target_event::FSDTargetEvent;
use fss_all_bodies_found_event::FSSAllBodiesFoundEvent;
use fss_body_signals_event::FSSBodySignalsEvent;
use fss_discovery_scan_event::FSSDiscoveryScan;
use fss_signal_discovered_event::FSSSignalDiscoveredEvent;
use fuel_scoop_event::FuelScoopEvent;
use hull_damage_event::HullDamageEvent;
use interdicted_event::InterdictedEvent;
use interdiction_event::InterdictionEvent;
use invited_to_squadron_event::InvitedToSquadronEvent;
use jet_cone_boost_event::JetConeBoostEvent;
use jet_cone_damage_event::JetConeDamageEvent;
use join_a_crew_event::JoinACrewEvent;
use joined_squadron_event::JoinedSquadronEvent;
use kick_crew_member_event::KickCrewMemberEvent;
use kicked_from_squadron_event::KickedFromSquadronEvent;
use launch_drone_event::LaunchDroneEvent;
use launch_fighter_event::LaunchFighterEvent;
use launch_srv_event::LaunchSRVEvent;
use leave_body_event::LeaveBodyEvent;
use left_squadron_event::LeftSquadronEvent;
use liftoff_event::LiftoffEvent;
use load_game_event::LoadGameEvent;
use loadout_equip_module_event::LoadoutEquipModuleEvent;
use loadout_event::LoadoutEvent;
use loadout_remove_module_event::LoadoutRemoveModuleEvent;
use location_event::LocationEvent;
use market_buy_event::MarketBuyEvent;
use market_event::MarketEvent;
use market_sell_event::MarketSellEvent;
use mass_module_store_event::MassModuleStoreEvent;
use material_collected_event::MaterialCollectedEvent;
use material_discarded_event::MaterialDiscarded;
use material_discovered_event::MaterialDiscoveredEvent;
use material_trade_event::MaterialTradeEvent;
use materials_event::MaterialsEvent;
use mining_refined::MiningRefinedEvent;
use mission_abandoned_event::MissionAbandonedEvent;
use mission_accepted_event::MissionAcceptedEvent;
use mission_completed_event::MissionCompletedEvent;
use mission_failed_event::MissionFailedEvent;
use mission_redirected_event::MissionRedirectedEvent;
use missions_event::MissionsEvent;
use module_buy_and_store_event::ModuleBuyAndStoreEvent;
use module_buy_event::ModuleBuyEvent;
use module_retrieve_event::ModuleRetrieveEvent;
use module_sell_event::ModuleSellEvent;
use module_sell_remote_event::ModuleSellRemoteEvent;
use module_store_event::ModuleStoreEvent;
use module_swap_event::ModuleSwapEvent;
use multi_sell_exploration_data_event::MultiSellExplorationDataEvent;
use music_event::MusicEvent;
use nav_beacon_scan_event::NavBeaconScanEvent;
use new_commander_event::NewCommanderEvent;
use npc_crew_rank_event::NPCCrewRankEvent;
use npc_crew_wage_paid_event::NPCCrewWagePaidEvent;
use outfitting_event::OutfittingEvent;
use passengers_event::PassengersEvent;
use pay_bounties_event::PayBountiesEvent;
use pay_fines_event::PayFinesEvent;
use powerplay_collect_event::PowerplayCollectEvent;
use powerplay_defect_event::PowerplayDefectEvent;
use powerplay_deliver_event::PowerplayDeliverEvent;
use powerplay_event::PowerplayEvent;
use powerplay_fast_track_event::PowerplayFastTrackEvent;
use powerplay_join_event::PowerplayJoinEvent;
use powerplay_leave_event::PowerplayLeaveEvent;
use powerplay_salary_event::PowerplaySalaryEvent;
use powerplay_vote_event::PowerplayVoteEvent;
use progress_event::ProgressEvent;
use promotion_event::PromotionEvent;
use prospected_asteroid_event::ProspectedAsteroidEvent;
use pvp_kill_event::PVPKillEvent;
use quit_a_crew_event::QuitACrewEvent;
use rank_event::RankEvent;
use reboot_repair_event::RebootRepairEvent;
use receive_text_event::ReceiveTextEvent;
use redeem_voucher_event::RedeemVoucherEvent;
use refuel_all_event::RefuelAllEvent;
use refuel_partial_event::RefuelPartialEvent;
use rename_suit_loadout_event::RenameSuitLoadoutEvent;
use repair_all_event::RepairAllEvent;
use repair_drone_event::RepairDroneEvent;
use repair_event::RepairEvent;
use reputation_event::ReputationEvent;
use reservoir_replenished_event::ReservoirReplenishedEvent;
use restock_vehicle_event::RestockVehicleEvent;
use resurrect_event::ResurrectEvent;
use saa_scan_complete_event::SAAScanCompleteEvent;
use saa_signals_found_event::SAASignalsFoundEvent;
use scan_bary_centre_event::ScanBaryCentreEvent;
use scan_event::ScanEvent;
use scan_organic_event::ScanOrganicEvent;
use scanned_event::ScannedEvent;
use scientific_research_event::ScientificResearchEvent;
use screenshot_event::ScreenshotEvent;
use search_and_rescue_event::SearchAndRescueEvent;
use sell_drones_event::SellDronesEvent;
use sell_exploration_date_event::SellExplorationDataEvent;
use sell_micro_resources_event::SellMicroResourcesEvent;
use sell_organic_data::SellOrganicDataEvent;
use sell_ship_on_rebuy_event::SellShipOnRebuyEvent;
use sell_suit_event::SellSuitEvent;
use sell_weapon_event::SellWeaponEvent;
use send_text_event::SendTextEvent;
use set_user_ship_name_event::SetUserShipNameEvent;
use shared_bookmark_to_squadron_event::ShardedBookmarkToSquadronEvent;
use shield_state_event::ShieldStateEvent;
use ship_locker_event::ShipLockerEvent;
use ship_targeted_event::ShipTargetedEvent;
use shipyard_buy_event::ShipyardBuyEvent;
use shipyard_event::ShipyardEvent;
use shipyard_new_event::ShipyardNewEvent;
use shipyard_sell_event::ShipyardSellEvent;
use shipyard_swap_event::ShipyardSwapEvent;
use shipyard_transfer_event::ShipyardTransferEvent;
use squadron_created_event::SquadronCreatedEvent;
use squadron_demotion_event::SquadronDemotionEvent;
use squadron_promotion_event::SquadronPromotionEvent;
use squadron_startup_event::SquadronStartupEvent;
use srv_destroyed_event::SRVDestroyedEvent;
use start_jump_event::StartJumpEvent;
use statistics_event::StatisticsEvent;
use stored_modules_event::StoredModulesEvent;
use stored_ships_event::StoredShipsEvent;
use suit_loadout_event::SuitLoadoutEvent;
use supercruise_destination_drop_event::SupercruiseDestinationDropEvent;
use supercruise_entry_event::SupercruiseEntryEvent;
use supercruise_exit_event::SupercruiseExitEvent;
use switch_suit_loadout_event::SwitchSuitLoadoutEvent;
use synthasis_event::SynthesisEvent;
use technology_broker_event::TechnologyBrokerEvent;
use touchdown_event::TouchdownEvent;
use trade_micro_resources_event::TradeMicroResourcesEvent;
use transfer_micro_resources_event::TransferMicroResourcesEvent;
use under_attack_event::UnderAttackEvent;
use undocked_event::UndockedEvent;
use upgrade_suit_event::UpgradeSuitEvent;
use upgrade_weapon_event::UpgradeWeaponEvent;
use use_consumable_event::UseConsumableEvent;
use uss_drop_event::USSDropEvent;
use vehicle_switch_event::VehicleSwitchEvent;
use wing_add_event::WingAddEvent;
use wing_invite_event::WingInviteEvent;
use wing_join_event::WingJoinEvent;
use won_a_trophy_for_squadron_event::WonATrophyForSquadronEvent;

use crate::logs::content::log_event_content::community_goal_event::CommunityGoalEvent;
use crate::logs::content::log_event_content::ship_redeemed::ShipRedeemed;
use crate::logs::content::log_event_content::shipyard_redeem::ShipyardRedeem;
use crate::logs::content::log_event_content::start_jump_event::StartJumpType;
use crate::modules::partials::PartialSystemInfo;

pub mod afmu_repairs_event;
pub mod applied_to_squadron_event;
pub mod approach_body_event;
pub mod approach_settlement_event;
pub mod asteroid_cracked_event;
pub mod backpack_change_event;
pub mod backpack_event;
pub mod book_dropship_event;
pub mod book_taxi_event;
pub mod bounty_event;
pub mod buy_ammo_event;
pub mod buy_drones_event;
pub mod buy_exploration_data_event;
pub mod buy_micro_resource_event;
pub mod buy_suit_event;
pub mod buy_trade_data_event;
pub mod buy_weapon_event;
pub mod cancel_dropship_event;
pub mod cancel_taxi_event;
pub mod cap_ship_bond_event;
pub mod cargo_depot_event;
pub mod cargo_event;
pub mod cargo_transfer_event;
pub mod carrier_bank_transfer_event;
pub mod carrier_buy_event;
pub mod carrier_cancel_decommission_event;
pub mod carrier_crew_services_event;
pub mod carrier_decommision_event;
pub mod carrier_deposit_fuel_event;
pub mod carrier_docking_permission_event;
pub mod carrier_finance_event;
pub mod carrier_jump_cancelled_event;
pub mod carrier_jump_event;
pub mod carrier_jump_request_event;
pub mod carrier_module_pack_event;
pub mod carrier_name_changed_event;
pub mod carrier_ship_pack_event;
pub mod carrier_stats_event;
pub mod carrier_trade_order_event;
pub mod change_crew_role_event;
pub mod clear_impound_event;
pub mod clear_saved_game_event;
pub mod codex_entry_event;
pub mod collect_cargo_event;
pub mod collect_items_event;
pub mod commander_event;
pub mod commit_crime_event;
pub mod community_goal_discard_event;
pub mod community_goal_event;
pub mod community_goal_join_event;
pub mod community_goal_reward_event;
pub mod continued_event;
pub mod create_suit_loadout_event;
pub mod crew_assign_event;
pub mod crew_fire_event;
pub mod crew_hire_event;
pub mod crew_launch_fighter_event;
pub mod crew_member_joins_event;
pub mod crew_member_quits_event;
pub mod crew_member_role_change_event;
pub mod crime_victim_event;
pub mod data_scanned_event;
pub mod datalink_scan_event;
pub mod datalink_voucher_event;
pub mod delete_suit_loadout_event;
pub mod died_event;
pub mod disbanded_squadron_event;
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
pub mod drop_ship_deploy_event;
pub mod eject_cargo_event;
pub mod embark_event;
pub mod end_crew_session_event;
pub mod engineer_contribution_event;
pub mod engineer_craft_event;
pub mod engineer_progress_event;
pub mod escape_interdiction_event;
pub mod faction_kill_bond_event;
pub mod fetch_remote_module_event;
pub mod fighter_destroyed_event;
pub mod fighter_rebuilt_event;
pub mod file_header_event;
pub mod friends_event;
pub mod fs_embark_event;
pub mod fsd_jump_event;
pub mod fsd_target_event;
pub mod fss_all_bodies_found_event;
pub mod fss_body_signals_event;
pub mod fss_discovery_scan_event;
pub mod fss_signal_discovered_event;
pub mod fuel_scoop_event;
pub mod hull_damage_event;
pub mod interdicted_event;
pub mod interdiction_event;
pub mod invited_to_squadron_event;
pub mod jet_cone_boost_event;
pub mod jet_cone_damage_event;
pub mod join_a_crew_event;
pub mod joined_squadron_event;
pub mod kick_crew_member_event;
pub mod kicked_from_squadron_event;
pub mod launch_drone_event;
pub mod launch_fighter_event;
pub mod launch_srv_event;
pub mod leave_body_event;
pub mod left_squadron_event;
pub mod liftoff_event;
pub mod load_game_event;
pub mod loadout_equip_module_event;
pub mod loadout_event;
pub mod loadout_remove_module_event;
pub mod location_event;
pub mod market_buy_event;
pub mod market_event;
pub mod market_sell_event;
pub mod mass_module_store_event;
pub mod material_collected_event;
pub mod material_discarded_event;
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
pub mod module_buy_and_store_event;
pub mod module_buy_event;
pub mod module_retrieve_event;
pub mod module_sell_event;
pub mod module_sell_remote_event;
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
pub mod powerplay_collect_event;
pub mod powerplay_defect_event;
pub mod powerplay_deliver_event;
pub mod powerplay_event;
pub mod powerplay_fast_track_event;
pub mod powerplay_join_event;
pub mod powerplay_leave_event;
pub mod powerplay_salary_event;
pub mod powerplay_vote_event;
pub mod progress_event;
pub mod promotion_event;
pub mod prospected_asteroid_event;
pub mod pvp_kill_event;
pub mod quit_a_crew_event;
pub mod rank_event;
pub mod reboot_repair_event;
pub mod receive_text_event;
pub mod redeem_voucher_event;
pub mod refuel_all_event;
pub mod refuel_partial_event;
pub mod rename_suit_loadout_event;
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
pub mod scientific_research_event;
pub mod screenshot_event;
pub mod search_and_rescue_event;
pub mod sell_drones_event;
pub mod sell_exploration_date_event;
pub mod sell_micro_resources_event;
pub mod sell_organic_data;
pub mod sell_ship_on_rebuy_event;
pub mod sell_suit_event;
pub mod sell_weapon_event;
pub mod send_text_event;
pub mod set_user_ship_name_event;
pub mod shared_bookmark_to_squadron_event;
pub mod shield_state_event;
pub mod ship_locker_event;
pub mod ship_redeemed;
pub mod ship_targeted_event;
pub mod shipyard_buy_event;
pub mod shipyard_event;
pub mod shipyard_new_event;
pub mod shipyard_redeem;
pub mod shipyard_sell_event;
pub mod shipyard_swap_event;
pub mod shipyard_transfer_event;
pub mod squadron_created_event;
pub mod squadron_demotion_event;
pub mod squadron_promotion_event;
pub mod squadron_startup_event;
pub mod srv_destroyed_event;
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
pub mod trade_micro_resources_event;
pub mod transfer_micro_resources_event;
pub mod under_attack_event;
pub mod undocked_event;
pub mod upgrade_suit_event;
pub mod upgrade_weapon_event;
pub mod use_consumable_event;
pub mod uss_drop_event;
pub mod vehicle_switch_event;
pub mod wing_add_event;
pub mod wing_invite_event;
pub mod wing_join_event;
pub mod won_a_trophy_for_squadron_event;

/// Enum containing all the possible events that can be found in a [JournalFile].
///
/// > **Note** not all events might already be parsed or be parsed correctly. If you feel that
/// > an existing entry is not parsed correctly or if there is a missing entry, please open an issue
/// > on [GitHub](https://github.com/rster2002/ed-journals/issues/new).
#[derive(Debug, Serialize, Deserialize, Kinded, Clone, PartialEq)]
#[serde(tag = "event")]
pub enum LogEventContent {
    // Startup
    Cargo(CargoEvent),
    ClearSavedGame(ClearSavedGameEvent),
    Commander(CommanderEvent),

    #[serde(rename = "Fileheader")]
    FileHeader(FileHeaderEvent),
    LoadGame(LoadGameEvent),
    Loadout(LoadoutEvent),
    Materials(MaterialsEvent),
    Missions(MissionsEvent),
    NewCommander(NewCommanderEvent),
    Passengers(PassengersEvent),
    Powerplay(PowerplayEvent),
    Progress(ProgressEvent),
    Rank(RankEvent),
    Reputation(ReputationEvent),
    Statistics(StatisticsEvent),

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
    CapShipBond(CapShipBondEvent),
    Died(DiedEvent),
    EscapeInterdiction(EscapeInterdictionEvent),
    FactionKillBond(FactionKillBondEvent),
    FighterDestroyed(FighterDestroyedEvent),
    HeatDamage,
    HeatWarning,
    HullDamage(HullDamageEvent),
    Interdicted(InterdictedEvent),
    Interdiction(InterdictionEvent),
    PVPKill(PVPKillEvent),
    SRVDestroyed(SRVDestroyedEvent),
    ShieldState(ShieldStateEvent),
    ShipTargeted(ShipTargetedEvent),
    UnderAttack(UnderAttackEvent),

    // Exploration
    BuyExplorationData(BuyExplorationDataEvent),
    CodexEntry(CodexEntryEvent),
    DiscoveryScan(DiscoveryScanEvent),
    FSSAllBodiesFound(FSSAllBodiesFoundEvent),
    FSSBodySignals(FSSBodySignalsEvent),
    FSSDiscoveryScan(FSSDiscoveryScan),
    FSSSignalDiscovered(FSSSignalDiscoveredEvent),
    MaterialCollected(MaterialCollectedEvent),
    MaterialDiscarded(MaterialDiscarded),
    MaterialDiscovered(MaterialDiscoveredEvent),
    MultiSellExplorationData(MultiSellExplorationDataEvent),
    NavBeaconScan(NavBeaconScanEvent),
    SAAScanComplete(SAAScanCompleteEvent),
    SAASignalsFound(SAASignalsFoundEvent),
    Scan(ScanEvent),
    ScanBaryCentre(ScanBaryCentreEvent),
    SellExplorationData(SellExplorationDataEvent),
    Screenshot(ScreenshotEvent),

    // Trade
    AsteroidCracked(AsteroidCrackedEvent),
    BuyTradeData(BuyTradeDateEvent),
    CollectCargo(CollectCargoEvent),
    EjectCargo(EjectCargoEvent),
    MarketBuy(MarketBuyEvent),
    MarketSell(MarketSellEvent),
    MiningRefined(MiningRefinedEvent),

    // Station services
    BuyAmmo(BuyAmmoEvent),
    BuyDrones(BuyDronesEvent),
    CargoDepot(CargoDepotEvent),
    CommunityGoal(CommunityGoalEvent),
    CommunityGoalDiscard(CommunityGoalDiscardEvent),
    CommunityGoalJoin(CommunityGoalJoinEvent),
    CommunityGoalReward(CommunityGoalRewardEvent),
    CrewAssign(CrewAssignEvent),
    CrewFire(CrewFireEvent),
    CrewHire(CrewHireEvent),
    EngineerContribution(EngineerContributionEvent),
    EngineerCraft(EngineerCraftEvent),
    EngineerProgress(EngineerProgressEvent),
    FetchRemoteModule(FetchRemoteModuleEvent),
    Market(MarketEvent),
    MassModuleStore(MassModuleStoreEvent),
    MaterialTrade(MaterialTradeEvent),
    MissionAbandoned(MissionAbandonedEvent),
    MissionAccepted(MissionAcceptedEvent),
    MissionCompleted(MissionCompletedEvent),
    MissionFailed(MissionFailedEvent),
    MissionRedirected(MissionRedirectedEvent),
    ModuleBuy(ModuleBuyEvent),
    ModuleBuyAndStore(ModuleBuyAndStoreEvent),
    ModuleRetrieve(ModuleRetrieveEvent),
    ModuleSell(ModuleSellEvent),
    ModuleSellRemote(ModuleSellRemoteEvent),
    ModuleStore(ModuleStoreEvent),
    ModuleSwap(ModuleSwapEvent),
    Outfitting(OutfittingEvent),
    PayBounties(PayBountiesEvent),
    PayFines(PayFinesEvent),
    RedeemVoucher(RedeemVoucherEvent),
    RefuelAll(RefuelAllEvent),
    RefuelPartial(RefuelPartialEvent),
    Repair(RepairEvent),
    RepairAll(RepairAllEvent),
    RestockVehicle(RestockVehicleEvent),
    ScientificResearch(ScientificResearchEvent),
    SearchAndRescue(SearchAndRescueEvent),
    SellDrones(SellDronesEvent),
    SellShipOnRebuy(SellShipOnRebuyEvent),
    SetUserShipName(SetUserShipNameEvent),
    Shipyard(ShipyardEvent),
    ShipyardBuy(ShipyardBuyEvent),
    ShipyardNew(ShipyardNewEvent),
    ShipyardSell(ShipyardSellEvent),
    ShipyardSwap(ShipyardSwapEvent),
    ShipyardTransfer(ShipyardTransferEvent),
    ShipyardRedeem(ShipyardRedeem),
    ShipRedeemed(ShipRedeemed),
    StoredModules(StoredModulesEvent),
    StoredShips(StoredShipsEvent),
    TechnologyBroker(TechnologyBrokerEvent),
    ClearImpound(ClearImpoundEvent),

    // Powerplay
    PowerplayCollect(PowerplayCollectEvent),
    PowerplayDefect(PowerplayDefectEvent),
    PowerplayDeliver(PowerplayDeliverEvent),
    PowerplayFastTrack(PowerplayFastTrackEvent),
    PowerplayJoin(PowerplayJoinEvent),
    PowerplayLeave(PowerplayLeaveEvent),
    PowerplaySalary(PowerplaySalaryEvent),
    PowerplayVote(PowerplayVoteEvent),

    // Squadrons
    AppliedToSquadron(AppliedToSquadronEvent),
    DisbandedSquadron(DisbandedSquadronEvent),
    InvitedToSquadron(InvitedToSquadronEvent),
    JoinedSquadron(JoinedSquadronEvent),
    KickedFromSquadron(KickedFromSquadronEvent),
    LeftSquadron(LeftSquadronEvent),
    SharedBookmarkToSquadron(ShardedBookmarkToSquadronEvent),
    SquadronCreated(SquadronCreatedEvent),
    SquadronDemotion(SquadronDemotionEvent),
    SquadronPromotion(SquadronPromotionEvent),
    SquadronStartup(SquadronStartupEvent),
    WonATrophyForSquadron(WonATrophyForSquadronEvent),

    // Fleet carriers
    CarrierJump(CarrierJumpEvent),
    CarrierBuy(CarrierBuyEvent),
    CarrierStats(CarrierStatsEvent),
    CarrierJumpRequest(CarrierJumpRequestEvent),
    CarrierDecommission(CarrierDecommissionEvent),
    CarrierCancelDecommission(CarrierCancelDecommissionEvent),
    CarrierBankTransfer(CarrierBankTransferEvent),
    CarrierDepositFuel(CarrierDepositFuelEvent),
    CarrierCrewServices(CarrierCrewServicesEvent),
    CarrierFinance(CarrierFinanceEvent),
    CarrierShipPack(CarrierShipPackEvent),
    CarrierModulePack(CarrierModulePackEvent),
    CarrierTradeOrder(CarrierTradeOrderEvent),
    CarrierDockingPermission(CarrierDockingPermissionEvent),
    CarrierNameChange(CarrierNameChangeEvent),
    CarrierJumpCancelled(CarrierJumpCancelled),

    // Odyssey
    Backpack(BackpackEvent),
    BackpackChange(BackpackChangeEvent),
    BookDropship(BookDropshipEvent),
    BookTaxi(BookTaxiEvent),
    BuyMicroResources(BuyMicroResourceEvent),
    BuySuit(BuySuitEvent),
    BuyWeapon(BuyWeaponEvent),
    CancelDropship(CancelDropshipEvent),
    CancelTaxi(CancelTaxiEvent),
    CollectItems(CollectItemsEvent),
    CreateSuitLoadout(CreateSuitLoadoutEvent),
    DeleteSuitLoadout(DeleteSuitLoadoutEvent),
    Disembark(DisembarkEvent),
    DropItems(DropItemsEvent),
    DropshipDeploy(DropshipDeployEvent),
    Embark(EmbarkEvent),
    FCMaterials(FCMaterialsEvent),
    LoadoutEquipModule(LoadoutEquipModuleEvent),
    LoadoutRemoveModule(LoadoutRemoveModuleEvent),
    RenameSuitLoadout(RenameSuitLoadoutEvent),
    Resupply,
    ScanOrganic(ScanOrganicEvent),
    SellMicroResources(SellMicroResourcesEvent),
    SellOrganicData(SellOrganicDataEvent),
    SellSuit(SellSuitEvent),
    SellWeapon(SellWeaponEvent),
    ShipLocker(ShipLockerEvent),
    SwitchSuitLoadout(SwitchSuitLoadoutEvent),
    SuitLoadout(SuitLoadoutEvent),
    TransferMicroResources(TransferMicroResourcesEvent),
    TradeMicroResources(TradeMicroResourcesEvent),
    UpgradeSuit(UpgradeSuitEvent),
    UpgradeWeapon(UpgradeWeaponEvent),
    UseConsumable(UseConsumableEvent),

    // Other
    #[serde(rename = "AfmuRepairs")]
    AFMURepairs(AFMURepairsEvent),
    ApproachSettlement(ApproachSettlementEvent),
    CargoTransfer(CargoTransferEvent),
    ChangeCrewRole(ChangeCrewRoleEvent),
    CockpitBreached,
    CommitCrime(CommitCrimeEvent),
    Continued(ContinuedEvent),
    CrewLaunchFighter(CrewLaunchFighterEvent),
    CrewMemberJoins(CrewMemberJoinsEvent),
    CrewMemberQuits(CrewMemberQuitsEvent),
    CrewMemberRoleChange(CrewMemberRoleChangeEvent),
    CrimeVictim(CrimeVictimEvent),
    DataScanned(DataScannedEvent),
    DatalinkVoucher(DatalinkVoucherEvent),
    DatalinkScan(DatalinkScanEvent),
    DockFighter(DockFighterEvent),
    DockSRV(DockSRVEvent),
    EndCrewSession(EndCrewSessionEvent),
    FighterRebuilt(FighterRebuiltEvent),
    Friends(FriendsEvent),
    FuelScoop(FuelScoopEvent),
    JetConeBoost(JetConeBoostEvent),
    JetConeDamage(JetConeDamageEvent),
    JoinACrew(JoinACrewEvent),
    KickCrewMember(KickCrewMemberEvent),
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
    QuitACrew(QuitACrewEvent),
    RebootRepair(RebootRepairEvent),
    ReceiveText(ReceiveTextEvent),
    RepairDrone(RepairDroneEvent),
    ReservoirReplenished(ReservoirReplenishedEvent),
    Resurrect(ResurrectEvent),
    Scanned(ScannedEvent),
    SelfDestruct,
    SendText(SendTextEvent),
    Shutdown,
    SupercruiseDestinationDrop(SupercruiseDestinationDropEvent),
    Synthesis(SynthesisEvent),
    SystemsShutdown,
    USSDrop(USSDropEvent),
    VehicleSwitch(VehicleSwitchEvent),
    WingAdd(WingAddEvent),
    WingInvite(WingInviteEvent),
    WingJoin(WingJoinEvent),
    WingLeave,

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(Value),
}

impl LogEventContent {
    pub fn system_address(&self) -> Option<u64> {
        Some(match self {
            LogEventContent::Location(event) => event.location_info.system_address,
            LogEventContent::FSDJump(event) => event.system_info.system_address,
            LogEventContent::CarrierJump(event) => event.system_info.system_address,
            LogEventContent::ApproachSettlement(event) => event.system_address,
            LogEventContent::CarrierBuy(event) => event.system_address,
            LogEventContent::CarrierJumpRequest(event) => event.system_address,
            LogEventContent::CodexEntry(event) => event.system_address,
            LogEventContent::DiscoveryScan(event) => event.system_address,
            LogEventContent::Disembark(event) => event.system_address,
            LogEventContent::Docked(event) => event.system_address,
            LogEventContent::DropshipDeploy(event) => event.system_address,
            LogEventContent::Embark(event) => event.system_address,
            LogEventContent::FSSAllBodiesFound(event) => event.system_address,
            LogEventContent::FSSBodySignals(event) => event.system_address,
            LogEventContent::FSSDiscoveryScan(event) => event.system_address,
            LogEventContent::FSSSignalDiscovered(event) => event.system_address,
            LogEventContent::LeaveBody(event) => event.system_address,
            LogEventContent::Liftoff(event) => event.system_address,
            LogEventContent::NavBeaconScan(event) => event.system_address,
            LogEventContent::SAAScanComplete(event) => event.system_address,
            LogEventContent::SAASignalsFound(event) => event.system_address,
            LogEventContent::Scan(event) => event.system_address,
            LogEventContent::ScanBaryCentre(event) => event.system_address,
            LogEventContent::ScanOrganic(event) => event.system_address,
            LogEventContent::StartJump(event) => match event.jump {
                StartJumpType::Hyperspace { system_address, .. } => system_address,
                StartJumpType::Supercruise => return None,
            },
            LogEventContent::SupercruiseEntry(event) => event.system_address,
            LogEventContent::SupercruiseExit(event) => event.system_address,
            LogEventContent::Touchdown(event) => event.system_address,
            _ => return None,
        })
    }

    pub fn star_name(&self) -> Option<&str> {
        Some(match self {
            LogEventContent::Location(event) => &event.location_info.star_system,
            LogEventContent::FSDJump(event) => &event.system_info.star_system,
            LogEventContent::CarrierJump(event) => &event.system_info.star_system,
            LogEventContent::CarrierBuy(event) => &event.location,
            LogEventContent::CarrierJumpRequest(event) => &event.system_name,
            LogEventContent::CodexEntry(event) => &event.system,
            LogEventContent::Disembark(event) => &event.star_system,
            LogEventContent::Docked(event) => &event.star_system,
            LogEventContent::DropshipDeploy(event) => &event.star_system,
            LogEventContent::Embark(event) => &event.star_system,
            LogEventContent::FSSAllBodiesFound(event) => &event.system_name,
            LogEventContent::FSSDiscoveryScan(event) => &event.system_name,
            LogEventContent::LeaveBody(event) => &event.star_system,
            LogEventContent::Liftoff(event) => &event.star_system,
            LogEventContent::Scan(event) => &event.star_system,
            LogEventContent::ScanBaryCentre(event) => &event.star_system,
            LogEventContent::StartJump(event) => match &event.jump {
                StartJumpType::Hyperspace { star_system, .. } => star_system,
                StartJumpType::Supercruise => return None,
            },
            LogEventContent::SupercruiseEntry(event) => &event.star_system,
            LogEventContent::SupercruiseExit(event) => &event.star_system,
            LogEventContent::Touchdown(event) => &event.star_system,
            _ => return None,
        })
    }

    pub fn small_system_info(&self) -> Option<PartialSystemInfo> {
        let system_address = self.system_address()?;
        let star_name = self.star_name()?;

        Some(PartialSystemInfo {
            system_address,
            star_name: star_name.to_string(),
        })
    }

    pub fn body_id(&self) -> Option<u8> {
        Some(match self {
            LogEventContent::Location(event) => event.location_info.body_id,
            LogEventContent::FSDJump(event) => event.system_info.body_id,
            LogEventContent::CarrierJump(event) => event.system_info.body_id,
            LogEventContent::CarrierJump(event) => event.system_info.body_id,
            LogEventContent::ApproachSettlement(event) => event.body_id,
            LogEventContent::CarrierJumpRequest(event) => event.body_id,
            LogEventContent::CodexEntry(event) => event.body_id,
            LogEventContent::DropshipDeploy(event) => event.body_id,
            LogEventContent::FSSBodySignals(event) => event.body_id,
            LogEventContent::LeaveBody(event) => event.body_id,
            LogEventContent::SAAScanComplete(event) => event.body_id,
            LogEventContent::SAASignalsFound(event) => event.body_id,
            LogEventContent::ScanBaryCentre(event) => event.body_id,
            LogEventContent::Scan(event) => event.body_id,
            LogEventContent::Touchdown(event) => event.body_id,
            LogEventContent::ScanOrganic(event) => event.body,
            LogEventContent::Touchdown(event) => event.body_id,
            _ => return None,
        })
    }

    pub fn body_name(&self) -> Option<&str> {
        Some(match self {
            LogEventContent::Location(event) => &event.location_info.body,
            LogEventContent::FSDJump(event) => &event.system_info.body,
            LogEventContent::CarrierJump(event) => &event.system_info.body,
            LogEventContent::CarrierJump(event) => &event.system_info.body,
            LogEventContent::ApproachSettlement(event) => &event.body_name,
            LogEventContent::CarrierJumpRequest(event) => match &event.body {
                Some(name) => name,
                None => return None,
            },
            LogEventContent::DropshipDeploy(event) => &event.body,
            LogEventContent::FSSBodySignals(event) => &event.body_name,
            LogEventContent::LeaveBody(event) => &event.body,
            LogEventContent::SAAScanComplete(event) => &event.body_name,
            LogEventContent::Scan(event) => &event.body_name,
            LogEventContent::Touchdown(event) => &event.body,
            _ => return None,
        })
    }
}
