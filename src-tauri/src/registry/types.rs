//! Serializable registry and app manifest types for JSON interchange.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistryEntry {
    pub id: String,
    pub display_name: String,
    pub author: String,
    pub repo: String,
    pub description: String,
    pub tags: Vec<String>,
    pub verified: bool,
    pub published_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManifestWindow {
    #[serde(default = "default_width")]
    pub width: u32,
    #[serde(default = "default_height")]
    pub height: u32,
    #[serde(default)]
    pub resizable: bool,
    #[serde(default)]
    pub always_on_top: bool,
}

impl Default for ManifestWindow {
    fn default() -> Self {
        Self {
            width: default_width(),
            height: default_height(),
            resizable: true,
            always_on_top: false,
        }
    }
}

fn default_width() -> u32 {
    800
}

fn default_height() -> u32 {
    600
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppManifest {
    pub id: String,
    pub display_name: String,
    pub version: String,
    pub description: String,
    pub icon: String,
    pub entry_point: String,
    pub permissions: Vec<String>,
    #[serde(default)]
    pub window: ManifestWindow,
}
