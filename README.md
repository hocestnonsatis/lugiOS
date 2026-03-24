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

---

## Registry URL

By default the host downloads the registry from:

`https://raw.githubusercontent.com/hocestnonsatis/lugiOS/main/registry/registry.json`

Override at runtime with:

```bash
# PowerShell example
$env:LUGIOS_REGISTRY_URL = "https://raw.githubusercontent.com/OWNER/REPO/BRANCH/path/registry.json"
npm run tauri dev
```

The canonical list in this repo lives at [`registry/registry.json`](./registry/registry.json).

---

## Repository layout

```
lugiOS/
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
