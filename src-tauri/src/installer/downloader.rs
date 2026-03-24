//! GitHub Releases API client: resolve latest asset and stream download.

use futures_util::StreamExt;
use serde::Deserialize;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Manager};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use crate::error::LugosError;
use crate::installer::extractor::{extract_to_tempdir, find_app_root};
use crate::registry::types::AppManifest;

#[derive(Debug, Deserialize)]
struct GhAsset {
    name: String,
    browser_download_url: String,
}

#[derive(Debug, Deserialize)]
struct GhRelease {
    assets: Vec<GhAsset>,
}

fn http_client() -> Result<reqwest::Client, LugosError> {
    reqwest::Client::builder()
        .user_agent("lugos-host/0.1 (releases; open source)")
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(Into::into)
}

pub fn parse_github_repo(repo_url: &str) -> Result<(String, String), LugosError> {
    let u = repo_url
        .trim_end_matches('/')
        .trim_end_matches(".git")
        .trim();
    const PREFIX: &str = "https://github.com/";
    if !u.starts_with(PREFIX) {
        return Err(LugosError::Msg(format!(
            "repository URL must start with {PREFIX}"
        )));
    }
    let rest = u.strip_prefix(PREFIX).ok_or_else(|| {
        LugosError::Msg("could not parse GitHub repository URL".into())
    })?;
    let mut parts = rest.splitn(3, '/');
    let owner = parts
        .next()
        .filter(|s| !s.is_empty())
        .ok_or_else(|| LugosError::Msg("missing GitHub owner".into()))?;
    let repo = parts
        .next()
        .filter(|s| !s.is_empty())
        .ok_or_else(|| LugosError::Msg("missing GitHub repo name".into()))?;
    Ok((owner.to_string(), repo.to_string()))
}

fn pick_asset(assets: &[GhAsset]) -> Option<&GhAsset> {
    let archives: Vec<_> = assets
        .iter()
        .filter(|a| {
            let n = a.name.to_lowercase();
            n.ends_with(".tar.gz") || n.ends_with(".tgz") || n.ends_with(".zip")
        })
        .collect();
    if archives.is_empty() {
        return None;
    }
    archives
        .iter()
        .find(|a| a.name.to_lowercase().contains("dist"))
        .copied()
        .or_else(|| archives.first().copied())
}

async fn download_to_path(url: &str, dest: &Path) -> Result<(), LugosError> {
    let client = http_client()?;
    let resp = client.get(url).send().await?.error_for_status()?;
    if let Some(parent) = dest.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }
    let mut file = File::create(dest).await?;
    let mut stream = resp.bytes_stream();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk).await?;
    }
    file.flush().await?;
    Ok(())
}

pub async fn fetch_app_manifest(repo_url: &str, app: &AppHandle) -> Result<AppManifest, LugosError> {
    let (owner, repo) = parse_github_repo(repo_url)?;
    let api = format!(
        "https://api.github.com/repos/{owner}/{repo}/releases/latest"
    );
    let client = http_client()?;
    let release: GhRelease = client
        .get(&api)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    let asset = pick_asset(&release.assets).ok_or_else(|| {
        LugosError::Msg(
            "latest release has no .tar.gz / .tgz / .zip asset; add a release attachment"
                .into(),
        )
    })?;

    let cache_root = app.path().app_cache_dir()?;
    let dl_dir = cache_root.join("lugos-downloads");
    tokio::fs::create_dir_all(&dl_dir).await?;
    let ext = if asset.name.to_lowercase().ends_with(".zip") {
        ".zip"
    } else {
        ".tar.gz"
    };
    let archive_path = dl_dir.join(format!(
        "{owner}-{repo}-latest-{}{ext}",
        chrono::Utc::now().timestamp_millis()
    ));
    download_to_path(&asset.browser_download_url, &archive_path).await?;

    let tmp_extract = dl_dir.join(format!(
        "extract-{}",
        chrono::Utc::now().timestamp_millis()
    ));
    extract_to_tempdir(&archive_path, &tmp_extract)?;
    let root = find_app_root(&tmp_extract)?;
    let manifest_path = root.join("app.manifest.json");
    let bytes = std::fs::read(&manifest_path)?;
    let manifest: AppManifest = serde_json::from_slice(&bytes)?;

    let _ = std::fs::remove_file(&archive_path);
    let _ = std::fs::remove_dir_all(&tmp_extract);

    Ok(manifest)
}

pub async fn download_latest_archive(
    repo_url: &str,
    app: &AppHandle,
) -> Result<PathBuf, LugosError> {
    let (owner, repo) = parse_github_repo(repo_url)?;
    let api = format!(
        "https://api.github.com/repos/{owner}/{repo}/releases/latest"
    );
    let client = http_client()?;
    let release: GhRelease = client
        .get(&api)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    let asset = pick_asset(&release.assets).ok_or_else(|| {
        LugosError::Msg(
            "latest release has no .tar.gz / .tgz / .zip asset; add a release attachment"
                .into(),
        )
    })?;

    let cache_root = app.path().app_cache_dir()?;
    let dl_dir = cache_root.join("lugos-downloads");
    tokio::fs::create_dir_all(&dl_dir).await?;
    let ext = if asset.name.to_lowercase().ends_with(".zip") {
        ".zip"
    } else {
        ".tar.gz"
    };
    let archive_path = dl_dir.join(format!(
        "{owner}-{repo}-install-{}{ext}",
        chrono::Utc::now().timestamp_millis()
    ));
    download_to_path(&asset.browser_download_url, &archive_path).await?;
    Ok(archive_path)
}
