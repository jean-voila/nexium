// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod commands;
mod config;
mod contacts;
mod core;
mod invoice;
mod nexium_api;
mod types;
use commands::*;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .invoke_handler(tauri::generate_handler![
            get_constants::get_constants,
            check_config_values::check_config_values,
            get_gitlab_oauth_token::get_gitlab_oauth_token,
            load_config_from_file::load_config_from_file,
            save_config_to_file::save_config_to_file,
            load_config::load_config,
            save_config::save_config,
            keypair_generation::keypair_generation,
            send_gpg_key::send_gpg_key,
            get_login::get_login,
            save_facture_to_file::save_facture_to_file,
            check_invoice_values::check_invoice_values,
            get_names_from_login::get_names_from_login,
            load_invoice_from_file::load_invoice_from_file,
            calculate_transaction_fee::calculate_transaction_fee,
            get_balance::get_balance,
            send_transaction::send_transaction,
            get_transactions::get_transactions,
            get_server_infos::get_server_infos,
            write_key_to_file::write_key_to_file,
            read_key_from_file::read_key_from_file,
            check_send_transaction::check_send_transaction,
            search_first_users::search_first_users,
            contact_get::contact_get,
            contact_add::contact_add,
            contact_update::contact_update,
            contact_remove::contact_remove,
            contact_search::contact_search,
            contacts::get_recent_contacts,
            contact_mark_used::contact_mark_used,
            get_user_stats::get_user_stats,
            get_peers::get_peers,
            check_peer_status::check_peer_status,
            try_connect_to_server::try_connect_to_server,
            find_working_server::find_working_server,
        ])
        .plugin(tauri_plugin_fs::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
