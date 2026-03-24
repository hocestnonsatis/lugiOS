# LugiOS

**LugiOS** is a desktop runtime and marketplace for sandboxed web mini-apps. Developers distribute apps through GitHub; users discover them in a built-in marketplace and run each app in its own Tauri **WebviewWindow**, with permissions enforced by a host-managed **AppBridge** SDK.

Stack: **Tauri v2** (Rust), **SvelteKit** + **TypeScript** (host UI), **Tailwind CSS**, and a small **esbuild**-bundled SDK under `sdk/` for app authors.

---

## Features

- **Marketplace** — Fetches a JSON registry (`registry/registry.json`), caches it, and supports refresh.
- **Install & run** — Downloads release archives from GitHub, extracts them, validates `app.manifest.json`, persists user-selected **grants**, and generates per-app Tauri capabilities at runtime.
- **AppBridge** — Injected into each mini-app webview (`window.AppBridge`) for storage, notifications, clipboard, filesystem dialogs, gated `fetch`, and audio, according to granted permissions.
- **Offline-safe registry** — If the remote registry cannot be reached, the host falls back to an embedded copy bundled at build time.

---

## Documentation

| Document | Purpose |
|----------|---------|
| [ARCHITECTURE.md](./ARCHITECTURE.md) | System design, data flow, and on-disk layout |
| [AGENTS.md](./AGENTS.md) | Task breakdown and rules for contributors / coding agents |
| [docs/SMOKE_TEST.md](./docs/SMOKE_TEST.md) | Manual regression checklist before releases |

---

## Prerequisites

- [Node.js](https://nodejs.org/) (LTS recommended)
- [Rust](https://www.rust-lang.org/tools/install) stable toolchain
- Platform-specific Tauri dependencies ([prerequisites](https://v2.tauri.app/start/prerequisites/))

---

## Quick start

Clone the repository and install dependencies (the `postinstall` script installs the SDK package):

```bash
git clone https://github.com/hocestnonsatis/lugiOS.git
cd lugiOS
npm install
```

**Development** (SvelteKit dev server + Tauri window):

```bash
npm run tauri dev
```

**Production frontend + desktop build**:

```bash
npm run tauri build
```

Other useful scripts:

| Command | Description |
|---------|-------------|
| `npm run dev` | SvelteKit/Vite only (no Tauri shell) |
| `npm run build` | Build SDK + static frontend into `build/` |
| `npm run build:sdk` | Rebuild `sdk/` → `src-tauri/resources/bridge.iife.js` |
| `npm run check` | Type-check Svelte and TypeScript |
| `npm run icons` | Regenerate outlined logos in `branding/`, platform icons under `src-tauri/icons/` ([Tauri icon](https://v2.tauri.app/develop/icons/)), and `static/favicon.png` |
| `npm run package:demo-hello` | Zip `fixtures/demo-hello-app/` → `fixtures/demo-hello-dist.zip` |
| `npm run package:demo-notes` | Zip `fixtures/demo-notes-app/` → `fixtures/demo-notes-dist.zip` |
| `npm run package:fixtures` | Build both zip files |
| `npm run simulate:demo-outdated` | Lowers installed demo app manifest versions under the app data dir so you can verify mini-app **Update** (see script header) |

**Install smoke test:** Registry entries **`demo-hello`** and **`demo-notes`** install from [`lugi-demo-hello`](https://github.com/hocestnonsatis/lugi-demo-hello/releases) and [`lugi-demo-notes`](https://github.com/hocestnonsatis/lugi-demo-notes/releases). After installing, open the app from **Installed**.

A longer manual checklist lives in [`docs/SMOKE_TEST.md`](./docs/SMOKE_TEST.md).

If the marketplace still shows old repo URLs or you see 404s on GitHub API after updating `registry/registry.json`, click **Refresh** on the Marketplace or delete `%APPDATA%\\com.hocestnonsatis.lugios\\registry_cache.json`, then restart the app. Rebuild the host (`npm run tauri dev` / `tauri build`) so the **embedded** registry (compile-time `include_str!`) matches your JSON.

---

## Registry URL

By default the host downloads the registry from:

`https://raw.githubusercontent.com/hocestnonsatis/lugiOS/main/registry/registry.json`

**In the app:** **Settings → Marketplace registry** lets you save a custom HTTPS URL (stored under the app data directory). **Refresh** or **Save and refresh** applies it immediately.

Override for a single process with:

```bash
# PowerShell example
$env:LUGIOS_REGISTRY_URL = "https://raw.githubusercontent.com/OWNER/REPO/BRANCH/path/registry.json"
npm run tauri dev
```

If set, the environment variable wins over the saved URL until you unset it and restart.

The canonical list in this repo lives at [`registry/registry.json`](./registry/registry.json).

---

## Host application updates (Tauri updater)

The desktop host can check **GitHub Releases** for a signed update manifest (`latest.json`). Configure endpoints and the **public** signing key in [`src-tauri/tauri.conf.json`](./src-tauri/tauri.conf.json). Keep the **private** key out of git (see `.gitignore` for `src-tauri/.tauri/*.key`).

For **signed updater bundles**, set `bundle.createUpdaterArtifacts` to `true` and provide `TAURI_SIGNING_PRIVATE_KEY` or `TAURI_SIGNING_PRIVATE_KEY_PATH` when running `tauri build`. The repository defaults to `createUpdaterArtifacts: false` so `npm run tauri build` succeeds without signing keys.

Use **Settings → LugiOS updates → Check for LugiOS updates** to run a check from the UI.

---

## Repository layout

```
lugiOS/
├── branding/         # LugiOS wordmark + app icon SVG (outlined paths; optional font file for regeneration)
├── fixtures/         # demo-hello-app + demo-notes-app sources; `npm run package:fixtures` → *-dist.zip
├── src/              # Host shell (SvelteKit)
├── src-tauri/        # Tauri v2 backend (Rust)
├── sdk/              # AppBridge SDK source (bundled into the host)
└── registry/         # App catalog JSON consumed by the host
```

---

## Contributing

Read [AGENTS.md](./AGENTS.md) for module boundaries and conventions before opening large changes.

---

## License

MIT — see [package.json](./package.json) `license` field.
