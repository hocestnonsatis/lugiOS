<script lang="ts">
  import {
    installedApps,
    installedError,
    installedLoading,
    launchApp,
    loadInstalled,
    uninstallApp,
  } from "$lib/stores/installed";
  import { invokeErrorMessage } from "$lib/invokeError";
  import { onMount } from "svelte";

  let busyId = $state<string | null>(null);
  let actionErr = $state<string | null>(null);

  onMount(() => loadInstalled());

  async function open(id: string) {
    actionErr = null;
    busyId = id;
    try {
      await launchApp(id);
    } catch (e) {
      actionErr = invokeErrorMessage(e);
    } finally {
      busyId = null;
    }
  }

  async function remove(id: string) {
    actionErr = null;
    busyId = id;
    try {
      await uninstallApp(id);
    } catch (e) {
      actionErr = invokeErrorMessage(e);
    } finally {
      busyId = null;
    }
  }
</script>

<div class="flex flex-col gap-6">
  <div>
    <h1 class="text-2xl font-semibold text-white">Installed apps</h1>
    <p class="mt-1 text-sm text-lugos-muted">
      Launch apps in an isolated window or remove them from disk.
    </p>
  </div>

  {#if actionErr}
    <p class="rounded-lg border border-red-900/50 bg-red-950/40 p-4 text-sm text-red-200">
      {actionErr}
    </p>
  {/if}

  {#if $installedError}
    <p class="rounded-lg border border-red-900/50 bg-red-950/40 p-4 text-sm text-red-200">
      {$installedError}
    </p>
  {/if}

  {#if $installedLoading && $installedApps.length === 0}
    <p class="text-lugos-muted text-sm">Loading…</p>
  {:else if $installedApps.length === 0}
    <p class="text-lugos-muted text-sm">
      Nothing installed yet. Browse the Marketplace to add an app.
    </p>
  {:else}
    <ul class="space-y-3">
      {#each $installedApps as app (app.id)}
        <li
          class="flex flex-wrap items-center justify-between gap-3 rounded-xl border border-lugos-border bg-lugos-surface px-4 py-3"
        >
          <div>
            <p class="font-medium text-white">{app.displayName}</p>
            <p class="text-xs text-lugos-muted">v{app.version} · {app.id}</p>
          </div>
          <div class="flex gap-2">
            <button
              type="button"
              class="rounded-lg bg-lugos-accent px-3 py-1.5 text-sm font-medium text-white hover:bg-blue-600 disabled:opacity-50"
              disabled={busyId !== null}
              onclick={() => open(app.id)}
            >
              Open
            </button>
            <button
              type="button"
              class="rounded-lg border border-red-900/40 px-3 py-1.5 text-sm text-red-200 hover:bg-red-950/50 disabled:opacity-50"
              disabled={busyId !== null}
              onclick={() => remove(app.id)}
            >
              Uninstall
            </button>
          </div>
        </li>
      {/each}
    </ul>
  {/if}
</div>
