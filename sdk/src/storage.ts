import { requireGrant, appIdFromEnv } from "./grants.js";
import { lugosInvoke } from "./invoke.js";

export async function get(key: string): Promise<string | null> {
  requireGrant("storage");
  const v = await lugosInvoke("storage_get", {
    app_id: appIdFromEnv(),
    key,
  });
  return (v as string | null) ?? null;
}

export async function set(key: string, value: string): Promise<void> {
  requireGrant("storage");
  await lugosInvoke("storage_set", {
    app_id: appIdFromEnv(),
    key,
    value,
  });
}

export async function del(key: string): Promise<void> {
  requireGrant("storage");
  await lugosInvoke("storage_delete", {
    app_id: appIdFromEnv(),
    key,
  });
}
