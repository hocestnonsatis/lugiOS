//! Locates and loads the AppBridge IIFE bundle for mini-app webviews.

use tauri::{AppHandle, Manager};

use crate::error::LugosError;

/// Reads `bridge.iife.js` from bundled resources (release) or `src-tauri/resources` (dev).
pub fn read_bundle(app: &AppHandle) -> Result<String, LugosError> {
    let resource = app.path().resource_dir()?.join("bridge.iife.js");
    if resource.exists() {
        return Ok(std::fs::read_to_string(resource)?);
    }
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let dev_path = manifest_dir.join("resources").join("bridge.iife.js");
    std::fs::read_to_string(&dev_path).map_err(|_| {
        LugosError::Msg(
            "AppBridge bundle missing: run `npm run build:sdk` so src-tauri/resources/bridge.iife.js exists"
                .into(),
        )
    })
}
