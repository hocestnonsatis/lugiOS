//! Generates per-app Tauri v2 capability JSON on disk.

use std::path::PathBuf;
use tauri::{AppHandle, Manager};

use crate::error::LugosError;
use crate::permissions::types::{GrantRecord, Permission};

fn capabilities_dir(app: &AppHandle) -> Result<PathBuf, LugosError> {
    Ok(app.path().app_data_dir()?.join("capabilities"))
}

fn map_permission_to_tauri(grant: &Permission) -> Vec<&'static str> {
    match grant {
        Permission::Storage => vec![
            "allow-storage-get",
            "allow-storage-set",
            "allow-storage-delete",
        ],
        Permission::Notifications => vec!["notification:default"],
        Permission::ClipboardRead => vec!["allow-lugos-clipboard-read"],
        Permission::ClipboardWrite => vec!["allow-lugos-clipboard-write"],
        Permission::AudioPlay => vec!["allow-lugos-audio-read-asset"],
        Permission::FilesystemRead => vec![
            "dialog:allow-open",
            "fs:allow-read-file",
        ],
        Permission::FilesystemWrite => vec![
            "dialog:allow-save",
            "fs:allow-write-file",
            "fs:allow-mkdir",
        ],
        Permission::Network | Permission::NetworkDomain(_) => vec!["allow-lugos-fetch"],
    }
}

/// Writes a capability JSON file consumable by [`tauri::Manager::add_capability`].
pub fn generate_capability_file(app: &AppHandle, grant: &GrantRecord) -> Result<(), LugosError> {
    let dir = capabilities_dir(app)?;
    std::fs::create_dir_all(&dir)?;
    let path = dir.join(format!("{}.json", grant.app_id));
    let window_label = format!("app:{}", grant.app_id);

    let mut permissions: Vec<String> = vec!["core:default".to_string()];
    for p in &grant.granted {
        for t in map_permission_to_tauri(p) {
            permissions.push(t.to_string());
        }
    }

    let json = serde_json::json!({
        "identifier": format!("runtime-{}", grant.app_id),
        "description": format!("LugiOS runtime capability for {}", grant.app_id),
        "windows": [window_label],
        "permissions": permissions,
    });

    std::fs::write(path, serde_json::to_vec_pretty(&json)?)?;
    Ok(())
}
