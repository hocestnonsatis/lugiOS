# LugiOS smoke test checklist

Run these after meaningful changes to the host shell, installer, or AppBridge.

## Build

- [ ] `cd src-tauri && cargo build` succeeds (no errors).
- [ ] `cd src-tauri && cargo clippy -- -D warnings` is clean.
- [ ] `npm run check` passes (Svelte / TypeScript).
- [ ] `npm run tauri build` completes. For signed host updates, enable `bundle.createUpdaterArtifacts` and set `TAURI_SIGNING_PRIVATE_KEY_PATH` (see README).

## Registry and marketplace

- [ ] Marketplace loads entries (network or embedded fallback).
- [ ] Search filters cards; refresh updates the list.
- [ ] Installed apps show **Installed · v…** on list and detail views.
- [ ] Settings: custom registry URL saves, **Save and refresh** updates the catalog; **Use default URL** restores the built-in list.

## Install and permissions

- [ ] Install a demo app (e.g. `demo-hello`, `demo-notes`) from the registry; permission dialog matches manifest.
- [ ] App appears under **Installed**; **Open** launches an isolated window.

## Mini-app updates

- [ ] **Check for updates** on Installed reports current versions (or rate-limit message).
- [ ] After `npm run simulate:demo-outdated`, an update is offered when the GitHub tag is newer; **Update** completes and a success toast appears.

## Host self-update (optional)

- [ ] Settings → **Check for LugiOS updates** runs without crashing. With no published `latest.json` / signed assets, an explanatory error is acceptable.

## AppBridge (mini-app webview)

- [ ] With `storage` granted, `AppBridge.storage.set` / `get` persists across launches.
- [ ] Calling a method without the required permission throws `PermissionDeniedError`.
