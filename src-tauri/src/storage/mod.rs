//! App-scoped key-value storage backed by SQLite (one database per mini-app).

use std::path::PathBuf;
use tauri::{AppHandle, Manager};

use crate::error::LugosError;
use crate::util::sanitize_app_id;

fn db_path(app: &AppHandle, app_id: &str) -> Result<PathBuf, LugosError> {
    let id = sanitize_app_id(app_id)?;
    let dir = app.path().app_data_dir()?.join("data").join(&id);
    std::fs::create_dir_all(&dir)?;
    Ok(dir.join("store.db"))
}

fn open_conn(path: &PathBuf) -> Result<rusqlite::Connection, LugosError> {
    let c = rusqlite::Connection::open(path)?;
    c.execute_batch(
        "CREATE TABLE IF NOT EXISTS lugos_kv (
            key TEXT PRIMARY KEY NOT NULL,
            value TEXT NOT NULL
        );",
    )?;
    Ok(c)
}

pub fn storage_get_sync(app: &AppHandle, app_id: &str, key: &str) -> Result<Option<String>, LugosError> {
    let path = db_path(app, app_id)?;
    let conn = open_conn(&path)?;
    let mut stmt = conn.prepare("SELECT value FROM lugos_kv WHERE key = ?1")?;
    let mut rows = stmt.query(rusqlite::params![key])?;
    if let Some(row) = rows.next()? {
        Ok(Some(row.get(0)?))
    } else {
        Ok(None)
    }
}

pub fn storage_set_sync(
    app: &AppHandle,
    app_id: &str,
    key: &str,
    value: &str,
) -> Result<(), LugosError> {
    let path = db_path(app, app_id)?;
    let conn = open_conn(&path)?;
    conn.execute(
        "INSERT INTO lugos_kv (key, value) VALUES (?1, ?2)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        rusqlite::params![key, value],
    )?;
    Ok(())
}

pub fn storage_delete_sync(app: &AppHandle, app_id: &str, key: &str) -> Result<(), LugosError> {
    let path = db_path(app, app_id)?;
    let conn = open_conn(&path)?;
    conn.execute("DELETE FROM lugos_kv WHERE key = ?1", rusqlite::params![key])?;
    Ok(())
}
