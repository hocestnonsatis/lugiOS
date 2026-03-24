# LugiOS Architecture

## Overview

LugiOS is a desktop application runtime and marketplace that allows developers to publish web-based mini-applications as easily as publishing an npm package. Users install and run these apps through a host runtime built with Tauri v2. Apps are distributed via GitHub repositories and discovered through a central community-maintained registry.

The system is analogous to a browser extension ecosystem (Chrome Web Store model) but for the desktop, where each app is a sandboxed WebView window with a declarative permission model.

---

## High-Level Architecture

```
┌──────────────────────────────────────────────────────┐
│                  Registry Repo (GitHub)               │
│             registry.json  ←  developer PRs          │
└─────────────────────────┬────────────────────────────┘
                          │ HTTPS fetch
┌─────────────────────────▼────────────────────────────┐
│                  LugiOS Host (Tauri v2)                │
│                                                      │
│  ┌───────────────┐        ┌────────────────────────┐ │
│  │ Marketplace   │        │   App Runtime Engine   │ │
│  │ (SvelteKit)   │        │  (WebviewWindow pool)  │ │
│  └──────┬────────┘        └──────────┬─────────────┘ │
│         │                            │               │
│  ┌──────▼────────────────────────────▼─────────────┐ │
│  │              Permission Engine                   │ │
│  │   (runtime capability generation, Tauri v2)     │ │
│  └─────────────────────────────────────────────────┘ │
│                          │                           │
│  ┌───────────────────────▼─────────────────────────┐ │
│  │              App Bridge SDK                      │ │
│  │     (injected into each app WebView)             │ │
│  └─────────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────────┘
                          │ install (tarball download)
┌─────────────────────────▼────────────────────────────┐
│               App Repository (GitHub)                 │
│   app.manifest.json  +  dist/  +  package.json       │
└──────────────────────────────────────────────────────┘
```

---

## Repository Structure

LugiOS is a Cargo + npm monorepo.

```
lugiOS/
├── src-tauri/                   # Tauri v2 Rust backend
│   ├── src/
│   │   ├── main.rs
│   │   ├── lib.rs
│   │   ├── registry/            # Registry fetch & cache
│   │   │   ├── mod.rs
│   │   │   ├── fetcher.rs       # GitHub API calls
│   │   │   └── types.rs         # RegistryEntry, AppManifest
│   │   ├── installer/           # Download, extract, verify
│   │   │   ├── mod.rs
│   │   │   ├── downloader.rs
│   │   │   └── extractor.rs
│   │   ├── permissions/         # Permission engine
│   │   │   ├── mod.rs
│   │   │   ├── capability.rs    # Runtime capability generation
│   │   │   └── types.rs         # Permission enum & grant model
│   │   ├── runtime/             # App window management
│   │   │   ├── mod.rs
│   │   │   ├── window_manager.rs
│   │   │   └── bridge.rs        # AppBridge injection
│   │   └── storage/             # App-scoped key-value store
│   │       └── mod.rs
│   ├── capabilities/            # Static host capabilities
│   └── Cargo.toml
│
├── src/                         # SvelteKit frontend (host shell UI)
│   ├── routes/
│   │   ├── +layout.svelte
│   │   ├── marketplace/         # Browse & install apps
│   │   ├── installed/           # Manage installed apps
│   │   └── settings/
│   ├── lib/
│   │   ├── components/
│   │   └── stores/
│   └── app.html
│
├── sdk/                         # AppBridge SDK (injected into app WebViews)
│   ├── src/
│   │   ├── index.ts             # window.AppBridge entry
│   │   ├── storage.ts
│   │   ├── notifications.ts
│   │   ├── filesystem.ts
│   │   └── network.ts
│   ├── package.json
│   └── tsconfig.json
│
├── registry/                    # Community registry (separate repo, referenced here for spec)
│   └── registry.json
│
├── docs/
│   ├── ARCHITECTURE.md          # This file
│   ├── AGENTS.md
│   └── app-developer-guide.md
│
├── package.json
├── tauri.conf.json
└── Cargo.toml                   # Workspace root
```

---

## Core Subsystems

### 1. Registry

The registry is `registry/registry.json` in this monorepo, published on `main` and fetched from
`https://raw.githubusercontent.com/hocestnonsatis/lugiOS/main/registry/registry.json`.
Set the `LUGIOS_REGISTRY_URL` environment variable to override the download URL at runtime.

**registry.json schema:**

