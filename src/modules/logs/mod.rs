//! Elite Dangerous create journal log files which contain information about various events that
//! happen in the game. This module provides models and readers for working with these files.
//!
//! * [LogDir](logs::LogDir) represents the directory which includes all the journal files for the player. It can
//!   be used get specific journal log files.
//! * [LogFile](logs::LogFile) is used to create readers with which can then be used to read the contents of the
//!   target file.
//! * The [LogEvent](logs::LogEvent) struct is a single entry in a log file and is what is emitted by the different
//!   kinds of readers.
//! * The [LogEventContent](logs::LogEventContent) is the actual content of the entry and is a large enum containing all
//!   the different types of events that are logged.
//! * The [blocking](logs::blocking) module provides readers which block the current thread and requires you to
//!   manually manage threads, especially when using the 'live' readers.
//! * The [asynchronous](logs::asynchronous) module contains the readers for when you're working in an asynchronous
//!   environment like for example when using tokio.
//!
//! Using these models, you can read the contents of the journal log files. Note however that this
//! is **lossy**, meaning that it is possible to deserialize the contents of a log file, but it is
//! not possible to serialize the resulting entry back to the same log entry. It is possible to
//! serialize the contents using [serde], but this will have a different shape than how it was
//! originally formatted on the log file.

pub use content::log_event_content::afmu_repairs_event;
pub use content::log_event_content::applied_to_squadron_event;
pub use content::log_event_content::approach_body_event;
pub use content::log_event_content::approach_settlement_event;
pub use content::log_event_content::asteroid_cracked_event;
pub use content::log_event_content::backpack_change_event;
pub use content::log_event_content::backpack_event;
pub use content::log_event_content::book_dropship_event;
pub use content::log_event_content::book_taxi_event;
pub use content::log_event_content::bounty_event;
pub use content::log_event_content::buy_ammo_event;
pub use content::log_event_content::buy_drones_event;
pub use content::log_event_content::buy_exploration_data_event;
pub use content::log_event_content::buy_micro_resource_event;
pub use content::log_event_content::buy_suit_event;
pub use content::log_event_content::buy_trade_data_event;
pub use content::log_event_content::buy_weapon_event;
pub use content::log_event_content::cancel_dropship_event;
pub use content::log_event_content::cancel_taxi_event;
pub use content::log_event_content::cap_ship_bond_event;
pub use content::log_event_content::cargo_depot_event;
pub use content::log_event_content::cargo_event;
pub use content::log_event_content::cargo_transfer_event;
pub use content::log_event_content::carrier_bank_transfer_event;
pub use content::log_event_content::carrier_buy_event;
pub use content::log_event_content::carrier_cancel_decommission_event;
pub use content::log_event_content::carrier_crew_services_event;
pub use content::log_event_content::carrier_decommision_event;
pub use content::log_event_content::carrier_deposit_fuel_event;
pub use content::log_event_content::carrier_docking_permission_event;
pub use content::log_event_content::carrier_finance_event;
pub use content::log_event_content::carrier_jump_cancelled_event;
pub use content::log_event_content::carrier_jump_event;
pub use content::log_event_content::carrier_jump_request_event;
pub use content::log_event_content::carrier_module_pack_event;
pub use content::log_event_content::carrier_name_changed_event;
pub use content::log_event_content::carrier_ship_pack_event;
pub use content::log_event_content::carrier_stats_event;
pub use content::log_event_content::carrier_trade_order_event;
pub use content::log_event_content::change_crew_role_event;
pub use content::log_event_content::clear_impound_event;
pub use content::log_event_content::clear_saved_game_event;
pub use content::log_event_content::codex_entry_event;
pub use content::log_event_content::collect_cargo_event;
pub use content::log_event_content::collect_items_event;
pub use content::log_event_content::commander_event;
pub use content::log_event_content::commit_crime_event;
pub use content::log_event_content::community_goal_discard_event;
pub use content::log_event_content::community_goal_event;
pub use content::log_event_content::community_goal_join_event;
pub use content::log_event_content::community_goal_reward_event;
pub use content::log_event_content::continued_event;
pub use content::log_event_content::create_suit_loadout_event;
pub use content::log_event_content::crew_assign_event;
pub use content::log_event_content::crew_fire_event;
pub use content::log_event_content::crew_hire_event;
pub use content::log_event_content::crew_launch_fighter_event;
pub use content::log_event_content::crew_member_joins_event;
pub use content::log_event_content::crew_member_quits_event;
pub use content::log_event_content::crew_member_role_change_event;
pub use content::log_event_content::crime_victim_event;
pub use content::log_event_content::data_scanned_event;
pub use content::log_event_content::datalink_scan_event;
pub use content::log_event_content::datalink_voucher_event;
pub use content::log_event_content::delete_suit_loadout_event;
pub use content::log_event_content::died_event;
pub use content::log_event_content::disbanded_squadron_event;
pub use content::log_event_content::discovery_scan_event;
pub use content::log_event_content::disembark_event;
pub use content::log_event_content::dock_fighter_event;
pub use content::log_event_content::dock_srv_event;
pub use content::log_event_content::docked_event;
pub use content::log_event_content::docking_cancelled_event;
pub use content::log_event_content::docking_denied_event;
pub use content::log_event_content::docking_granted_event;
pub use content::log_event_content::docking_requested_event;
pub use content::log_event_content::docking_timeout_event;
pub use content::log_event_content::drop_items_event;
pub use content::log_event_content::drop_ship_deploy_event;
pub use content::log_event_content::eject_cargo_event;
pub use content::log_event_content::embark_event;
pub use content::log_event_content::end_crew_session_event;
pub use content::log_event_content::engineer_contribution_event;
pub use content::log_event_content::engineer_craft_event;
pub use content::log_event_content::engineer_progress_event;
pub use content::log_event_content::escape_interdiction_event;
pub use content::log_event_content::faction_kill_bond_event;
pub use content::log_event_content::fc_embark_event;
pub use content::log_event_content::fetch_remote_module_event;
pub use content::log_event_content::fighter_destroyed_event;
pub use content::log_event_content::fighter_rebuilt_event;
pub use content::log_event_content::file_header_event;
pub use content::log_event_content::friends_event;
pub use content::log_event_content::fsd_jump_event;
pub use content::log_event_content::fsd_target_event;
pub use content::log_event_content::fss_all_bodies_found_event;
pub use content::log_event_content::fss_body_signals_event;
pub use content::log_event_content::fss_discovery_scan_event;
pub use content::log_event_content::fss_signal_discovered_event;
pub use content::log_event_content::fuel_scoop_event;
pub use content::log_event_content::hull_damage_event;
pub use content::log_event_content::interdicted_event;
pub use content::log_event_content::interdiction_event;
pub use content::log_event_content::invited_to_squadron_event;
pub use content::log_event_content::jet_cone_boost_event;
pub use content::log_event_content::jet_cone_damage_event;
pub use content::log_event_content::join_a_crew_event;
pub use content::log_event_content::joined_squadron_event;
pub use content::log_event_content::kick_crew_member_event;
pub use content::log_event_content::kicked_from_squadron_event;
pub use content::log_event_content::launch_drone_event;
pub use content::log_event_content::launch_fighter_event;
pub use content::log_event_content::launch_srv_event;
pub use content::log_event_content::leave_body_event;
pub use content::log_event_content::left_squadron_event;
pub use content::log_event_content::liftoff_event;
pub use content::log_event_content::load_game_event;
pub use content::log_event_content::loadout_equip_module_event;
pub use content::log_event_content::loadout_event;
pub use content::log_event_content::loadout_remove_module_event;
pub use content::log_event_content::location_event;
pub use content::log_event_content::market_buy_event;
pub use content::log_event_content::market_event;
pub use content::log_event_content::market_sell_event;
pub use content::log_event_content::mass_module_store_event;
pub use content::log_event_content::material_collected_event;
pub use content::log_event_content::material_discarded_event;
pub use content::log_event_content::material_discovered_event;
pub use content::log_event_content::material_trade_event;
pub use content::log_event_content::materials_event;
pub use content::log_event_content::mining_refined;
pub use content::log_event_content::mission_abandoned_event;
pub use content::log_event_content::mission_accepted_event;
pub use content::log_event_content::mission_completed_event;
pub use content::log_event_content::mission_failed_event;
pub use content::log_event_content::mission_redirected_event;
pub use content::log_event_content::missions_event;
pub use content::log_event_content::module_buy_and_store_event;
pub use content::log_event_content::module_buy_event;
pub use content::log_event_content::module_retrieve_event;
pub use content::log_event_content::module_sell_event;
pub use content::log_event_content::module_sell_remote_event;
pub use content::log_event_content::module_store_event;
pub use content::log_event_content::module_swap_event;
pub use content::log_event_content::multi_sell_exploration_data_event;
pub use content::log_event_content::music_event;
pub use content::log_event_content::nav_beacon_scan_event;
pub use content::log_event_content::new_commander_event;
pub use content::log_event_content::npc_crew_rank_event;
pub use content::log_event_content::npc_crew_wage_paid_event;
pub use content::log_event_content::outfitting_event;
pub use content::log_event_content::passengers_event;
pub use content::log_event_content::pay_bounties_event;
pub use content::log_event_content::pay_fines_event;
pub use content::log_event_content::powerplay_collect_event;
pub use content::log_event_content::powerplay_defect_event;
pub use content::log_event_content::powerplay_deliver_event;
pub use content::log_event_content::powerplay_event;
pub use content::log_event_content::powerplay_fast_track_event;
pub use content::log_event_content::powerplay_join_event;
pub use content::log_event_content::powerplay_leave_event;
pub use content::log_event_content::powerplay_salary_event;
pub use content::log_event_content::powerplay_vote_event;
pub use content::log_event_content::progress_event;
pub use content::log_event_content::promotion_event;
pub use content::log_event_content::prospected_asteroid_event;
pub use content::log_event_content::pvp_kill_event;
pub use content::log_event_content::quit_a_crew_event;
pub use content::log_event_content::rank_event;
pub use content::log_event_content::reboot_repair_event;
pub use content::log_event_content::receive_text_event;
pub use content::log_event_content::redeem_voucher_event;
pub use content::log_event_content::refuel_all_event;
pub use content::log_event_content::refuel_partial_event;
pub use content::log_event_content::rename_suit_loadout_event;
pub use content::log_event_content::repair_all_event;
pub use content::log_event_content::repair_drone_event;
pub use content::log_event_content::repair_event;
pub use content::log_event_content::reputation_event;
pub use content::log_event_content::reservoir_replenished_event;
pub use content::log_event_content::restock_vehicle_event;
pub use content::log_event_content::resurrect_event;
pub use content::log_event_content::saa_scan_complete_event;
pub use content::log_event_content::saa_signals_found_event;
pub use content::log_event_content::scan_bary_centre_event;
pub use content::log_event_content::scan_event;
pub use content::log_event_content::scan_organic_event;
pub use content::log_event_content::scanned_event;
pub use content::log_event_content::scientific_research_event;
pub use content::log_event_content::screenshot_event;
pub use content::log_event_content::search_and_rescue_event;
pub use content::log_event_content::sell_drones_event;
pub use content::log_event_content::sell_exploration_date_event;
pub use content::log_event_content::sell_micro_resources_event;
pub use content::log_event_content::sell_organic_data;
pub use content::log_event_content::sell_ship_on_rebuy_event;
pub use content::log_event_content::sell_suit_event;
pub use content::log_event_content::sell_weapon_event;
pub use content::log_event_content::send_text_event;
pub use content::log_event_content::set_user_ship_name_event;
pub use content::log_event_content::shared_bookmark_to_squadron_event;
pub use content::log_event_content::shield_state_event;
pub use content::log_event_content::ship_locker_event;
pub use content::log_event_content::ship_redeemed;
pub use content::log_event_content::ship_targeted_event;
pub use content::log_event_content::shipyard_buy_event;
pub use content::log_event_content::shipyard_event;
pub use content::log_event_content::shipyard_new_event;
pub use content::log_event_content::shipyard_redeem;
pub use content::log_event_content::shipyard_sell_event;
pub use content::log_event_content::shipyard_swap_event;
pub use content::log_event_content::shipyard_transfer_event;
pub use content::log_event_content::squadron_created_event;
pub use content::log_event_content::squadron_demotion_event;
pub use content::log_event_content::squadron_promotion_event;
pub use content::log_event_content::squadron_startup_event;
pub use content::log_event_content::srv_destroyed_event;
pub use content::log_event_content::start_jump_event;
pub use content::log_event_content::statistics_event;
pub use content::log_event_content::stored_modules_event;
pub use content::log_event_content::stored_ships_event;
pub use content::log_event_content::suit_loadout_event;
pub use content::log_event_content::supercruise_destination_drop_event;
pub use content::log_event_content::supercruise_entry_event;
pub use content::log_event_content::supercruise_exit_event;
pub use content::log_event_content::switch_suit_loadout_event;
pub use content::log_event_content::synthasis_event;
pub use content::log_event_content::technology_broker_event;
pub use content::log_event_content::touchdown_event;
pub use content::log_event_content::trade_micro_resources_event;
pub use content::log_event_content::transfer_micro_resources_event;
pub use content::log_event_content::under_attack_event;
pub use content::log_event_content::undocked_event;
pub use content::log_event_content::upgrade_suit_event;
pub use content::log_event_content::upgrade_weapon_event;
pub use content::log_event_content::use_consumable_event;
pub use content::log_event_content::uss_drop_event;
pub use content::log_event_content::vehicle_switch_event;
pub use content::log_event_content::wing_add_event;
pub use content::log_event_content::wing_invite_event;
pub use content::log_event_content::wing_join_event;
pub use content::log_event_content::won_a_trophy_for_squadron_event;
pub use content::LogEvent;
pub use content::LogEventContent;
pub use content::LogEventContentKind;
pub use log_dir::LogDir;
pub use log_dir::LogDirError;
pub use log_file::LogFile;
pub use log_file::LogFileError;

mod log_dir;
mod log_file;

/// Contains models for all the different events that are written in the log files.
mod content;

/// Contains readers for when working in a synchronous environment like a manually spawned thread.
#[deprecated]
pub mod blocking;

/// Contains readers for when working in an asynchronous environment like Tokio.
#[cfg(all(feature = "asynchronous", feature = "tokio"))]
#[cfg_attr(docsrs, doc(cfg(feature = "asynchronous")))]
#[deprecated]
pub mod asynchronous;
