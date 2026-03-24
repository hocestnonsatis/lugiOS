//! Tauri IPC commands called from the host shell or from mini-app webviews.

use arboard::Clipboard;
use reqwest::Url;
use serde::Serialize;
use tauri::{AppHandle, Manager, WebviewWindow};
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_notification::NotificationExt;

use crate::error::LugosError;
use crate::github;
use crate::installer;
use crate::permissions::{self, GrantRecord, Permission};
use crate::registry::{self, AppManifest, RegistryEntry};
use crate::runtime;
use crate::storage;
use crate::util::sanitize_app_id;

fn require_app_window(window: &WebviewWindow, app_id: &str) -> Result<(), LugosError> {
    let id = sanitize_app_id(app_id)?;
    let expected = format!("app:{id}");
    if window.label().to_string() != expected {
        return Err(LugosError::PermissionDenied(
            "this IPC endpoint is only available from the mini-app window".into(),
        ));
    }
    Ok(())
}

fn grant_for_app(app: &AppHandle, app_id: &str) -> Result<GrantRecord, LugosError> {
    permissions::load_grant(app, app_id)?
        .ok_or_else(|| LugosError::PermissionDenied("no grant record for this app".into()))
}

fn grant_has(grant: &GrantRecord, p: &Permission) -> bool {
    match p {
        Permission::NetworkDomain(host) => grant.granted.iter().any(|g| match g {
            Permission::Network => true,
            Permission::NetworkDomain(h) => h == host,
            _ => false,
        }),
        Permission::Network => grant.granted.iter().any(|g| matches!(g, Permission::Network)),
        _ => grant.granted.contains(p),
    }
}

fn url_allowed_by_grant(url: &Url, grant: &GrantRecord) -> bool {
    if !matches!(url.scheme(), "http" | "https") {
        return false;
    }
    if grant.granted.iter().any(|g| matches!(g, Permission::Network)) {
        return true;
    }
    let Some(host) = url.host_str() else {
        return false;
    };
    grant.granted.iter().any(|g| {
        if let Permission::NetworkDomain(h) = g {
            h == host || host == format!("www.{h}")
        } else {
            false
        }
    })
}

#[tauri::command]
pub async fn get_registry(app: AppHandle) -> Result<Vec<RegistryEntry>, LugosError> {
    registry::fetch_registry(&app).await
}

#[tauri::command]
pub async fn refresh_registry(app: AppHandle) -> Result<Vec<RegistryEntry>, LugosError> {
    registry::refresh_registry(&app).await
}

#[tauri::command]
pub async fn install_app(
    app: AppHandle,
    app_id: String,
    repo_url: String,
    grants: Vec<String>,
) -> Result<(), LugosError> {
    installer::install_app(&app, &app_id, &repo_url, grants).await
}

#[tauri::command]
pub fn uninstall_app(app: AppHandle, app_id: String) -> Result<(), LugosError> {
    installer::uninstall_app(&app, &app_id)
}

#[tauri::command]
pub fn list_installed(app: AppHandle) -> Result<Vec<AppManifest>, LugosError> {
    installer::list_installed(&app)
}

#[tauri::command]
pub fn get_grant(app: AppHandle, app_id: String) -> Result<Option<GrantRecord>, LugosError> {
    permissions::load_grant(&app, &app_id)
}

#[tauri::command]
pub fn launch_app(app: AppHandle, app_id: String) -> Result<(), LugosError> {
    runtime::launch_app(&app, &app_id)
}

#[tauri::command]
pub fn close_app(app: AppHandle, app_id: String) -> Result<(), LugosError> {
    runtime::close_app(&app, &app_id)
}

#[tauri::command]
pub fn list_running_apps(app: AppHandle) -> Result<Vec<String>, LugosError> {
    Ok(runtime::list_running(&app))
}

#[tauri::command]
pub fn storage_get(
    app: AppHandle,
    window: WebviewWindow,
    app_id: String,
    key: String,
) -> Result<Option<String>, LugosError> {
    require_app_window(&window, &app_id)?;
    let g = grant_for_app(&app, &sanitize_app_id(&app_id)?)?;
    if !grant_has(&g, &Permission::Storage) {
        return Err(LugosError::PermissionDenied(
            "storage permission not granted".into(),
        ));
    }
    let id = sanitize_app_id(&app_id)?;
    storage::storage_get_sync(&app, &id, &key)
}

