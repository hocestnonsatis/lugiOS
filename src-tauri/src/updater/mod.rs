//! Compares installed mini-app versions with GitHub `releases/latest` tags.

use serde::Serialize;
use semver::Version;
use std::collections::HashMap;
use tauri::AppHandle;

use crate::error::LugosError;
use crate::installer;
use crate::registry::{self, types::RegistryEntry};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppUpdateStatus {
    pub app_id: String,
    pub display_name: String,
    pub installed_version: String,
    pub repo_url: Option<String>,
    pub latest_version: Option<String>,
    pub update_available: bool,
    pub check_error: Option<String>,
}

fn normalize_version(s: &str) -> Option<Version> {
    let t = s.trim();
    let t = t.strip_prefix('v').or_else(|| t.strip_prefix('V')).unwrap_or(t);
    Version::parse(t).ok()
}

fn latest_is_newer(latest_tag: &str, installed: &str) -> bool {
    match (normalize_version(latest_tag), normalize_version(installed)) {
        (Some(a), Some(b)) => a > b,
        _ => false,
    }
}

pub async fn check_app_updates(app: &AppHandle) -> Result<Vec<AppUpdateStatus>, LugosError> {
    let entries: Vec<RegistryEntry> = registry::fetch_registry(app).await?;
    let by_id: HashMap<String, String> = entries
        .into_iter()
        .map(|e| (e.id, e.repo))
        .collect();

    let installed = installer::list_installed(app)?;
    let mut out = Vec::with_capacity(installed.len());

    for m in installed {
        let repo_opt = by_id.get(&m.id).cloned();
        let row = if let Some(repo_url) = repo_opt {
            match installer::fetch_latest_release_tag(&repo_url).await {
                Ok(tag) => {
                    let update_available = latest_is_newer(&tag, &m.version);
                    AppUpdateStatus {
                        app_id: m.id.clone(),
                        display_name: m.display_name.clone(),
                        installed_version: m.version.clone(),
                        repo_url: Some(repo_url),
                        latest_version: Some(tag),
                        update_available,
                        check_error: None,
                    }
                }
                Err(e) => AppUpdateStatus {
                    app_id: m.id.clone(),
                    display_name: m.display_name.clone(),
                    installed_version: m.version.clone(),
                    repo_url: Some(repo_url),
                    latest_version: None,
                    update_available: false,
                    check_error: Some(e.to_string()),
                },
            }
        } else {
            AppUpdateStatus {
                app_id: m.id.clone(),
                display_name: m.display_name.clone(),
                installed_version: m.version.clone(),
                repo_url: None,
                latest_version: None,
                update_available: false,
                check_error: Some(
                    "Not listed in the current marketplace registry; cannot compare to GitHub."
                        .into(),
                ),
            }
        };
        out.push(row);
    }
    out.sort_by(|a, b| a.display_name.cmp(&b.display_name));
    Ok(out)
}
