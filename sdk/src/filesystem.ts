import { appIdFromEnv, requireGrant } from "./grants.js";
import { lugosInvoke } from "./invoke.js";

export async function pickAndRead(opts: {
  accept?: string[];
}): Promise<{ name: string; content: Uint8Array } | null> {
  requireGrant("filesystem:read");
  void opts;
  const raw = await lugosInvoke("lugos_fs_pick_read", {
    app_id: appIdFromEnv(),
  });
  if (!raw) return null;
  const o = raw as { name: string; content: number[] };
  return {
    name: o.name,
    content: new Uint8Array(o.content),
  };
}

export async function saveDialog(opts: {
  defaultName?: string;
  content: Uint8Array;
}): Promise<void> {
  requireGrant("filesystem:write");
  await lugosInvoke("lugos_fs_save_dialog", {
    app_id: appIdFromEnv(),
    default_name: opts.defaultName ?? null,
    content: [...opts.content],
  });
}