#[tauri::command]
pub fn storage_set(
    app: AppHandle,
    window: WebviewWindow,
    app_id: String,
    key: String,
    value: String,
) -> Result<(), LugosError> {
    require_app_window(&window, &app_id)?;
    let g = grant_for_app(&app, &sanitize_app_id(&app_id)?)?;
    if !grant_has(&g, &Permission::Storage) {
        return Err(LugosError::PermissionDenied(
            "storage permission not granted".into(),
        ));
    }
    let id = sanitize_app_id(&app_id)?;
    storage::storage_set_sync(&app, &id, &key, &value)
}

#[tauri::command]
pub fn storage_delete(
    app: AppHandle,
    window: WebviewWindow,
    app_id: String,
    key: String,
) -> Result<(), LugosError> {
    require_app_window(&window, &app_id)?;
    let g = grant_for_app(&app, &sanitize_app_id(&app_id)?)?;
    if !grant_has(&g, &Permission::Storage) {
        return Err(LugosError::PermissionDenied(
            "storage permission not granted".into(),
        ));
    }
    let id = sanitize_app_id(&app_id)?;
    storage::storage_delete_sync(&app, &id, &key)
}

#[derive(Serialize)]
pub struct FsPickResult {
    pub name: String,
    pub content: Vec<u8>,
}

#[tauri::command]
pub fn lugos_fs_pick_read(
    app: AppHandle,
    window: WebviewWindow,
    app_id: String,
) -> Result<Option<FsPickResult>, LugosError> {
    require_app_window(&window, &app_id)?;
    let g = grant_for_app(&app, &sanitize_app_id(&app_id)?)?;
    if !grant_has(&g, &Permission::FilesystemRead) {
        return Err(LugosError::PermissionDenied(
            "filesystem:read not granted".into(),
        ));
    }
    let picked = app.dialog().file().blocking_pick_file();
    let Some(fp) = picked else {
        return Ok(None);
    };
    let path = fp.into_path().map_err(|_| LugosError::Msg("could not resolve file path".into()))?;
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("file")
        .to_string();
    let content = std::fs::read(&path)?;
    Ok(Some(FsPickResult { name, content }))
}

#[tauri::command]
pub fn lugos_fs_save_dialog(
    app: AppHandle,
    window: WebviewWindow,
    app_id: String,
    default_name: Option<String>,
    content: Vec<u8>,
) -> Result<(), LugosError> {
    require_app_window(&window, &app_id)?;
    let g = grant_for_app(&app, &sanitize_app_id(&app_id)?)?;
    if !grant_has(&g, &Permission::FilesystemWrite) {
        return Err(LugosError::PermissionDenied(
            "filesystem:write not granted".into(),
        ));
    }
    let mut d = app.dialog().file();
    if let Some(n) = default_name {
        d = d.set_file_name(n);
    }
    let picked = d.blocking_save_file();
    let Some(fp) = picked else {
        return Ok(());
    };
    let path = fp.into_path().map_err(|_| LugosError::Msg("could not resolve file path".into()))?;
    std::fs::write(path, content)?;
    Ok(())
}

#[tauri::command]
pub fn lugos_clipboard_read(
    app: AppHandle,
    window: WebviewWindow,
    app_id: String,
) -> Result<String, LugosError> {
    require_app_window(&window, &app_id)?;
    let g = grant_for_app(&app, &sanitize_app_id(&app_id)?)?;
    if !grant_has(&g, &Permission::ClipboardRead) {
        return Err(LugosError::PermissionDenied(
            "clipboard:read not granted".into(),
        ));
    }
    let _ = app;
    let mut cb = Clipboard::new().map_err(|e| LugosError::Msg(e.to_string()))?;
    cb.get_text()
        .map_err(|e| LugosError::Msg(e.to_string()))
}