```json
[
  {
    "id": "pomodoro-timer",
    "displayName": "Pomodoro Timer",
    "author": "somedev",
    "repo": "https://github.com/somedev/pomodoro-timer",
    "description": "A minimal Pomodoro timer for focused work sessions.",
    "tags": ["productivity", "timer"],
    "verified": false,
    "publishedAt": "2025-01-10T00:00:00Z",
    "logoUrl": "https://example.com/icons/pomodoro.png"
  }
]
```

Optional **`logoUrl`**: HTTPS image used on marketplace cards and the app detail header. If omitted, the host uses the GitHub repository owner’s avatar (and the API-backed `ownerAvatarUrl` on the detail view when available).

The host app fetches and caches this file on launch, with a configurable TTL (default: 1 hour). No backend server is required.

---

### 2. App Package Format

Every LugiOS app is a GitHub repository with the following required structure:

```
my-app/
├── app.manifest.json     # LugiOS-specific manifest (required)
├── package.json          # npm metadata (required)
├── icon.png              # App icon, 512x512 (required)
├── dist/
│   └── index.html        # Build output entry point (required)
└── src/                  # Source files (optional, for developer reference)
```

**app.manifest.json schema:**

```json
{
  "id": "pomodoro-timer",
  "displayName": "Pomodoro Timer",
  "version": "1.2.0",
  "description": "A minimal Pomodoro timer.",
  "icon": "icon.png",
  "entryPoint": "dist/index.html",
  "permissions": [
    "notifications",
    "storage",
    "audio:play"
  ],
  "window": {
    "width": 400,
    "height": 600,
    "resizable": false,
    "alwaysOnTop": false
  }
}
```

Apps are distributed as **GitHub Release tarballs**. The host downloads the tarball attached to the latest release tag of the app's repository. This means developers do not need to commit their `dist/` folder — they upload it as a release artifact via CI.

---

### 3. Permission System

Permissions are declared in `app.manifest.json` and presented to the user during installation. No permission is granted implicitly; every capability requires explicit user approval at install time.

**Permission definitions:**

| Permission Token | Capability Granted |
|---|---|
| `storage` | App-scoped key-value store (no cross-app access) |
| `notifications` | OS-level desktop notifications |
| `clipboard:read` | Read clipboard contents |
| `clipboard:write` | Write to clipboard |
| `audio:play` | Play audio files bundled with the app |
| `filesystem:read` | Open a file picker; read user-selected files |
| `filesystem:write` | Save file dialog; write to user-selected paths |
| `network` | Unrestricted outbound HTTP/HTTPS fetch |
| `network:domain:<host>` | Fetch restricted to a single domain |

**Explicitly unavailable (Phase 1):**

- `shell` — no arbitrary command execution
- `process` — no spawning child processes
- `filesystem:read:arbitrary` — no access to paths the user did not explicitly select

At install time, the Permission Engine generates a per-app Tauri v2 capability JSON file and saves it alongside the extracted app bundle. This file is loaded when the app's WebviewWindow is created, ensuring Tauri's own ACL enforces the boundaries at the IPC layer.

---

### 4. App Runtime Engine

Each installed app runs in an isolated `WebviewWindow` instance. The host maintains a window registry mapping `app-id → WebviewWindow handle`.

**Launch sequence:**

1. User clicks "Open" on an installed app.
2. Host reads the app's stored capability grant from disk.
3. Host generates (or reuses) the Tauri capability file for this app.
4. A new `WebviewWindow` is created pointing to `dist/index.html` (served from the local install path, not a dev server).
5. Before the page loads, the AppBridge SDK bundle is injected into the WebView via `webview.evaluate_script()`.
6. The app's JavaScript interacts only through `window.AppBridge` — direct `__TAURI__` bindings are not exposed.

Multiple apps can run simultaneously in separate windows.

---

### 5. AppBridge SDK

The AppBridge is a TypeScript bundle compiled from `sdk/src/` and injected into each app WebView at window creation time. It exposes a typed API surface scoped strictly to the app's granted permissions.

Calling a method that corresponds to a non-granted permission throws a `PermissionDeniedError` at the SDK level before any IPC call is made, providing fast developer feedback.

**App-side usage:**

```typescript
const { storage, notifications, fs } = window.AppBridge;

// Storage
await storage.set("lastSession", JSON.stringify({ duration: 25 }));
const val = await storage.get("lastSession");

// Notifications
await notifications.send({ title: "Break time!", body: "25 minutes done." });

// Filesystem (triggers OS file picker)
const file = await fs.pickAndRead({ accept: [".txt", ".md"] });
```

**SDK package layout:**

