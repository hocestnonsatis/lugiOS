//! LugiOS host: Tauri entry, modules, and IPC registration.

mod commands;
mod error;
mod github;
mod installer;
mod permissions;
mod registry;
mod runtime;
mod storage;
mod util;

pub use error::LugosError;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::get_registry,
            commands::refresh_registry,
            commands::install_app,
            commands::uninstall_app,
            commands::list_installed,
            commands::get_grant,
            commands::launch_app,
            commands::close_app,
            commands::list_running_apps,
            commands::storage_get,
            commands::storage_set,
            commands::storage_delete,
            commands::lugos_fs_pick_read,
            commands::lugos_fs_save_dialog,
            commands::lugos_clipboard_read,
            commands::lugos_clipboard_write,
            commands::lugos_notification_send,
            commands::lugos_audio_read_asset,
            commands::lugos_fetch,
            commands::preview_app_manifest,
            commands::get_github_repo_stats,
        ])
        .run(tauri::generate_context!())?;
    Ok(())
}
