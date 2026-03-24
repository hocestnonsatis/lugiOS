<script lang="ts">
  import Icon from "@iconify/svelte";
  import { mdiDownloadOutline, mdiPlay, mdiTrashCanOutline } from "$lib/iconData";
  import type { AppManifest, AppUpdateStatus } from "$lib/types";

  interface Props {
    app: AppManifest;
    update: AppUpdateStatus | undefined;
    busy: boolean;
    onOpen: () => void;
    onRemove: () => void;
    onUpgrade: () => void;
  }

  let { app, update, busy, onOpen, onRemove, onUpgrade }: Props = $props();

  const versionLine = $derived.by(() => {
    const base = `v${app.version}`;
    if (!update) return `${base} · ${app.id}`;
    if (update.checkError) return `${base} · ${update.checkError}`;
    if (update.latestVersion) return `${base} → latest ${update.latestVersion}`;
    return `${base} · ${app.id}`;
  });
</script>

<li
  class="flex flex-wrap items-center justify-between gap-3 rounded-xl border border-lugos-border bg-lugos-surface px-4 py-3"
>
  <div class="min-w-0 flex-1">
    <p class="font-medium text-white">{app.displayName}</p>
    <p class="text-xs text-lugos-muted">{versionLine}</p>
  </div>
  <div class="flex flex-wrap gap-2">
    {#if update?.updateAvailable}
      <button
        type="button"
        class="inline-flex items-center justify-center gap-1.5 rounded-lg border border-lugos-accent/40 bg-lugos-accent/10 px-3 py-1.5 text-sm font-medium text-lugos-accent hover:bg-lugos-accent/20 disabled:opacity-50"
        disabled={busy}
        onclick={onUpgrade}
      >
        <Icon icon={mdiDownloadOutline} class="size-4 shrink-0" />
        Update
      </button>
    {/if}
    <button
      type="button"
      class="inline-flex items-center justify-center gap-1.5 rounded-lg bg-lugos-accent px-3 py-1.5 text-sm font-medium text-white hover:bg-blue-600 disabled:opacity-50"
      disabled={busy}
      onclick={onOpen}
    >
      <Icon icon={mdiPlay} class="size-4 shrink-0" />
      Open
    </button>
    <button
      type="button"
      class="inline-flex items-center justify-center gap-1.5 rounded-lg border border-red-900/40 px-3 py-1.5 text-sm text-red-200 hover:bg-red-950/50 disabled:opacity-50"
      disabled={busy}
      onclick={onRemove}
    >
      <Icon icon={mdiTrashCanOutline} class="size-4 shrink-0" />
      Uninstall
    </button>
  </div>
</li>
