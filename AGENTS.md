# LugiOS — Agent Guide

This document is written for AI coding agents (Cursor, Claude Code, etc.) working on the LugiOS codebase. Read it fully before touching any file. It defines how the codebase is structured, how work is divided into tasks, what rules apply to every implementation, and what the agent must never do.

Always read `ARCHITECTURE.md` alongside this file.

---

## Agent Mental Model

LugiOS is a **Tauri v2 desktop application** that acts as a runtime and marketplace for sandboxed web-based mini-apps. It has three distinct execution contexts that must never be conflated:

| Context | What runs here | Language |
|---|---|---|
| **Host Rust backend** | Tauri commands, permission engine, installer, registry fetcher, storage | Rust |
| **Host Shell UI** | Marketplace, installed apps list, settings, permission dialogs | SvelteKit (TypeScript) |
| **App WebView** | Each installed app's own HTML/JS, sandboxed via AppBridge SDK | TypeScript (app author's code) |

The AppBridge SDK is the **only** communication channel between an app WebView and the host. It is injected by the runtime at window creation time. App authors do not call Tauri IPC directly.

---

## Repository Layout (Agent Reference)

```
lugiOS/
├── src-tauri/src/
│   ├── main.rs                  # Entry point, Tauri builder setup
│   ├── lib.rs                   # tauri::generate_handler! macro, plugin registration
│   ├── registry/                # Registry subsystem
│   │   ├── mod.rs
│   │   ├── fetcher.rs           # Fetch + cache registry.json from GitHub
│   │   └── types.rs             # RegistryEntry, AppManifest structs
│   ├── installer/               # Download + extract + verify
│   │   ├── mod.rs
│   │   ├── downloader.rs        # GitHub Releases API, streaming download
│   │   └── extractor.rs        # tar.gz / zip extraction to app data dir
│   ├── permissions/             # Permission engine
│   │   ├── mod.rs
│   │   ├── capability.rs        # Generate Tauri capability JSON at runtime
│   │   └── types.rs             # Permission enum, GrantRecord
│   ├── runtime/                 # App window lifecycle
│   │   ├── mod.rs
│   │   ├── window_manager.rs    # Create / close / track WebviewWindows
│   │   └── bridge.rs            # SDK injection into WebView
│   └── storage/
│       └── mod.rs               # App-scoped SQLite KV commands
│
├── src/                         # SvelteKit host shell
│   ├── routes/
│   │   ├── marketplace/         # App browsing + install trigger
│   │   ├── installed/           # Manage installed apps
│   │   └── settings/
│   └── lib/
│       ├── components/
│       └── stores/              # Svelte stores for registry, installed apps
│
├── sdk/                         # AppBridge SDK (injected into app WebViews)
│   └── src/
│       ├── index.ts
│       ├── storage.ts
│       ├── notifications.ts
│       ├── filesystem.ts
│       ├── network.ts
│       ├── clipboard.ts
│       ├── audio.ts
│       └── errors.ts
│
└── docs/
    ├── ARCHITECTURE.md
    └── AGENTS.md               # This file
```

---

## Implementation Rules

These rules apply to every file the agent creates or modifies. No exceptions.

### General

1. **One file per concern.** Each Rust module file implements exactly one coherent responsibility. If a file grows beyond ~200 lines, split it.
2. **No placeholder code.** Every function must be fully implemented. Do not leave `todo!()`, `unimplemented!()`, `// TODO`, or stub bodies unless the task explicitly scopes a skeleton phase.
3. **Short, focused files.** Target 80–150 lines per file. Extract helpers and types aggressively.
4. **Explicit error handling.** In Rust, use `thiserror` for error types and propagate with `?`. Never use `.unwrap()` or `.expect()` in non-test code.
5. **No unused imports.** Remove them before considering a task complete.

### Rust (src-tauri/)

6. All Tauri commands must be registered in `lib.rs` via `tauri::generate_handler!`. Never register commands inside `main.rs`.
7. Every Tauri command function signature must be `async` unless there is a documented reason it cannot be.
8. Use `serde::{Serialize, Deserialize}` on all structs that cross the IPC boundary. Derive both unless one direction is explicitly not needed.
9. Use `tauri::AppHandle` as the first parameter of every command that needs access to app state or paths. Do not use global statics.
10. File system paths must always be constructed via `app.path().app_data_dir()` or equivalent Tauri path APIs. Never hardcode OS-specific paths.
11. `reqwest` is the HTTP client. Always use the async client. Set a reasonable timeout (10s default) on every request.
12. Use `tokio` as the async runtime (Tauri v2 default). Do not introduce a second runtime.

