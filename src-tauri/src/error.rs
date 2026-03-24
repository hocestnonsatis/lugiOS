//! Application-wide error type returned across the Tauri IPC boundary.

use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum LugosError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Http(#[from] reqwest::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Tauri(#[from] tauri::Error),
    #[error(transparent)]
    Rusqlite(#[from] rusqlite::Error),
    #[error(transparent)]
    Zip(#[from] zip::result::ZipError),
    #[error("permission denied: {0}")]
    PermissionDenied(String),
    #[error("app not found: {0}")]
    AppNotFound(String),
    #[error("{0}")]
    Msg(String),
}

impl LugosError {
    fn kind(&self) -> &'static str {
        match self {
            LugosError::Io(_) => "io",
            LugosError::Http(_) => "http",
            LugosError::Json(_) => "json",
            LugosError::Tauri(_) => "tauri",
            LugosError::Rusqlite(_) => "rusqlite",
            LugosError::Zip(_) => "zip",
            LugosError::PermissionDenied(_) => "permission_denied",
            LugosError::AppNotFound(_) => "app_not_found",
            LugosError::Msg(_) => "message",
        }
    }
}

impl Serialize for LugosError {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut s = serializer.serialize_struct("LugosError", 2)?;
        s.serialize_field("kind", self.kind())?;
        s.serialize_field("message", &self.to_string())?;
        s.end()
    }
}
