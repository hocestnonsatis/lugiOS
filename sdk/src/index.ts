import * as audio from "./audio.js";
import * as clipboard from "./clipboard.js";
import { PermissionDeniedError, LugiOSError } from "./errors.js";
import * as fsMod from "./filesystem.js";
import { grantSet } from "./grants.js";
import * as notifications from "./notifications.js";
import * as network from "./network.js";
import * as storage from "./storage.js";

function deny(perm: string) {
  return async (): Promise<never> => {
    throw new PermissionDeniedError(perm);
  };
}

type AppBridgeRoot = Record<string, unknown>;

function buildBridge(): AppBridgeRoot {
  const g = grantSet();
  const b: AppBridgeRoot = {};

  if ([...g].some((p) => p === "storage")) {
    b.storage = Object.freeze({
      get: storage.get,
      set: storage.set,
      del: storage.del,
    });
  }

  if (g.has("notifications")) {
    b.notifications = Object.freeze({
      send: notifications.send,
    });
  }

  if (g.has("clipboard:read") || g.has("clipboard:write")) {
    b.clipboard = Object.freeze({
      read: g.has("clipboard:read") ? clipboard.read : deny("clipboard:read"),
      write: g.has("clipboard:write") ? clipboard.write : deny("clipboard:write"),
    });
  }

  if (g.has("filesystem:read") || g.has("filesystem:write")) {
    b.fs = Object.freeze({
      pickAndRead: g.has("filesystem:read")
        ? fsMod.pickAndRead
        : deny("filesystem:read"),
      saveDialog: g.has("filesystem:write")
        ? fsMod.saveDialog
        : deny("filesystem:write"),
    });
  }

  if ([...g].some((p) => p === "network" || p.startsWith("network:domain:"))) {
    b.network = Object.freeze({
      fetch: network.lugosFetch,
    });
  }

  if (g.has("audio:play")) {
    b.audio = Object.freeze({
      play: audio.play,
      loadAsset: audio.loadAsset,
    });
  }

  return Object.freeze(b);
}

const bridge = buildBridge();
(window as unknown as { AppBridge: unknown }).AppBridge = Object.freeze(bridge);

export type { PermissionDeniedError, LugiOSError };
export { bridge as AppBridge };