#[tauri::command]
pub fn lugos_clipboard_write(
    app: AppHandle,
    window: WebviewWindow,
    app_id: String,
    text: String,
) -> Result<(), LugosError> {
    require_app_window(&window, &app_id)?;
    let g = grant_for_app(&app, &sanitize_app_id(&app_id)?)?;
    if !grant_has(&g, &Permission::ClipboardWrite) {
        return Err(LugosError::PermissionDenied(
            "clipboard:write not granted".into(),
        ));
    }
    let _ = app;
    let mut cb = Clipboard::new().map_err(|e| LugosError::Msg(e.to_string()))?;
    cb.set_text(text)
        .map_err(|e| LugosError::Msg(e.to_string()))
}

#[tauri::command]
pub async fn lugos_notification_send(
    app: AppHandle,
    window: WebviewWindow,
    app_id: String,
    title: String,
    body: Option<String>,
) -> Result<(), LugosError> {
    require_app_window(&window, &app_id)?;
    let g = grant_for_app(&app, &sanitize_app_id(&app_id)?)?;
    if !grant_has(&g, &Permission::Notifications) {
        return Err(LugosError::PermissionDenied(
            "notifications permission not granted".into(),
        ));
    }
    let mut b = app.notification().builder().title(title);
    if let Some(body) = body {
        b = b.body(body);
    }
    b.show().map_err(|e| LugosError::Msg(e.to_string()))?;
    Ok(())
}

#[tauri::command]
pub fn lugos_audio_read_asset(
    app: AppHandle,
    window: WebviewWindow,
    app_id: String,
    asset_path: String,
) -> Result<Vec<u8>, LugosError> {
    require_app_window(&window, &app_id)?;
    let g = grant_for_app(&app, &sanitize_app_id(&app_id)?)?;
    if !grant_has(&g, &Permission::AudioPlay) {
        return Err(LugosError::PermissionDenied(
            "audio:play not granted".into(),
        ));
    }
    let id = sanitize_app_id(&app_id)?;
    let base = app.path().app_data_dir()?.join("apps").join(&id);
    let rel = std::path::Path::new(&asset_path);
    let full = base.join(rel);
    let full = full.canonicalize().map_err(|_| {
        LugosError::PermissionDenied("invalid asset path".into())
    })?;
    let base_canon = base.canonicalize().map_err(|_| LugosError::AppNotFound(id.clone()))?;
    if !full.starts_with(&base_canon) {
        return Err(LugosError::PermissionDenied(
            "asset path escapes app bundle".into(),
        ));
    }
    Ok(std::fs::read(&full)?)
}

#[tauri::command]
pub async fn lugos_fetch(
    app: AppHandle,
    window: WebviewWindow,
    app_id: String,
    url: String,
    method: Option<String>,
    body: Option<String>,
) -> Result<Vec<u8>, LugosError> {
    require_app_window(&window, &app_id)?;
    let g = grant_for_app(&app, &sanitize_app_id(&app_id)?)?;
    let parsed = Url::parse(&url).map_err(|e| LugosError::Msg(e.to_string()))?;
    if !url_allowed_by_grant(&parsed, &g) {
        return Err(LugosError::PermissionDenied(
            "network permission does not allow this URL".into(),
        ));
    }
    let client = reqwest::Client::builder()
        .user_agent("lugos-miniapp/0.1")
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(LugosError::Http)?;
    let method = method
        .unwrap_or_else(|| "GET".into())
        .to_uppercase();
    let mut req = match method.as_str() {
        "GET" => client.get(parsed.clone()),
        "POST" => client.post(parsed.clone()),
        "PUT" => client.put(parsed.clone()),
        "DELETE" => client.delete(parsed.clone()),
        "PATCH" => client.patch(parsed.clone()),
        _ => {
            return Err(LugosError::Msg(format!(
                "unsupported HTTP method: {method}"
            )))
        }
    };
    if let Some(b) = body {
        if method != "GET" {
            req = req.body(b);
        }
    }
    let _ = app;
    let resp = req.send().await?.error_for_status()?;
    Ok(resp.bytes().await?.to_vec())
}

#[tauri::command]
pub async fn preview_app_manifest(app: AppHandle, repo_url: String) -> Result<AppManifest, LugosError> {
    installer::fetch_app_manifest(&repo_url, &app).await
}

#[tauri::command]
pub async fn get_github_repo_stats(repo_url: String) -> Result<github::GitHubRepoStats, LugosError> {
    github::fetch_repo_stats(&repo_url).await
}
