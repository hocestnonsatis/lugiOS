//! Mini-app window lifecycle and AppBridge injection.

mod bridge;
mod window_manager;

pub use window_manager::{close_app, launch_app, list_running};
