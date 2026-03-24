//! Permission declarations, grant persistence, and capability generation.

pub mod capability;
mod types;

pub use types::{GrantRecord, Permission};

use std::path::PathBuf;
use tauri::{AppHandle, Manager};

use crate::error::LugosError;

fn grants_dir(app: &AppHandle) -> Result<PathBuf, LugosError> {
    Ok(app.path().app_data_dir()?.join("grants"))
}

pub fn save_grant(app: &AppHandle, grant: &GrantRecord) -> Result<(), LugosError> {
    let dir = grants_dir(app)?;
    std::fs::create_dir_all(&dir)?;
    let path = dir.join(format!("{}.json", grant.app_id));
    std::fs::write(path, serde_json::to_vec_pretty(grant)?)?;
    Ok(())
}

pub fn load_grant(app: &AppHandle, app_id: &str) -> Result<Option<GrantRecord>, LugosError> {
    let path = grants_dir(app)?.join(format!("{app_id}.json"));
    if !path.exists() {
        return Ok(None);
    }
    let bytes = std::fs::read(path)?;
    let grant = serde_json::from_slice(&bytes)?;
    Ok(Some(grant))
}
