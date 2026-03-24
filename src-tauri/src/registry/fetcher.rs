//! Fetches and caches the LugiOS app registry from GitHub.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

use crate::error::LugosError;
use crate::host_settings;
use crate::registry::types::RegistryEntry;

/// Registry URL resolution: see `host_settings` (`LUGIOS_REGISTRY_URL`, saved setting, or default).
const CACHE_FILE: &str = "registry_cache.json";
const CACHE_TTL_SECS: u64 = 3600;

const EMBEDDED_REGISTRY_JSON: &str = include_str!("../../../registry/registry.json");


fn parse_embedded_registry() -> Result<Vec<RegistryEntry>, LugosError> {
    serde_json::from_str(EMBEDDED_REGISTRY_JSON).map_err(Into::into)
}

async fn fetch_registry_remote(
    client: &reqwest::Client,
    url: &str,
) -> Result<Vec<RegistryEntry>, LugosError> {
    let body = client.get(url).send().await?.error_for_status()?;
    Ok(body.json().await?)
}

#[derive(Debug, Serialize, Deserialize)]
struct RegistryCacheFile {
    fetched_at_unix: i64,
    entries: Vec<RegistryEntry>,
}

fn cache_path(app: &AppHandle) -> Result<PathBuf, LugosError> {
    Ok(app.path().app_data_dir()?.join(CACHE_FILE))
}

fn http_client() -> Result<reqwest::Client, LugosError> {
    reqwest::Client::builder()
        .user_agent("lugios-host/0.1 (registry; open source)")
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(Into::into)
}

pub async fn fetch_registry(app: &AppHandle) -> Result<Vec<RegistryEntry>, LugosError> {
    let path = cache_path(app)?;
    if let Ok(bytes) = std::fs::read(&path) {
        if let Ok(cache) = serde_json::from_slice::<RegistryCacheFile>(&bytes) {
            let now = chrono::Utc::now().timestamp();
            if now - cache.fetched_at_unix < CACHE_TTL_SECS as i64 {
                return Ok(cache.entries);
            }
        }
    }

    let client = http_client()?;
    let url = host_settings::resolved_registry_url(app)?;
    match fetch_registry_remote(&client, &url).await {
        Ok(entries) => {
            let cache = RegistryCacheFile {
                fetched_at_unix: chrono::Utc::now().timestamp(),
                entries: entries.clone(),
            };
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(&path, serde_json::to_vec_pretty(&cache)?)?;
            Ok(entries)
        }
        Err(_) => parse_embedded_registry(),
    }
}

pub async fn refresh_registry(app: &AppHandle) -> Result<Vec<RegistryEntry>, LugosError> {
    let path = cache_path(app)?;
    let client = http_client()?;
    let url = host_settings::resolved_registry_url(app)?;
    match fetch_registry_remote(&client, &url).await {
        Ok(entries) => {
            let cache = RegistryCacheFile {
                fetched_at_unix: chrono::Utc::now().timestamp(),
                entries: entries.clone(),
            };
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(&path, serde_json::to_vec_pretty(&cache)?)?;
            Ok(entries)
        }
        Err(_) => parse_embedded_registry(),
    }
}
