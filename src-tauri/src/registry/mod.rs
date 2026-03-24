//! Registry subsystem: public types and fetch helpers.

mod fetcher;
pub mod types;

pub use fetcher::{fetch_registry, refresh_registry};
pub use types::{AppManifest, RegistryEntry};