```
sdk/src/
├── index.ts            # Assembles AppBridge from submodules, checks grants
├── storage.ts          # IPC → storage Tauri command
├── notifications.ts    # IPC → notification Tauri command
├── filesystem.ts       # IPC → fs Tauri commands
├── network.ts          # fetch wrapper (enforces domain whitelist if applicable)
├── clipboard.ts
├── audio.ts
└── errors.ts           # PermissionDeniedError, LugiOSError
```

---

### 6. Install Flow

```
User clicks "Install"
    ↓
Host calls GitHub Releases API
  GET /repos/:owner/:repo/releases/latest
    ↓
Download tarball asset (app-dist.tar.gz or zip)
    ↓
Verify: parse app.manifest.json, check required fields
    ↓
Show Permission Dialog
  - Lists all requested permissions
  - User approves or cancels
    ↓
Extract to ~/.local/share/lugios/apps/:app-id/  (Linux)
              ~/Library/Application Support/lugios/apps/:app-id/  (macOS)
              %APPDATA%\lugios\apps\:app-id\  (Windows)
    ↓
Write granted permissions to
  ~/.local/share/lugios/grants/:app-id.json
    ↓
Generate Tauri capability file at
  src-tauri/capabilities/runtime/:app-id.json
    ↓
App appears in "Installed" list — ready to launch
```

---

### 7. Local Storage (App-scoped)

Each app has access to an isolated key-value store backed by a SQLite database (via `tauri-plugin-sql`). The store is namespaced by app ID at the Rust layer — apps cannot access each other's data regardless of what keys they use.

Store location: `~/.local/share/lugios/data/:app-id/store.db`

---

## Data Flow Summary

```
Registry fetch:   Host → GitHub (registry.json) → parse → display in Marketplace UI
Install:          Host → GitHub Releases API → download → verify → permission prompt → extract → grant write → ready
Launch:           Host → read grant → create WebviewWindow → inject SDK → app runs
IPC call:         App JS → window.AppBridge.X() → SDK checks grant → Tauri IPC → Rust handler → OS
```

---

## Tech Stack

| Layer | Technology |
|---|---|
| Host runtime | Tauri v2 |
| Rust backend | Rust (stable) |
| Host shell UI | SvelteKit (inside Tauri WebviewWindow) |
| App WebViews | Tauri `WebviewWindow` (one per app) |
| App SDK | TypeScript, compiled to a single IIFE bundle |
| Registry source | GitHub (plain JSON file, PR-based submissions) |
| App distribution | GitHub Releases (tarball/zip assets) |
| App-scoped storage | SQLite via `tauri-plugin-sql` |
| HTTP client (Rust) | `reqwest` |
| Archive extraction | `flate2` + `tar` crates |

---

## Security Model

- **No shell access** in Phase 1. Shell permission does not exist in the permission enum.
- **No arbitrary filesystem access.** `filesystem:read` and `filesystem:write` exclusively operate through OS file/save dialogs — the app never receives a raw path.
- **Network isolation by default.** Without `network` or `network:domain:*` permission, the WebView has no outbound fetch capability.
- **Capability file is the enforcement boundary.** Permissions are enforced at the Tauri IPC layer (Rust), not only at the SDK layer. Even if a malicious app bypasses the AppBridge, the Tauri ACL denies the IPC call.
- **Cross-app data isolation.** The Rust storage handler prefixes every key with the calling app's ID, which is resolved from the WebviewWindow label — not from anything the app can spoof.
- **Verified badge.** The core team manually reviews apps before marking `"verified": true` in the registry. Unverified apps display a clear warning at install time.

---

## Phase Roadmap

**Phase 1 — Core Runtime**
- Host app scaffold (Tauri v2 + SvelteKit shell)
- Registry fetch and Marketplace UI
- Install flow (GitHub Releases download + extraction)
- Permission engine (declaration → prompt → capability generation)
- AppBridge SDK (`storage`, `notifications`, `clipboard`, `audio:play`)
- App launch (isolated WebviewWindow + SDK injection)

**Phase 2 — Developer Experience**
- `lugios-cli` scaffold tool (`lugios init my-app`)
- Local development mode (`lugios dev` — runs app in LugiOS without publishing)
- App update mechanism (check latest release tag vs installed version)
- `filesystem:read` and `filesystem:write` permissions
- `network` and `network:domain:*` permissions

**Phase 3 — Ecosystem**
- Verified badge review process
- App rating and comments (GitHub Discussions integration)
- Registry GitHub Actions CI (manifest lint, duplicate ID check)
- Cross-platform packaging (Windows installer, macOS .dmg, Linux AppImage)
- `lugios-sdk` npm package (for type-safe development against AppBridge API)
