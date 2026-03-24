//! Install, uninstall, and enumerate locally installed LugiOS apps.

mod downloader;
mod extractor;

use std::fs::File;

use tauri::{AppHandle, Manager};

pub use downloader::{fetch_app_manifest, fetch_latest_release_tag, parse_github_repo};
use downloader::download_latest_archive;
use extractor::extract_app;
use crate::error::LugosError;
use crate::permissions::{self, capability, GrantRecord, Permission};
use crate::registry::types::AppManifest;

fn apps_root(app: &AppHandle) -> Result<std::path::PathBuf, LugosError> {
    Ok(app.path().app_data_dir()?.join("apps"))
}

pub async fn install_app(
    app: &AppHandle,
    app_id: &str,
    repo_url: &str,
    grants: Vec<String>,
) -> Result<(), LugosError> {
    install_app_inner(app, app_id, repo_url, grants, false).await
}

/// Reinstalls from the repo’s latest release, dropping saved grants that the new manifest no longer lists.
pub async fn upgrade_installed_app(
    app: &AppHandle,
    app_id: &str,
    repo_url: &str,
    grants: Vec<String>,
) -> Result<(), LugosError> {
    install_app_inner(app, app_id, repo_url, grants, true).await
}

async fn install_app_inner(
    app: &AppHandle,
    app_id: &str,
    repo_url: &str,
    grants: Vec<String>,
    prune_stale_grants: bool,
) -> Result<(), LugosError> {
    let archive = download_latest_archive(repo_url, app).await?;
    let dest = apps_root(app)?.join(app_id);
    let dest_clone = dest.clone();
    let arch = archive.clone();
    tokio::task::spawn_blocking(move || extract_app(&arch, &dest_clone))
        .await
        .map_err(|e| LugosError::Msg(format!("extract join error: {e}")))??;
    std::fs::remove_file(&archive)?;

    let manifest_path = dest.join("app.manifest.json");
    let manifest: AppManifest = serde_json::from_reader(File::open(&manifest_path)?)?;
    if manifest.id != app_id {
        let _ = std::fs::remove_dir_all(&dest);
        return Err(LugosError::Msg(format!(
            "manifest id '{}' does not match requested app id '{}'",
            manifest.id, app_id
        )));
    }

    let grants: Vec<String> = if prune_stale_grants {
        grants
            .into_iter()
            .filter(|g| manifest.permissions.iter().any(|p| p == g))
            .collect()
    } else {
        for g in &grants {
            if !manifest.permissions.iter().any(|p| p == g) {
                let _ = std::fs::remove_dir_all(&dest);
                return Err(LugosError::Msg(format!(
                    "grant '{g}' is not declared in the app manifest"
                )));
            }
        }
        grants
    };

    let mut perms = Vec::with_capacity(grants.len());
    for g in grants {
        let p = Permission::parse_token(&g).map_err(LugosError::Msg)?;
        perms.push(p);
    }

    let grant = GrantRecord {
        app_id: app_id.to_string(),
        granted: perms,
        granted_at: chrono::Utc::now(),
    };
    permissions::save_grant(app, &grant)?;
    capability::generate_capability_file(app, &grant)?;
    Ok(())
}

pub fn uninstall_app(app: &AppHandle, app_id: &str) -> Result<(), LugosError> {
    let app_dir = apps_root(app)?.join(app_id);
    if app_dir.exists() {
        std::fs::remove_dir_all(&app_dir)?;
    }
    let grant = app.path().app_data_dir()?.join("grants").join(format!("{app_id}.json"));
    if grant.exists() {
        std::fs::remove_file(grant)?;
    }
    let cap = app
        .path()
        .app_data_dir()?
        .join("capabilities")
        .join(format!("{app_id}.json"));
    if cap.exists() {
        std::fs::remove_file(cap)?;
    }
    Ok(())
}

pub fn list_installed(app: &AppHandle) -> Result<Vec<AppManifest>, LugosError> {
    let base = apps_root(app)?;
    if !base.exists() {
        return Ok(vec![]);
    }
    let mut out = Vec::new();
    for entry in std::fs::read_dir(&base)? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let p = entry.path().join("app.manifest.json");
        if p.exists() {
            let m: AppManifest = serde_json::from_reader(File::open(p)?)?;
            out.push(m);
        }
    }
    out.sort_by(|a, b| a.display_name.cmp(&b.display_name));
    Ok(out)
}