### SvelteKit (src/)

13. All Tauri IPC calls from the frontend use `@tauri-apps/api/core` → `invoke`. Never import from deprecated v1 paths.
14. Keep Svelte components under 150 lines. Extract sub-components for anything rendered in a loop or conditionally.
15. Use Svelte stores (`lib/stores/`) for all shared state (registry list, installed apps, permission grants). Do not pass large props down multiple component levels.
16. No inline styles. Use Tailwind utility classes exclusively.
17. All `invoke` calls must be wrapped in try/catch with typed error handling.

### AppBridge SDK (sdk/)

18. The SDK compiles to a single IIFE bundle (`sdk/dist/bridge.iife.js`). Do not introduce dynamic imports.
19. Every exported method must check the app's granted permissions before making an IPC call. Throw `PermissionDeniedError` (from `errors.ts`) immediately if the permission is not granted.
20. The granted permission set is injected by the host at window creation time as a global constant. The SDK reads it from `window.__LUGOS_GRANTS__` (a frozen array of permission token strings).
21. The SDK must never import Tauri's own JS API directly. All communication goes through `window.__TAURI_INTERNALS__` invoke primitive to keep the bundle self-contained.

---

## Task Format

Each task handed to the agent follows this structure. Read the full task before writing any code.

```
## Task: <subsystem> — <short title>

**Scope:** <which files are in scope>
**Depends on:** <tasks or modules that must exist first>
**Goal:** <what the implementation must achieve>
**Acceptance criteria:**
- [ ] criterion 1
- [ ] criterion 2
**Do not:**
- list of explicit exclusions
```

---

## Task List

Work through tasks in order. Do not start a task until all its dependencies are marked complete. Mark a task complete by checking all its acceptance criteria.

---

### Task 01 — Rust: Project scaffold & error types

**Scope:** `src-tauri/src/main.rs`, `src-tauri/src/lib.rs`, `src-tauri/Cargo.toml`
**Depends on:** nothing
**Goal:** Set up the Tauri v2 application builder with plugin registration stubs and a global `LugosError` type.

**Acceptance criteria:**
- [ ] `Cargo.toml` declares: `tauri`, `tauri-plugin-sql`, `tauri-plugin-fs`, `tauri-plugin-notification`, `reqwest` (with `json`, `stream` features), `tokio` (full), `serde`, `serde_json`, `thiserror`, `flate2`, `tar`, `zip`
- [ ] `lib.rs` exports `run()` which builds the Tauri app, registers all plugins, and calls `generate_handler!` with an empty list for now
- [ ] `main.rs` calls `lugios_lib::run()`
- [ ] A `LugosError` enum exists in `src-tauri/src/lib.rs` using `thiserror`, with variants: `Io(#[from] std::io::Error)`, `Http(#[from] reqwest::Error)`, `Json(#[from] serde_json::Error)`, `PermissionDenied(String)`, `AppNotFound(String)`
- [ ] `LugosError` implements `Serialize` so it can be returned from Tauri commands

**Do not:**
- Add any application logic yet
- Add any Tauri commands yet

---

### Task 02 — Rust: Registry types & fetcher

**Scope:** `src-tauri/src/registry/`
**Depends on:** Task 01
**Goal:** Implement the registry fetch-and-cache subsystem.

**Acceptance criteria:**
- [ ] `types.rs` defines `RegistryEntry` and `AppManifest` structs matching the schemas in `ARCHITECTURE.md`, fully derived with `Serialize`, `Deserialize`, `Debug`, `Clone`
- [ ] `AppManifest` includes: `id`, `display_name`, `version`, `description`, `icon`, `entry_point`, `permissions` (Vec of strings), `window` (width, height, resizable, always_on_top)
- [ ] `fetcher.rs` implements `fetch_registry(app: &AppHandle) -> Result<Vec<RegistryEntry>, LugosError>` which:
  - Fetches `https://raw.githubusercontent.com/hocestnonsatis/lugiOS/main/registry/registry.json` (or `LUGIOS_REGISTRY_URL` if set)
  - Parses the JSON into `Vec<RegistryEntry>`
  - Caches the result to `app_data_dir()/registry_cache.json` with a timestamp
  - Returns the cached version if the cache is less than 1 hour old
