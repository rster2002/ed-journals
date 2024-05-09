use crate::journal_event_content::book_dropship_event::BookDropshipEvent;
use crate::journal_event_content::buy_trade_data_event::BuyTradeDateEvent;
use crate::journal_event_content::cancel_dropship_event::CancelDropshipEvent;
use crate::journal_event_content::cancel_taxi_event::CancelTaxiEvent;
use crate::journal_event_content::cap_ship_bond_event::CapShipBondEvent;
use crate::journal_event_content::carrier_cancel_decommission_event::CarrierCancelDecommissionEvent;
use crate::journal_event_content::carrier_decommision_event::CarrierDecommissionEvent;
use crate::journal_event_content::carrier_docking_permission_event::CarrierDockingPermissionEvent;
use crate::journal_event_content::carrier_module_pack_event::CarrierModulePackEvent;
use crate::journal_event_content::carrier_name_changed_event::CarrierNameChangedEvent;
use crate::journal_event_content::carrier_ship_pack_event::CarrierShipPackEvent;
use crate::journal_event_content::carrier_trade_order_event::CarrierTradeOrderEvent;
use crate::journal_event_content::change_crew_role_event::ChangeCrewRoleEvent;
use crate::journal_event_content::clear_impound_event::ClearImpoundEvent;
use crate::journal_event_content::community_goal_discard_event::CommunityGoalDiscardEvent;
use crate::journal_event_content::community_goal_event::CommunityGoalEventGoal;
use crate::journal_event_content::community_goal_join_event::CommunityGoalJoinEvent;
use crate::journal_event_content::community_goal_reward_event::CommunityGoalRewardEvent;
use crate::journal_event_content::continued_event::ContinuedEvent;
use crate::journal_event_content::crew_fire_event::CrewFireEvent;
use crate::journal_event_content::crew_launch_fighter_event::CrewLaunchFighterEvent;
use crate::journal_event_content::crew_member_joins_event::CrewMemberJoinsEvent;
use crate::journal_event_content::crew_member_quits_event::CrewMemberQuitsEvent;
use crate::journal_event_content::crew_member_role_change_event::CrewMemberRoleChangeEvent;
use crate::journal_event_content::datalink_voucher_event::DatalinkVoucherEvent;
use crate::journal_event_content::delete_suit_loadout_event::DeleteSuitLoadoutEvent;
use crate::journal_event_content::disbanded_squadron_event::DisbandedSquadronEvent;
use crate::journal_event_content::drop_ship_deploy_event::DropShipDeployEvent;
use crate::journal_event_content::end_crew_session_event::EndCrewSessionEvent;
use crate::journal_event_content::invited_to_squadron_event::InvitedToSquadronEvent;
use crate::journal_event_content::jet_cone_damage_event::JetConeDamageEvent;
use crate::journal_event_content::join_a_crew_event::JoinACrewEvent;
use crate::journal_event_content::joined_squadron_event::JoinedSquadronEvent;
use crate::journal_event_content::kick_crew_member_event::KickCrewMemberEvent;
use crate::journal_event_content::kicked_from_squadron_event::KickedFromSquadronEvent;
use crate::journal_event_content::left_squadron_event::LeftSquadronEvent;
use crate::journal_event_content::loadout_remove_module_event::LoadoutRemoveModuleEvent;
use crate::journal_event_content::mass_module_store_event::MassModuleStoreEvent;
use crate::journal_event_content::material_discarded_event::MaterialDiscarded;
use crate::journal_event_content::powerplay_collect_event::PowerplayCollectEvent;
use crate::journal_event_content::powerplay_defect_event::PowerplayDefectEvent;
use crate::journal_event_content::powerplay_deliver_event::PowerplayDeliverEvent;
use crate::journal_event_content::powerplay_fast_track_event::PowerplayFastTrackEvent;
use crate::journal_event_content::powerplay_join_event::PowerplayJoinEvent;
use crate::journal_event_content::powerplay_vote_event::PowerplayVoteEvent;
use crate::journal_event_content::pvp_kill_event::PVPKillEvent;
use crate::journal_event_content::quit_a_crew_event::QuitACrewEvent;
use crate::journal_event_content::reboot_repair_event::RebootRepairEvent;
use crate::journal_event_content::refuel_partial_event::RefuelPartialEvent;
use crate::journal_event_content::rename_suit_loadout_event::RenameSuitLoadoutEvent;
use crate::journal_event_content::scientific_research_event::ScientificResearchEvent;
use crate::journal_event_content::screenshot_event::ScreenshotEvent;
use crate::journal_event_content::sell_exploration_date_event::SellExplorationDataEvent;
use crate::journal_event_content::sell_micro_resources_event::SellMicroResourcesEvent;
use crate::journal_event_content::sell_ship_on_rebuy_event::SellShipOnRebuyEvent;
use crate::journal_event_content::sell_suit_event::SellSuitEvent;
use crate::journal_event_content::sell_weapon_event::SellWeaponEvent;
use crate::journal_event_content::shared_bookmark_to_squadron_event::ShardedBookmarkToSquadronEvent;
use crate::journal_event_content::shipyard_sell_event::ShipyardSellEvent;
use crate::journal_event_content::squadron_created_event::SquadronCreatedEvent;
use crate::journal_event_content::squadron_demotion_event::SquadronDemotionEvent;
use crate::journal_event_content::squadron_promotion_event::SquadronPromotionEvent;
use crate::journal_event_content::trade_micro_resources_event::TradeMicroResourcesEvent;
use crate::journal_event_content::transfer_micro_resources_event::TransferMicroResourcesEvent;
use crate::journal_event_content::upgrade_suit_event::UpgradeSuitEvent;
use crate::journal_event_content::upgrade_weapon_event::UpgradeWeaponEvent;
use crate::journal_event_content::won_a_trophy_for_squadron_event::WonATrophyForSquadronEvent;
use kinded::Kinded;
use serde::Deserialize;

