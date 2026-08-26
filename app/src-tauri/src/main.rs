#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod keyring_store;
mod state;

use state::VaultState;

fn main() {
    tauri::Builder::default()
        .manage(VaultState::default())
        .invoke_handler(tauri::generate_handler![
            commands::vault_status,
            commands::create_vault,
            commands::unlock_vault,
            commands::unlock_remembered,
            commands::lock_vault,
            commands::list_entries,
            commands::add_entry,
            commands::update_entry,
            commands::delete_entry,
            commands::change_password,
            commands::forget_vault,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
