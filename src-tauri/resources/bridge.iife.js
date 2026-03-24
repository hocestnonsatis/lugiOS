"use strict";
(() => {
  // src/errors.ts
  var PermissionDeniedError = class extends Error {
    constructor(permission) {
      super(`Permission denied: ${permission}`);
      this.name = "PermissionDeniedError";
    }
  };

  // src/grants.ts
  function grantSet() {
    const g = window.__LUGOS_GRANTS__;
    if (!g?.length) return /* @__PURE__ */ new Set();
    return /* @__PURE__ */ new Set([...g]);
  }
  function requireGrant(token) {
    if (!grantSet().has(token)) {
      throw new PermissionDeniedError(token);
    }
  }
  function appIdFromEnv() {
    const id = window.__LUGOS_APP_ID__;
    if (!id) throw new Error("LugiOS: __LUGOS_APP_ID__ not set");
    return id;
  }
  function networkAllowedForUrl(url) {
    const s = grantSet();
    if (s.has("network")) return true;
    let u;
    try {
      u = new URL(url);
    } catch {
      return false;
    }
    const host = u.hostname;
    for (const p of s) {
      if (p.startsWith("network:domain:")) {
        const h = p.slice("network:domain:".length);
        if (host === h || host === `www.${h}`) return true;
      }
    }
    return false;
  }

  // src/invoke.ts
  function lugosInvoke(cmd, args = {}) {
    const w = window;
    const i = w.__TAURI_INTERNALS__;
    if (!i?.invoke) {
      throw new Error("LugiOS AppBridge requires a Tauri webview (Tauri internals missing)");
    }
    return i.invoke(cmd, args);
  }

  // src/audio.ts
  async function loadAsset(assetPath) {
    requireGrant("audio:play");
    const bytes = await lugosInvoke("lugos_audio_read_asset", {
      app_id: appIdFromEnv(),
      asset_path: assetPath
    });
    return new Uint8Array(bytes).buffer;
  }
  async function play(assetPath) {
    const buf = await loadAsset(assetPath);
    const ctx = new AudioContext();
    const audioBuf = await ctx.decodeAudioData(buf.slice(0));
    const src = ctx.createBufferSource();
    src.buffer = audioBuf;
    src.connect(ctx.destination);
    src.start();
  }

  // src/clipboard.ts
  async function read() {
    requireGrant("clipboard:read");
    return lugosInvoke("lugos_clipboard_read", {
      app_id: appIdFromEnv()
    });
  }
  async function write(text) {
    requireGrant("clipboard:write");
    await lugosInvoke("lugos_clipboard_write", {
      app_id: appIdFromEnv(),
      text
    });
  }

  // src/filesystem.ts
  async function pickAndRead(opts) {
    requireGrant("filesystem:read");
    void opts;
    const raw = await lugosInvoke("lugos_fs_pick_read", {
      app_id: appIdFromEnv()
    });
    if (!raw) return null;
    const o = raw;
    return {
      name: o.name,
      content: new Uint8Array(o.content)
    };
  }
  async function saveDialog(opts) {
    requireGrant("filesystem:write");
    await lugosInvoke("lugos_fs_save_dialog", {
      app_id: appIdFromEnv(),
      default_name: opts.defaultName ?? null,
      content: [...opts.content]
    });
  }

  // src/notifications.ts
  async function send(opts) {
    requireGrant("notifications");
    await lugosInvoke("lugos_notification_send", {
      app_id: appIdFromEnv(),
      title: opts.title,
      body: opts.body ?? null
    });
  }

  // src/network.ts
  async function lugosFetch(url, init) {
    if (!networkAllowedForUrl(url)) {
      throw new PermissionDeniedError("network");
    }
    const method = init?.method ?? "GET";
    let body = null;
    if (init?.body != null) {
      if (typeof init.body === "string") body = init.body;
      else if (init.body instanceof ArrayBuffer) {
        body = new TextDecoder().decode(init.body);
      } else {
        throw new Error("LugiOS fetch: only string or ArrayBuffer body supported");
      }
    }
    const bytes = await lugosInvoke("lugos_fetch", {
      app_id: appIdFromEnv(),
      url,
      method,
      body
    });
    const u8 = new Uint8Array(bytes);
    return new Response(u8, { status: 200 });
  }

  // src/storage.ts
  async function get(key) {
    requireGrant("storage");
    const v = await lugosInvoke("storage_get", {
      app_id: appIdFromEnv(),
      key
    });
    return v ?? null;
  }
  async function set(key, value) {
    requireGrant("storage");
    await lugosInvoke("storage_set", {
      app_id: appIdFromEnv(),
      key,
      value
    });
  }
  async function del(key) {
    requireGrant("storage");
    await lugosInvoke("storage_delete", {
      app_id: appIdFromEnv(),
      key
    });
  }

  // src/index.ts
  function deny(perm) {
    return async () => {
      throw new PermissionDeniedError(perm);
    };
  }
  function buildBridge() {
    const g = grantSet();
    const b = {};
    if ([...g].some((p) => p === "storage")) {
      b.storage = Object.freeze({
        get,
        set,
        del
      });
    }
    if (g.has("notifications")) {
      b.notifications = Object.freeze({
        send
      });
    }
    if (g.has("clipboard:read") || g.has("clipboard:write")) {
      b.clipboard = Object.freeze({
        read: g.has("clipboard:read") ? read : deny("clipboard:read"),
        write: g.has("clipboard:write") ? write : deny("clipboard:write")
      });
    }
    if (g.has("filesystem:read") || g.has("filesystem:write")) {
      b.fs = Object.freeze({
        pickAndRead: g.has("filesystem:read") ? pickAndRead : deny("filesystem:read"),
        saveDialog: g.has("filesystem:write") ? saveDialog : deny("filesystem:write")
      });
    }
    if ([...g].some((p) => p === "network" || p.startsWith("network:domain:"))) {
      b.network = Object.freeze({
        fetch: lugosFetch
      });
    }
    if (g.has("audio:play")) {
      b.audio = Object.freeze({
        play,
        loadAsset
      });
    }
    return Object.freeze(b);
  }
  var bridge = buildBridge();
  window.AppBridge = Object.freeze(bridge);
})();