use crate::models::journal_event_content::afmu_repairs_event::AFMURepairsEvent;
use crate::models::journal_event_content::applied_to_squadron_event::AppliedToSquadronEvent;
use crate::models::journal_event_content::approach_body_event::ApproachBodyEvent;
use crate::models::journal_event_content::approach_settlement_event::ApproachSettlementEvent;
use crate::models::journal_event_content::asteroid_cracked_event::AsteroidCrackedEvent;
use crate::models::journal_event_content::backpack_change_event::BackpackChangeEvent;
use crate::models::journal_event_content::backpack_event::BackpackEvent;
use crate::models::journal_event_content::book_taxi_event::BookTaxiEvent;
use crate::models::journal_event_content::bounty_event::BountyEvent;
use crate::models::journal_event_content::buy_ammo_event::BuyAmmoEvent;
use crate::models::journal_event_content::buy_drones_event::BuyDronesEvent;
use crate::models::journal_event_content::buy_exploration_data_event::BuyExplorationDataEvent;
use crate::models::journal_event_content::buy_micro_resource_event::BuyMicroResourceEvent;
use crate::models::journal_event_content::buy_suit_event::BuySuitEvent;
use crate::models::journal_event_content::buy_weapon_event::BuyWeaponEvent;
use crate::models::journal_event_content::cargo_depot_event::CargoDepotEvent;
use crate::models::journal_event_content::cargo_event::CargoEvent;
use crate::models::journal_event_content::cargo_transfer_event::CargoTransferEvent;
use crate::models::journal_event_content::carrier_bank_transfer_event::CarrierBankTransferEvent;
use crate::models::journal_event_content::carrier_buy_event::CarrierBuyEvent;
use crate::models::journal_event_content::carrier_crew_services_event::CarrierCrewServicesEvent;
use crate::models::journal_event_content::carrier_deposit_fuel_event::CarrierDepositFuelEvent;
use crate::models::journal_event_content::carrier_finance_event::CarrierFinanceEvent;
use crate::models::journal_event_content::carrier_jump_cancelled_event::CarrierJumpCancelled;
use crate::models::journal_event_content::carrier_jump_event::CarrierJumpEvent;
use crate::models::journal_event_content::carrier_jump_request_event::CarrierJumpRequestEvent;
use crate::models::journal_event_content::carrier_stats_event::CarrierStatsEvent;
use crate::models::journal_event_content::clear_saved_game_event::ClearSavedGameEvent;
use crate::models::journal_event_content::codex_entry_event::CodexEntryEvent;
use crate::models::journal_event_content::collect_cargo_event::CollectCargoEvent;
use crate::models::journal_event_content::collect_items_event::CollectItemsEvent;
use crate::models::journal_event_content::commander_event::CommanderEvent;
use crate::models::journal_event_content::commit_crime_event::CommitCrimeEvent;
use crate::models::journal_event_content::create_suit_loadout_event::CreateSuitLoadoutEvent;
use crate::models::journal_event_content::crew_assign_event::CrewAssignEvent;
use crate::models::journal_event_content::crew_hire_event::CrewHireEvent;
use crate::models::journal_event_content::crime_victim_event::CrimeVictimEvent;
use crate::models::journal_event_content::data_scanned_event::DataScannedEvent;
use crate::models::journal_event_content::datalink_scan_event::DatalinkScanEvent;
use crate::models::journal_event_content::died_event::DiedEvent;
use crate::models::journal_event_content::discovery_scan_event::DiscoveryScanEvent;
use crate::models::journal_event_content::disembark_event::DisembarkEvent;
use crate::models::journal_event_content::dock_fighter_event::DockFighterEvent;
use crate::models::journal_event_content::dock_srv_event::DockSRVEvent;
use crate::models::journal_event_content::docked_event::DockedEvent;
use crate::models::journal_event_content::docking_cancelled_event::DockingCancelled;
use crate::models::journal_event_content::docking_denied_event::DockingDeniedEvent;
use crate::models::journal_event_content::docking_granted_event::DockingGrantedEvent;
use crate::models::journal_event_content::docking_requested_event::DockingRequestedEvent;
use crate::models::journal_event_content::docking_timeout_event::DockingTimeoutEvent;
use crate::models::journal_event_content::drop_items_event::DropItemsEvent;
use crate::models::journal_event_content::eject_cargo_event::EjectCargoEvent;
use crate::models::journal_event_content::embark_event::EmbarkEvent;
use crate::models::journal_event_content::engineer_contribution_event::EngineerContributionEvent;
use crate::models::journal_event_content::engineer_craft_event::EngineerCraftEvent;
use crate::models::journal_event_content::engineer_progress_event::EngineerProgressEvent;
use crate::models::journal_event_content::escape_interdiction_event::EscapeInterdictionEvent;
use crate::models::journal_event_content::faction_kill_bond_event::FactionKillBondEvent;
use crate::models::journal_event_content::fetch_remote_module_event::FetchRemoteModuleEvent;
use crate::models::journal_event_content::fighter_destroyed_event::FighterDestroyedEvent;
use crate::models::journal_event_content::fighter_rebuilt_event::FighterRebuiltEvent;
use crate::models::journal_event_content::file_header_event::FileHeaderEvent;
use crate::models::journal_event_content::friends_event::FriendsEvent;
use crate::models::journal_event_content::fs_embark_event::FCMaterialsEvent;
use crate::models::journal_event_content::fsd_jump_event::FSDJumpEvent;
use crate::models::journal_event_content::fsd_target_event::FSDTargetEvent;
use crate::models::journal_event_content::fss_all_bodies_found_event::FSSAllBodiesFoundEvent;
use crate::models::journal_event_content::fss_body_signals_event::FSSBodySignalsEvent;
use crate::models::journal_event_content::fss_discovery_scan_event::FSSDiscoveryScan;
use crate::models::journal_event_content::fss_signal_discovered_event::FSSSignalDiscoveredEvent;
use crate::models::journal_event_content::fuel_scoop_event::FuelScoopEvent;
use crate::models::journal_event_content::hull_damage_event::HullDamageEvent;
use crate::models::journal_event_content::interdicted_event::InterdictedEvent;
use crate::models::journal_event_content::interdiction_event::InterdictionEvent;
use crate::models::journal_event_content::jet_cone_boost_event::JetConeBoostEvent;
use crate::models::journal_event_content::launch_drone_event::LaunchDroneEvent;
use crate::models::journal_event_content::launch_fighter_event::LaunchFighterEvent;
use crate::models::journal_event_content::launch_srv_event::LaunchSRVEvent;
use crate::models::journal_event_content::leave_body_event::LeaveBodyEvent;
use crate::models::journal_event_content::liftoff_event::LiftoffEvent;
use crate::models::journal_event_content::load_game_event::LoadGameEvent;
use crate::models::journal_event_content::loadout_equip_module_event::LoadoutEquipModuleEvent;
use crate::models::journal_event_content::loadout_event::LoadoutEvent;
use crate::models::journal_event_content::location_event::LocationEvent;
use crate::models::journal_event_content::market_buy_event::MarketBuyEvent;
use crate::models::journal_event_content::market_event::MarketEvent;
use crate::models::journal_event_content::market_sell_event::MarketSellEvent;
use crate::models::journal_event_content::material_collected_event::MaterialCollectedEvent;
use crate::models::journal_event_content::material_discovered_event::MaterialDiscoveredEvent;
use crate::models::journal_event_content::material_trade_event::MaterialTradeEvent;
use crate::models::journal_event_content::materials_event::MaterialsEvent;
use crate::models::journal_event_content::mining_refined::MiningRefinedEvent;
use crate::models::journal_event_content::mission_abandoned_event::MissionAbandonedEvent;
use crate::models::journal_event_content::mission_accepted_event::MissionAcceptedEvent;
use crate::models::journal_event_content::mission_completed_event::MissionCompletedEvent;
use crate::models::journal_event_content::mission_failed_event::MissionFailedEvent;
use crate::models::journal_event_content::mission_redirected_event::MissionRedirectedEvent;
use crate::models::journal_event_content::missions_event::MissionsEvent;
use crate::models::journal_event_content::module_buy_and_store_event::ModuleBuyAndStoreEvent;
use crate::models::journal_event_content::module_buy_event::ModuleBuyEvent;
use crate::models::journal_event_content::module_retrieve_event::ModuleRetrieveEvent;
use crate::models::journal_event_content::module_sell_event::ModuleSellEvent;
use crate::models::journal_event_content::module_sell_remote_event::ModuleSellRemoteEvent;
use crate::models::journal_event_content::module_store_event::ModuleStoreEvent;
use crate::models::journal_event_content::module_swap_event::ModuleSwapEvent;
use crate::models::journal_event_content::multi_sell_exploration_data_event::MultiSellExplorationDataEvent;
use crate::models::journal_event_content::music_event::MusicEvent;
use crate::models::journal_event_content::nav_beacon_scan_event::NavBeaconScanEvent;
use crate::models::journal_event_content::new_commander_event::NewCommanderEvent;
use crate::models::journal_event_content::npc_crew_rank_event::NPCCrewRankEvent;
use crate::models::journal_event_content::npc_crew_wage_paid_event::NPCCrewWagePaidEvent;
use crate::models::journal_event_content::outfitting_event::OutfittingEvent;
use crate::models::journal_event_content::passengers_event::PassengersEvent;
use crate::models::journal_event_content::pay_bounties_event::PayBountiesEvent;
use crate::models::journal_event_content::pay_fines_event::PayFinesEvent;
use crate::models::journal_event_content::powerplay_event::PowerplayEvent;
use crate::models::journal_event_content::powerplay_leave_event::PowerplayLeaveEvent;
use crate::models::journal_event_content::powerplay_salary_event::PowerplaySalaryEvent;
use crate::models::journal_event_content::progress_event::ProgressEvent;
use crate::models::journal_event_content::promotion_event::PromotionEvent;
use crate::models::journal_event_content::prospected_asteroid_event::ProspectedAsteroidEvent;
use crate::models::journal_event_content::rank_event::RankEvent;
use crate::models::journal_event_content::receive_text_event::ReceiveTextEvent;
use crate::models::journal_event_content::redeem_voucher_event::RedeemVoucherEvent;
use crate::models::journal_event_content::refuel_all_event::RefuelAllEvent;
use crate::models::journal_event_content::repair_all_event::RepairAllEvent;
use crate::models::journal_event_content::repair_drone_event::RepairDroneEvent;
use crate::models::journal_event_content::repair_event::RepairEvent;
use crate::models::journal_event_content::reputation_event::ReputationEvent;
use crate::models::journal_event_content::reservoir_replenished_event::ReservoirReplenishedEvent;
use crate::models::journal_event_content::restock_vehicle_event::RestockVehicleEvent;
use crate::models::journal_event_content::resurrect_event::ResurrectEvent;
use crate::models::journal_event_content::saa_scan_complete_event::SAAScanCompleteEvent;
use crate::models::journal_event_content::saa_signals_found_event::SAASignalsFoundEvent;
use crate::models::journal_event_content::scan_bary_centre_event::ScanBaryCentreEvent;
use crate::models::journal_event_content::scan_event::ScanEvent;
use crate::models::journal_event_content::scan_organic_event::ScanOrganicEvent;
use crate::models::journal_event_content::scanned_event::ScannedEvent;
use crate::models::journal_event_content::search_and_rescue_event::SearchAndRescueEvent;
use crate::models::journal_event_content::sell_drones_event::SellDronesEvent;
use crate::models::journal_event_content::sell_organic_data::SellOrganicDataEvent;
use crate::models::journal_event_content::send_text_event::SendTextEvent;
use crate::models::journal_event_content::set_user_ship_name_event::SetUserShipNameEvent;
use crate::models::journal_event_content::shield_state_event::ShieldStateEvent;
use crate::models::journal_event_content::ship_locker_event::ShipLockerEvent;
use crate::models::journal_event_content::ship_targeted_event::ShipTargetedEvent;
use crate::models::journal_event_content::shipyard_buy_event::ShipyardBuyEvent;
use crate::models::journal_event_content::shipyard_event::ShipyardEvent;
use crate::models::journal_event_content::shipyard_new_event::ShipyardNewEvent;
use crate::models::journal_event_content::shipyard_swap_event::ShipyardSwapEvent;
use crate::models::journal_event_content::shipyard_transfer_event::ShipyardTransferEvent;
use crate::models::journal_event_content::squadron_startup_event::SquadronStartupEvent;
use crate::models::journal_event_content::srv_destroyed_event::SRVDestroyedEvent;
use crate::models::journal_event_content::start_jump_event::StartJumpEvent;
use crate::models::journal_event_content::statistics_event::StatisticsEvent;
use crate::models::journal_event_content::stored_modules_event::StoredModulesEvent;
use crate::models::journal_event_content::stored_ships_event::StoredShipsEvent;
use crate::models::journal_event_content::suit_loadout_event::SuitLoadoutEvent;
use crate::models::journal_event_content::supercruise_destination_drop_event::SupercruiseDestinationDropEvent;
use crate::models::journal_event_content::supercruise_entry_event::SupercruiseEntryEvent;
use crate::models::journal_event_content::supercruise_exit_event::SupercruiseExitEvent;
use crate::models::journal_event_content::switch_suit_loadout_event::SwitchSuitLoadoutEvent;
use crate::models::journal_event_content::synthasis_event::SynthesisEvent;
use crate::models::journal_event_content::technology_broker_event::TechnologyBrokerEvent;
use crate::models::journal_event_content::touchdown_event::TouchdownEvent;
use crate::models::journal_event_content::under_attack_event::UnderAttackEvent;
use crate::models::journal_event_content::undocked_event::UndockedEvent;
use crate::models::journal_event_content::use_consumable_event::UseConsumableEvent;
use crate::models::journal_event_content::uss_drop_event::USSDropEvent;
use crate::models::journal_event_content::vehicle_switch_event::VehicleSwitchEvent;
use crate::models::journal_event_content::wing_add_event::WingAddEvent;
use crate::models::journal_event_content::wing_invite_event::WingInviteEvent;
use crate::models::journal_event_content::wing_join_event::WingJoinEvent;

#[cfg(not(feature = "strict"))]
use serde_json::Value;

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
mod buy_trade_data_event;
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
pub mod engineer_apply_event;
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
/// Contains structs and enums which are shared across events. Things like commodity and material
/// names, ship types, exobiology data etc. can be found here.
pub mod shared;
pub mod shared_bookmark_to_squadron_event;
pub mod shield_state_event;
pub mod ship_locker_event;
pub mod ship_targeted_event;
pub mod shipyard_buy_event;
pub mod shipyard_event;
pub mod shipyard_new_event;
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
#[derive(Debug, Deserialize, Kinded, Clone, PartialEq)]
#[serde(tag = "event")]
pub enum JournalEventContent {
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
    CommunityGoal(CommunityGoalEventGoal),
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
    CarrierNameChanged(CarrierNameChangedEvent),
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
    DropShipDeploy(DropShipDeployEvent),
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
