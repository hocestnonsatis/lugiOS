//! Creates and tracks isolated `WebviewWindow` instances for mini-apps.

use std::path::PathBuf;
use tauri::webview::WebviewWindowBuilder;
use tauri::{AppHandle, Manager, Url, WebviewUrl};

use crate::error::LugosError;
use crate::permissions::{self};
use crate::registry::types::AppManifest;
use crate::runtime::bridge;
use crate::util::sanitize_app_id;

fn app_install_dir(app: &AppHandle, app_id: &str) -> Result<PathBuf, LugosError> {
    Ok(app.path().app_data_dir()?.join("apps").join(app_id))
}

fn load_installed_manifest(app: &AppHandle, app_id: &str) -> Result<AppManifest, LugosError> {
    let p = app_install_dir(app, app_id)?.join("app.manifest.json");
    if !p.exists() {
        return Err(LugosError::AppNotFound(app_id.to_string()));
    }
    let m: AppManifest = serde_json::from_reader(std::fs::File::open(p)?)?;
    Ok(m)
}

/// Builds the bootstrap prelude (grants + app id) plus the AppBridge bundle for `initialization_script`.
fn combined_init_script(app: &AppHandle, grant: &permissions::GrantRecord) -> Result<String, LugosError> {
    let safe_id = sanitize_app_id(&grant.app_id)?;
    let tokens: Vec<String> = grant.granted.iter().map(|p| p.as_token()).collect();
    let grants_json = serde_json::to_string(&tokens)?;
    let sdk = bridge::read_bundle(app)?;
    Ok(format!(
        r#"Object.defineProperty(globalThis, '__LUGOS_GRANTS__', {{ value: Object.freeze({grants_json}), writable: false, configurable: false }});
Object.defineProperty(globalThis, '__LUGOS_APP_ID__', {{ value: {sid:?}, writable: false, configurable: false }});
{sdk}"#,
        grants_json = grants_json,
        sid = safe_id,
        sdk = sdk
    ))
}

pub fn launch_app(app: &AppHandle, app_id: &str) -> Result<(), LugosError> {
    let id = sanitize_app_id(app_id)?;
    let grant = permissions::load_grant(app, &id)?.ok_or_else(|| LugosError::AppNotFound(id.clone()))?;
    let manifest = load_installed_manifest(app, &id)?;

    let cap_path = app
        .path()
        .app_data_dir()?
        .join("capabilities")
        .join(format!("{id}.json"));
    let cap_json = std::fs::read_to_string(&cap_path)?;
    let _ = app.add_capability(cap_json);

    let label = format!("app:{id}");
    if app.get_webview_window(&label).is_some() {
        return Ok(());
    }

    let install_dir = app_install_dir(app, &id)?;
    let entry = install_dir.join(&manifest.entry_point);
    if !entry.exists() {
        return Err(LugosError::Msg(format!(
            "entry point not found: {}",
            entry.display()
        )));
    }
    let url = Url::from_file_path(&entry).map_err(|_| {
        LugosError::Msg(format!("invalid file URL for {}", entry.display()))
    })?;

    let w = manifest.window;
    let init = combined_init_script(app, &grant)?;
    WebviewWindowBuilder::new(app, &label, WebviewUrl::External(url))
        .title(manifest.display_name.clone())
        .inner_size(w.width as f64, w.height as f64)
        .resizable(w.resizable)
        .always_on_top(w.always_on_top)
        .initialization_script(init)
        .build()?;
    Ok(())
}

pub fn close_app(app: &AppHandle, app_id: &str) -> Result<(), LugosError> {
    let id = sanitize_app_id(app_id)?;
    let label = format!("app:{id}");
    if let Some(w) = app.get_webview_window(&label) {
        w.close()?;
    }
    Ok(())
}

pub fn list_running(app: &AppHandle) -> Vec<String> {
    app.webview_windows()
        .keys()
        .filter_map(|l| l.strip_prefix("app:").map(String::from))
        .collect()
}