- [ ] `mod.rs` exports the public API
- [ ] Two Tauri commands: `get_registry() -> Result<Vec<RegistryEntry>, LugosError>` and `refresh_registry() -> Result<Vec<RegistryEntry>, LugosError>` — both registered in `lib.rs`

**Do not:**
- Implement any UI
- Fetch individual app manifests here (that is the installer's job)

---

### Task 03 — Rust: Permission types & engine

**Scope:** `src-tauri/src/permissions/`
**Depends on:** Task 01
**Goal:** Implement the permission declaration model and runtime capability file generator.

**Acceptance criteria:**
- [ ] `types.rs` defines a `Permission` enum covering all tokens in the permissions table in `ARCHITECTURE.md`. Use string serialization (`#[serde(rename = "...")]`) matching the token strings exactly (e.g. `"filesystem:read"`)
- [ ] `types.rs` defines `GrantRecord { app_id: String, granted: Vec<Permission>, granted_at: DateTime<Utc> }` using `chrono`
- [ ] `capability.rs` implements `generate_capability_file(app: &AppHandle, grant: &GrantRecord) -> Result<(), LugosError>` which writes a valid Tauri v2 capability JSON file to `app_data_dir()/capabilities/{app_id}.json`
- [ ] `mod.rs` implements `save_grant(app: &AppHandle, grant: &GrantRecord) -> Result<(), LugosError>` which writes the grant record to `app_data_dir()/grants/{app_id}.json`
- [ ] `mod.rs` implements `load_grant(app: &AppHandle, app_id: &str) -> Result<Option<GrantRecord>, LugosError>`
- [ ] Tauri command: `get_grant(app_id: String) -> Result<Option<GrantRecord>, LugosError>` registered in `lib.rs`

**Do not:**
- Show any UI dialogs from Rust — the permission prompt dialog is a SvelteKit component
- Implement the app window launch yet

---

### Task 04 — Rust: Installer

**Scope:** `src-tauri/src/installer/`
**Depends on:** Task 02, Task 03
**Goal:** Implement the full install flow: fetch latest release from GitHub, download, extract, and record the installation.

**Acceptance criteria:**
- [ ] `downloader.rs` implements `fetch_app_manifest(repo_url: &str) -> Result<AppManifest, LugosError>` which:
  - Parses `owner/repo` from the GitHub repo URL
  - Calls `https://api.github.com/repos/{owner}/{repo}/releases/latest`
  - Finds the tarball or zip asset
  - Downloads the asset with streaming to a temp file
- [ ] `extractor.rs` implements `extract_app(temp_path: &Path, dest: &Path) -> Result<(), LugosError>` supporting both `.tar.gz` and `.zip`
- [ ] `mod.rs` implements `install_app(app: &AppHandle, app_id: &str, repo_url: &str, grants: Vec<String>) -> Result<(), LugosError>` which orchestrates: download → extract to `app_data_dir()/apps/{app_id}/` → parse manifest → save grant record → generate capability file
- [ ] `mod.rs` implements `uninstall_app(app: &AppHandle, app_id: &str) -> Result<(), LugosError>` which removes the app directory and grant file
- [ ] `mod.rs` implements `list_installed(app: &AppHandle) -> Result<Vec<AppManifest>, LugosError>` which reads all manifests from `app_data_dir()/apps/`
- [ ] Tauri commands: `install_app`, `uninstall_app`, `list_installed` — all registered in `lib.rs`

**Do not:**
- Require a GitHub auth token — all calls must work anonymously
- Implement the permission dialog — that is in SvelteKit

---

### Task 05 — Rust: App storage (KV)

**Scope:** `src-tauri/src/storage/mod.rs`
**Depends on:** Task 01
**Goal:** App-scoped key-value storage backed by SQLite.

**Acceptance criteria:**
- [ ] Uses `tauri-plugin-sql` to open a per-app SQLite database at `app_data_dir()/data/{app_id}/store.db`
- [ ] Implements Tauri commands: `storage_get(app_id: String, key: String) -> Result<Option<String>, LugosError>` and `storage_set(app_id: String, key: String, value: String) -> Result<(), LugosError>` and `storage_delete(app_id: String, key: String) -> Result<(), LugosError>`
- [ ] The `app_id` parameter is always provided by the host, never trusted from the app's JS — it is resolved from the `WebviewWindow` label in the runtime layer
- [ ] All three commands registered in `lib.rs`

**Do not:**
- Expose commands that allow one app to read another app's storage
- Allow arbitrary SQL from the app — only KV operations

---

### Task 06 — Rust: App runtime (window manager + bridge injection)

**Scope:** `src-tauri/src/runtime/`
**Depends on:** Task 03, Task 04, Task 05
**Goal:** Launch and manage isolated WebviewWindows for installed apps, injecting the AppBridge SDK.

**Acceptance criteria:**
- [ ] `window_manager.rs` implements `launch_app(app: &AppHandle, app_id: &str) -> Result<(), LugosError>` which:
  - Loads the grant record for the app
  - Creates a `WebviewWindowBuilder` with label = `app:{app_id}`, pointing to the local `index.html` using a custom protocol or local file URL
  - Applies window size/resizable from the manifest
  - Calls `bridge.rs` to inject the SDK before the page finishes loading
- [ ] `window_manager.rs` implements `close_app(app: &AppHandle, app_id: &str) -> Result<(), LugosError>`
- [ ] `window_manager.rs` implements `list_running(app: &AppHandle) -> Vec<String>` returning app IDs of currently open windows
- [ ] `bridge.rs` implements `inject_sdk(window: &WebviewWindow, grant: &GrantRecord) -> Result<(), LugosError>` which:
  - Reads the compiled SDK bundle from the host's asset directory
  - Evaluates a small bootstrap script that sets `window.__LUGOS_GRANTS__` to the frozen serialized grant
  - Evaluates the SDK bundle
- [ ] Tauri commands: `launch_app(app_id: String)`, `close_app(app_id: String)`, `list_running_apps()` — registered in `lib.rs`

**Do not:**
- Expose the raw `__TAURI__` object to app WebViews
- Use `data:` URLs for the SDK injection — read from asset file

---

### Task 07 — SDK: AppBridge core

**Scope:** `sdk/src/`
**Depends on:** Task 06 (defines the IPC command names)
**Goal:** Implement the AppBridge SDK that is injected into app WebViews.

**Acceptance criteria:**
- [ ] `errors.ts` exports `PermissionDeniedError extends Error` and `LugiOSError extends Error`
- [ ] `index.ts` reads `window.__LUGOS_GRANTS__` (string array) and assembles the `AppBridge` object, exposing only the modules whose permissions are granted
- [ ] `storage.ts` exports `get(key: string): Promise<string | null>`, `set(key: string, value: string): Promise<void>`, `del(key: string): Promise<void>` — requires `storage` permission
- [ ] `notifications.ts` exports `send(opts: { title: string; body?: string }): Promise<void>` — requires `notifications` permission
- [ ] `clipboard.ts` exports `read(): Promise<string>` (requires `clipboard:read`), `write(text: string): Promise<void>` (requires `clipboard:write`)
- [ ] `network.ts` exports `fetch(url: string, init?: RequestInit): Promise<Response>` — requires `network` or matching `network:domain:*`; validates URL against grant if domain-restricted
- [ ] `filesystem.ts` exports `pickAndRead(opts: { accept?: string[] }): Promise<{ name: string; content: Uint8Array }>` (requires `filesystem:read`) and `saveDialog(opts: { defaultName?: string; content: Uint8Array }): Promise<void>` (requires `filesystem:write`)
- [ ] `audio.ts` exports `play(assetPath: string): Promise<void>` — requires `audio:play`; assetPath is relative to the app bundle
- [ ] The SDK compiles to `sdk/dist/bridge.iife.js` via a build script (esbuild or tsup, no bundler lock-in)
- [ ] `window.AppBridge` is set to a frozen object

**Do not:**
- Import from `@tauri-apps/api` — use `window.__TAURI_INTERNALS__` invoke directly
- Expose any methods that are not tied to a declared permission

---

### Task 08 — SvelteKit: Host shell scaffold

**Scope:** `src/`
**Depends on:** Task 02 (registry commands), Task 04 (installer commands)
**Goal:** Build the host shell UI — marketplace, installed apps, and permission dialog.

**Acceptance criteria:**
- [ ] `src/lib/stores/registry.ts` — a Svelte store that calls `invoke('get_registry')` on init and exposes `{ entries, loading, error, refresh }`
- [ ] `src/lib/stores/installed.ts` — a Svelte store that calls `invoke('list_installed')` and tracks local state updates after install/uninstall
- [ ] `src/routes/marketplace/+page.svelte` — displays registry entries as cards with name, description, author, tags, verified badge; each card has an "Install" button
- [ ] `src/routes/marketplace/+page.svelte` — "Install" opens a Permission Dialog modal before calling `invoke('install_app', ...)`
- [ ] `src/lib/components/PermissionDialog.svelte` — lists all permissions the app requests with human-readable descriptions; has "Allow & Install" and "Cancel" buttons
- [ ] `src/routes/installed/+page.svelte` — lists installed apps; each has "Open" (calls `invoke('launch_app', ...)`) and "Uninstall" buttons
- [ ] A sidebar or top nav links between Marketplace and Installed views
- [ ] All `invoke` calls wrapped in try/catch with visible error states

**Do not:**
- Build a settings page yet
- Implement app update checking yet

---

### Task 09 — Integration: End-to-end smoke test

**Scope:** No new files. Verify the system works together.
**Depends on:** Tasks 01–08
**Goal:** Confirm the full install → launch → SDK use flow works on the development machine.

**Acceptance criteria:**
- [ ] `cargo build` succeeds with no warnings
- [ ] `npm run dev` (or `tauri dev`) launches the host shell
- [ ] Marketplace fetches and displays registry entries
- [ ] A real test app (a minimal HTML file with `window.AppBridge.storage.set(...)`) can be installed from a GitHub repo
- [ ] Permission dialog correctly reflects the permissions declared in the app's manifest
- [ ] After install, the app appears in Installed
- [ ] Clicking "Open" launches the app in its own window
- [ ] Calling `AppBridge.storage.set` from the app persists data; `AppBridge.storage.get` retrieves it
- [ ] Calling a method without the required permission throws `PermissionDeniedError`

---

## What the Agent Must Never Do

- **Never expose `__TAURI__` to app WebViews.** The CSP and IPC boundary must be maintained at all times.
- **Never use `.unwrap()` or `.expect()` in non-test Rust code.**
- **Never hardcode paths.** Always use Tauri's path resolver APIs.
- **Never trust `app_id` from JavaScript.** Always resolve it from the `WebviewWindow` label on the Rust side.
- **Never add a dependency not listed in Task 01's Cargo.toml** without documenting the reason in a comment above the `[dependencies]` entry.
- **Never commit `sdk/dist/` or build artifacts.** These are generated.
- **Never implement shell execution** — not even behind a feature flag. It is out of scope.

---

## Coding Conventions

### Naming

- Rust: `snake_case` for everything. Structs `PascalCase`. Errors `LugosError`.
- TypeScript: `camelCase` functions, `PascalCase` types/classes, `SCREAMING_SNAKE` for injected globals.
- Svelte components: `PascalCase.svelte`.
- Tauri command names (invoked from frontend): `snake_case` strings, matching the Rust function name exactly.

### Commit Scope Tags (for reference)

```
feat(registry): ...
feat(installer): ...
feat(permissions): ...
feat(runtime): ...
feat(sdk): ...
feat(ui): ...
fix(installer): ...
```

### File Header

Every new Rust file should begin with a one-line doc comment describing its single responsibility:

```rust
//! Fetches and caches the LugiOS app registry from GitHub.
```

---

## Quick Reference: Key IPC Commands

| Frontend invoke string | Rust handler | Returns |
|---|---|---|
| `get_registry` | `registry::fetcher` | `Vec<RegistryEntry>` |
| `refresh_registry` | `registry::fetcher` | `Vec<RegistryEntry>` |
| `install_app` | `installer::mod` | `()` |
| `uninstall_app` | `installer::mod` | `()` |
| `list_installed` | `installer::mod` | `Vec<AppManifest>` |
| `get_grant` | `permissions::mod` | `Option<GrantRecord>` |
| `launch_app` | `runtime::window_manager` | `()` |
| `close_app` | `runtime::window_manager` | `()` |
| `list_running_apps` | `runtime::window_manager` | `Vec<String>` |
| `storage_get` | `storage::mod` | `Option<String>` |
| `storage_set` | `storage::mod` | `()` |
| `storage_delete` | `storage::mod` | `()` |
| `check_app_updates` | `updater::mod` | `Vec<AppUpdateStatus>` |
| `upgrade_app` | `commands` (installer) | `()` |
| `get_host_settings` | `commands` | `HostSettingsPayload` |
| `set_host_registry_url` | `commands` | `HostSettingsPayload` |
