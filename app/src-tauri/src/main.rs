#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod keyring_store;
mod state;

use state::VaultState;

fn main() {
    tauri::Builder::default()
        .manage(VaultState::default())
        .setup(|app| {
            // 自动更新仅支持桌面端；移动端走应用商店分发，不注册该插件。
            #[cfg(desktop)]
            {
                app.handle()
                    .plugin(tauri_plugin_updater::Builder::new().build())?;
                app.handle().plugin(tauri_plugin_process::init())?;
            }
            Ok(())
        })
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
