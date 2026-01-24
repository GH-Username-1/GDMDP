// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod state;

use commands::*;
use state::AppState;

fn main() {
    tauri::Builder::default()
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            create_vault,
            open_vault,
            lock_vault,
            is_locked,
            list_entries,
            add_entry,
            update_entry,
            delete_entry,
            search_entries,
            generate_password_cmd,
            create_backup,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
