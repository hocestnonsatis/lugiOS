import { appIdFromEnv, requireGrant } from "./grants.js";
import { lugosInvoke } from "./invoke.js";

export async function send(opts: {
  title: string;
  body?: string;
}): Promise<void> {
  requireGrant("notifications");
  await lugosInvoke("lugos_notification_send", {
    app_id: appIdFromEnv(),
    title: opts.title,
    body: opts.body ?? null,
  });
}
