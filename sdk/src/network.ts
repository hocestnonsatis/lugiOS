import { appIdFromEnv, networkAllowedForUrl } from "./grants.js";
import { PermissionDeniedError } from "./errors.js";
import { lugosInvoke } from "./invoke.js";

export async function lugosFetch(
  url: string,
  init?: RequestInit,
): Promise<Response> {
  if (!networkAllowedForUrl(url)) {
    throw new PermissionDeniedError("network");
  }
  const method = init?.method ?? "GET";
  let body: string | null = null;
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
    body,
  });
  const u8 = new Uint8Array(bytes as number[]);
  return new Response(u8, { status: 200 });
}
