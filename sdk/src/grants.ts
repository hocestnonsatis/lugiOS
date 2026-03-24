import { PermissionDeniedError } from "./errors.js";

export function grantSet(): Set<string> {
  const g = (window as unknown as { __LUGOS_GRANTS__?: readonly string[] })
    .__LUGOS_GRANTS__;
  if (!g?.length) return new Set();
  return new Set([...g]);
}

export function requireGrant(token: string): void {
  if (!grantSet().has(token)) {
    throw new PermissionDeniedError(token);
  }
}

export function appIdFromEnv(): string {
  const id = (window as unknown as { __LUGOS_APP_ID__?: string }).__LUGOS_APP_ID__;
  if (!id) throw new Error("LugiOS: __LUGOS_APP_ID__ not set");
  return id;
}

export function networkAllowedForUrl(url: string): boolean {
  const s = grantSet();
  if (s.has("network")) return true;
  let u: URL;
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
