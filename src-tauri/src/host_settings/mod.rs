//! Persists host-only preferences (custom marketplace registry URL) under app data.

use std::path::PathBuf;

use reqwest::Url;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

use crate::error::LugosError;

const SETTINGS_FILE: &str = "host_settings.json";

/// Default registry URL (same as `registry::fetcher` fallback).
pub const DEFAULT_REGISTRY_URL: &str =
    "https://raw.githubusercontent.com/hocestnonsatis/lugiOS/main/registry/registry.json";

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct HostSettings {
    /// When set, used unless `LUGIOS_REGISTRY_URL` is defined.
    pub registry_url: Option<String>,
}

fn settings_path(app: &AppHandle) -> Result<PathBuf, LugosError> {
    Ok(app.path().app_data_dir()?.join(SETTINGS_FILE))
}

pub fn load(app: &AppHandle) -> Result<HostSettings, LugosError> {
    let path = settings_path(app)?;
    if !path.exists() {
        return Ok(HostSettings::default());
    }
    let bytes = std::fs::read(&path)?;
    let s: HostSettings = serde_json::from_slice(&bytes)?;
    Ok(s)
}

pub fn save(app: &AppHandle, settings: &HostSettings) -> Result<(), LugosError> {
    let path = settings_path(app)?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&path, serde_json::to_vec_pretty(settings)?)?;
    Ok(())
}

fn validate_registry_url(raw: &str) -> Result<(), LugosError> {
    let t = raw.trim();
    let url = Url::parse(t).map_err(|e| LugosError::Msg(format!("invalid registry URL: {e}")))?;
    match url.scheme() {
        "https" => Ok(()),
        "http" => {
            let host = url.host_str().unwrap_or("");
            if host == "localhost" || host == "127.0.0.1" || host == "[::1]" {
                Ok(())
            } else {
                Err(LugosError::Msg(
                    "only https URLs are allowed (http is limited to localhost for development)"
                        .into(),
                ))
            }
        }
        _ => Err(LugosError::Msg(
            "registry URL must use http (localhost only) or https".into(),
        )),
    }
}

/// Clears saved URL when `url` is `None` or empty after trim.
pub fn apply_registry_url_override(
    app: &AppHandle,
    url: Option<String>,
) -> Result<HostSettings, LugosError> {
    let mut settings = load(app)?;
    match url {
        None => settings.registry_url = None,
        Some(s) => {
            let t = s.trim().to_string();
            if t.is_empty() {
                settings.registry_url = None;
            } else {
                validate_registry_url(&t)?;
                settings.registry_url = Some(t);
            }
        }
    }
    save(app, &settings)?;
    Ok(settings)
}

/// Effective URL: `LUGIOS_REGISTRY_URL` env (non-empty) wins, then saved setting, then default.
pub fn resolved_registry_url(app: &AppHandle) -> Result<String, LugosError> {
    if let Ok(v) = std::env::var("LUGIOS_REGISTRY_URL") {
        let t = v.trim();
        if !t.is_empty() {
            return Ok(t.to_string());
        }
    }
    let settings = load(app)?;
    if let Some(ref u) = settings.registry_url {
        let t = u.trim();
        if !t.is_empty() {
            return Ok(t.to_string());
        }
    }
    Ok(DEFAULT_REGISTRY_URL.to_string())
}

pub fn env_registry_override_active() -> bool {
    std::env::var("LUGIOS_REGISTRY_URL")
        .map(|v| !v.trim().is_empty())
        .unwrap_or(false)
}
