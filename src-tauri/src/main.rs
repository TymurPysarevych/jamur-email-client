// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::commands::messages::*;

mod commands;
mod structs;

fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![fetch_messages, fetch_by_query])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
