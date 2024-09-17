// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::commands::google::oauth::*;
use crate::commands::messages::*;
use log::info;
use crate::commands::imap::save_imap_config;
use crate::commands::user::credentials_exist;

pub mod commands;
pub mod database;
pub mod structs;

fn main() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let state = create_auth_state();

    tauri::Builder::default()
        .manage(state)
        .setup(|_app| {
            info!("⚠️Checking for database migrations ...");
            database::db_init::run_migration(database::db_init::establish_connection());
            info!("... database migrations complete ✅");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            fetch_messages,
            fetch_by_query,
            authenticate_google,
            fetch_gmail_messages,
            credentials_exist,
            save_imap_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
