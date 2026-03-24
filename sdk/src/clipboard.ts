import { appIdFromEnv, requireGrant } from "./grants.js";
import { lugosInvoke } from "./invoke.js";

export async function read(): Promise<string> {
  requireGrant("clipboard:read");
  return lugosInvoke("lugos_clipboard_read", {
    app_id: appIdFromEnv(),
  }) as Promise<string>;
}

export async function write(text: string): Promise<void> {
  requireGrant("clipboard:write");
  await lugosInvoke("lugos_clipboard_write", {
    app_id: appIdFromEnv(),
    text,
  });
}
