// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::commands::google::oauth::*;
use crate::commands::messages::*;
use log::info;

pub mod commands;
pub mod structs;
pub mod database;

fn main() {
    std::env::set_var("RUST_LOG", "info");
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
        .invoke_handler(tauri::generate_handler![fetch_messages, fetch_by_query, authenticate_google])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
