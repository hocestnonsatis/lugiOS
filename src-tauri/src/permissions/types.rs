//! Permission tokens and persisted grant records.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Permission {
    Storage,
    Notifications,
    ClipboardRead,
    ClipboardWrite,
    AudioPlay,
    FilesystemRead,
    FilesystemWrite,
    Network,
    NetworkDomain(String),
}

impl Permission {
    pub fn parse_token(s: &str) -> Result<Self, String> {
        if let Some(host) = s.strip_prefix("network:domain:") {
            if host.is_empty() {
                return Err("network:domain requires a host".into());
            }
            return Ok(Permission::NetworkDomain(host.to_string()));
        }
        match s {
            "storage" => Ok(Permission::Storage),
            "notifications" => Ok(Permission::Notifications),
            "clipboard:read" => Ok(Permission::ClipboardRead),
            "clipboard:write" => Ok(Permission::ClipboardWrite),
            "audio:play" => Ok(Permission::AudioPlay),
            "filesystem:read" => Ok(Permission::FilesystemRead),
            "filesystem:write" => Ok(Permission::FilesystemWrite),
            "network" => Ok(Permission::Network),
            _ => Err(format!("unknown permission token: {s}")),
        }
    }

    pub fn as_token(&self) -> String {
        match self {
            Permission::Storage => "storage".into(),
            Permission::Notifications => "notifications".into(),
            Permission::ClipboardRead => "clipboard:read".into(),
            Permission::ClipboardWrite => "clipboard:write".into(),
            Permission::AudioPlay => "audio:play".into(),
            Permission::FilesystemRead => "filesystem:read".into(),
            Permission::FilesystemWrite => "filesystem:write".into(),
            Permission::Network => "network".into(),
            Permission::NetworkDomain(h) => format!("network:domain:{h}"),
        }
    }
}

impl Serialize for Permission {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.as_token().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Permission {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Permission::parse_token(&s).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrantRecord {
    pub app_id: String,
    pub granted: Vec<Permission>,
    pub granted_at: DateTime<Utc>,
}
