<script lang="ts">
  import Icon from "@iconify/svelte";
  import InstalledAppRow from "$lib/components/InstalledAppRow.svelte";
  import {
    appUpdateStatuses,
    checkAppUpdates,
    installedApps,
    installedError,
    installedLoading,
    launchApp,
    loadInstalled,
    uninstallApp,
    upgradeApp,
    updatesCheckError,
    updatesCheckLoading,
  } from "$lib/stores/installed";
  import { mdiPackageVariant, mdiRefresh } from "$lib/iconData";
  import { invokeErrorMessage } from "$lib/invokeError";
  import { showToast } from "$lib/stores/toast";
  import { onMount } from "svelte";

  let busyId = $state<string | null>(null);
  let actionErr = $state<string | null>(null);

  onMount(() => {
    void loadInstalled().then(() => checkAppUpdates());
  });

  async function recheckUpdates() {
    await checkAppUpdates();
  }

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

  async function upgrade(id: string, displayName: string) {
    actionErr = null;
    busyId = id;
    try {
      await upgradeApp(id);
      showToast(`${displayName} was updated to the latest release.`, "success");
    } catch (e) {
      actionErr = invokeErrorMessage(e);
    } finally {
      busyId = null;
    }
  }
</script>

<div class="flex flex-col gap-6">
  <div class="flex flex-wrap items-end justify-between gap-4">
    <div>
      <h1 class="flex items-center gap-2 text-2xl font-semibold text-white">
        <Icon icon={mdiPackageVariant} class="size-7 text-lugos-accent" />
        Installed apps
      </h1>
      <p class="mt-1 text-sm text-lugos-muted">
        Launch apps in an isolated window, update from GitHub releases, or remove them from disk.
      </p>
    </div>
    <button
      type="button"
      class="app-no-drag inline-flex items-center gap-2 rounded-lg border border-lugos-border bg-lugos-surface px-3 py-2 text-sm font-medium text-slate-200 hover:bg-white/5 disabled:opacity-50"
      disabled={$updatesCheckLoading || busyId !== null}
      onclick={recheckUpdates}
    >
      <Icon
        icon={mdiRefresh}
        class={$updatesCheckLoading ? "size-4 animate-spin" : "size-4"}
        aria-hidden="true"
      />
      {$updatesCheckLoading ? "Checking…" : "Check for updates"}
    </button>
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

  {#if $updatesCheckError}
    <p class="rounded-lg border border-amber-900/40 bg-amber-950/30 p-4 text-sm text-amber-100">
      {$updatesCheckError}
    </p>
  {/if}

  {#if $installedLoading && $installedApps.length === 0}
    <p class="text-lugos-muted text-sm">Loading…</p>
  {:else if $installedApps.length === 0}
    <p class="flex items-start gap-2 text-sm text-lugos-muted">
      <Icon
        icon={mdiPackageVariant}
        class="mt-0.5 size-4 shrink-0 text-slate-500"
      />
      Nothing installed yet. Browse the Marketplace to add an app.
    </p>
  {:else}
    <ul class="space-y-3">
      {#each $installedApps as app (app.id)}
        <InstalledAppRow
          {app}
          update={$appUpdateStatuses.find((u) => u.appId === app.id)}
          busy={busyId !== null}
          onOpen={() => open(app.id)}
          onRemove={() => remove(app.id)}
          onUpgrade={() => upgrade(app.id, app.displayName)}
        />
      {/each}
    </ul>
  {/if}
</div>
