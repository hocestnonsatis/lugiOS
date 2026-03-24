<script lang="ts">
  import Icon from "@iconify/svelte";
  import ListingAppLogo from "$lib/components/ListingAppLogo.svelte";
  import PermissionDialog from "$lib/components/PermissionDialog.svelte";
  import {
    mdiChevronRight,
    mdiClose,
    mdiCloseCircleOutline,
    mdiDownloadOutline,
    mdiMagnify,
    mdiPlay,
    mdiRefresh,
  } from "$lib/iconData";
  import {
    initRegistry,
    refreshRegistry,
    registryEntries,
    registryError,
    registryLoading,
  } from "$lib/stores/registry";
  import {
    installApp,
    launchApp,
    loadInstalled,
    installedApps,
  } from "$lib/stores/installed";
  import type { AppManifest, RegistryEntry } from "$lib/types";
  import { afterNavigate } from "$app/navigation";
  import { invoke } from "@tauri-apps/api/core";
  import { invokeErrorMessage } from "$lib/invokeError";
  import { onMount } from "svelte";

  let dialogOpen = $state(false);
  let activeEntry = $state<RegistryEntry | null>(null);
  let preview = $state<AppManifest | null>(null);
  let previewError = $state<string | null>(null);
  let installBusy = $state(false);
  let installErr = $state<string | null>(null);
  let launchErr = $state<string | null>(null);
  let searchQuery = $state("");

  const filteredEntries = $derived.by(() => {
    const q = searchQuery.trim().toLowerCase();
    if (!q) return $registryEntries;
    return $registryEntries.filter((entry) => {
      const haystack = [
        entry.id,
        entry.displayName,
        entry.description,
        entry.author,
        entry.repo,
        ...entry.tags,
      ]
        .join(" ")
        .toLowerCase();
      return haystack.includes(q);
    });
  });

  onMount(() => {
    initRegistry();
    void loadInstalled();
  });

  afterNavigate((n) => {
    const path = n.to?.url.pathname ?? "";
    if (path.startsWith("/marketplace")) {
      void loadInstalled();
    }
  });

  async function onInstall(entry: RegistryEntry) {
    activeEntry = entry;
    preview = null;
    previewError = null;
    installErr = null;
    dialogOpen = false;
    try {
      preview = await invoke<AppManifest>("preview_app_manifest", {
        repoUrl: entry.repo,
      });
      dialogOpen = true;
    } catch (e) {
      previewError = invokeErrorMessage(e);
    }
  }

  function closeDialog() {
    dialogOpen = false;
    activeEntry = null;
    preview = null;
    previewError = null;
  }

  function closeError() {
    previewError = null;
    activeEntry = null;
  }

  async function confirmInstall(grants: string[]) {
    if (!activeEntry) return;
    installBusy = true;
    installErr = null;
    try {
      await installApp(activeEntry.id, activeEntry.repo, grants);
      closeDialog();
    } catch (e) {
      installErr = invokeErrorMessage(e);
    } finally {
      installBusy = false;
    }
  }

  async function onOpen(appId: string) {
    launchErr = null;
    try {
      await launchApp(appId);
    } catch (e) {
      launchErr = invokeErrorMessage(e);
    }
  }
</script>

<div class="flex flex-col gap-6">
  <div class="flex flex-wrap items-center gap-2">
    <label class="sr-only" for="marketplace-search">Search apps</label>
    <div class="relative min-w-0 flex-1 basis-full sm:basis-auto sm:max-w-xl">
      <Icon
        icon={mdiMagnify}
        class="pointer-events-none absolute left-3 top-1/2 size-4 -translate-y-1/2 text-slate-500"
        aria-hidden="true"
      />
      <input
        id="marketplace-search"
        type="search"
        autocomplete="off"
        placeholder="Search by name, id, author, repo, or tag…"
        bind:value={searchQuery}
        class="app-no-drag h-10 w-full rounded-lg border border-lugos-border bg-lugos-bg py-2 pl-9 pr-3 text-sm text-slate-100 placeholder:text-slate-500 focus:border-lugos-accent focus:outline-none focus:ring-1 focus:ring-lugos-accent"
      />
    </div>
    <button
      type="button"
      class="app-no-drag inline-flex size-10 shrink-0 items-center justify-center rounded-lg border border-lugos-border text-slate-200 hover:bg-white/5 disabled:opacity-50"
      disabled={$registryLoading}
      aria-label="Refresh registry"
      title="Refresh registry"
      onclick={() => refreshRegistry()}
    >
      <Icon
        icon={mdiRefresh}
        class={$registryLoading ? "size-[18px] animate-spin" : "size-[18px]"}
        aria-hidden="true"
      />
    </button>
    {#if searchQuery.trim()}
      <button
        type="button"
        class="app-no-drag inline-flex h-10 shrink-0 items-center justify-center gap-2 rounded-lg border border-lugos-border px-3 text-sm text-slate-200 hover:bg-white/5"
        onclick={() => (searchQuery = "")}
      >
        <Icon icon={mdiCloseCircleOutline} class="size-4 shrink-0" />
        Clear
      </button>
    {/if}
  </div>

  {#if $registryError}
    <p class="rounded-lg border border-red-900/50 bg-red-950/40 p-4 text-sm text-red-200">
      {$registryError}
    </p>
  {/if}

  {#if installErr}
    <p class="rounded-lg border border-red-900/50 bg-red-950/40 p-4 text-sm text-red-200">
      {installErr}
    </p>
  {/if}

  {#if launchErr}
    <p class="rounded-lg border border-red-900/50 bg-red-950/40 p-4 text-sm text-red-200">
      {launchErr}
    </p>
  {/if}

  {#if $registryLoading && $registryEntries.length === 0}
    <p class="text-lugos-muted text-sm">Loading registry…</p>
  {:else if $registryEntries.length === 0}
    <p class="text-lugos-muted text-sm">
      No entries yet. Check your network or whether the registry URL is
      reachable.
    </p>
  {:else if filteredEntries.length === 0}
    <p class="text-lugos-muted text-sm">
      No apps match “{searchQuery.trim()}”. Try a different term or
      <button
        type="button"
        class="text-lugos-accent underline decoration-lugos-accent/40 underline-offset-2 hover:decoration-lugos-accent"
        onclick={() => (searchQuery = "")}
      >
        clear search
      </button>.
    </p>
  {:else}
    <ul class="grid gap-4 sm:grid-cols-2">
      {#each filteredEntries as entry (entry.id)}
        {@const installedManifest = $installedApps.find((a) => a.id === entry.id)}
        {@const installed = installedManifest !== undefined}
        <li
          class="flex h-full min-h-0 flex-col overflow-hidden rounded-xl border border-lugos-border bg-lugos-surface"
        >
          <a
            href="/marketplace/{entry.id}"
            class="app-no-drag flex min-h-0 flex-1 flex-col p-5 pb-4 text-left outline-none transition-colors hover:bg-white/[0.04] focus-visible:ring-2 focus-visible:ring-inset focus-visible:ring-lugos-accent"
          >
            <div class="flex min-h-0 flex-1 gap-4">
              <ListingAppLogo {entry} />
              <div class="flex min-h-0 min-w-0 flex-1 flex-col">
                <div class="flex items-start justify-between gap-2">
                  <div class="min-w-0">
                    <h2 class="font-semibold text-white">{entry.displayName}</h2>
                    {#if installedManifest}
                      <p class="mt-1 text-xs font-medium text-emerald-300/90">
                        Installed · v{installedManifest.version}
                      </p>
                    {/if}
                  </div>
                  {#if entry.verified}
                    <span
                      class="shrink-0 rounded bg-emerald-500/20 px-2 py-0.5 text-xs font-medium text-emerald-300"
                      >Verified</span
                    >
                  {:else}
                    <span
                      class="shrink-0 rounded bg-amber-500/15 px-2 py-0.5 text-xs text-amber-200"
                      >Unverified</span
                    >
                  {/if}
                </div>
                <p class="mt-2 line-clamp-3 text-sm text-lugos-muted">
                  {entry.description}
                </p>
                <p class="mt-2 text-xs text-slate-500">
                  by {entry.author} · {entry.repo}
                </p>
                <div class="mt-3 flex flex-wrap gap-1">
                  {#each entry.tags as tag (tag)}
                    <span
                      class="rounded bg-lugos-bg px-2 py-0.5 text-xs text-slate-400"
                      >{tag}</span
                    >
                  {/each}
                </div>
                <span
                  class="mt-3 inline-flex items-center gap-0.5 text-xs font-medium text-blue-400"
                >
                  Details & GitHub stats
                  <Icon icon={mdiChevronRight} class="size-3.5 shrink-0" />
                </span>
              </div>
            </div>
          </a>
          <div
            class="shrink-0 border-t border-lugos-border/60 bg-lugos-surface px-5 py-4"
          >
            {#if installed}
              <div class="flex flex-col gap-2 sm:flex-row sm:gap-3">
                <button
                  type="button"
                  class="app-no-drag flex w-full items-center justify-center gap-2 rounded-lg bg-lugos-accent py-2 text-sm font-medium text-white hover:bg-blue-600 disabled:opacity-50 sm:flex-1"
                  disabled={installBusy}
                  onclick={() => onOpen(entry.id)}
                >
                  <Icon icon={mdiPlay} class="size-4 shrink-0" />
                  Open
                </button>
                <button
                  type="button"
                  class="app-no-drag flex w-full items-center justify-center gap-2 rounded-lg border border-lugos-border py-2 text-sm font-medium text-slate-200 hover:bg-white/5 disabled:opacity-50 sm:flex-1"
                  disabled={installBusy}
                  onclick={() => onInstall(entry)}
                >
                  <Icon icon={mdiRefresh} class="size-4 shrink-0" />
                  Reinstall…
                </button>
              </div>
            {:else}
              <button
                type="button"
                class="app-no-drag flex w-full items-center justify-center gap-2 rounded-lg bg-lugos-accent py-2 text-sm font-medium text-white hover:bg-blue-600 disabled:opacity-50"
                disabled={installBusy}
                onclick={() => onInstall(entry)}
              >
                <Icon icon={mdiDownloadOutline} class="size-4 shrink-0" />
                Install
              </button>
            {/if}
          </div>
        </li>
      {/each}
    </ul>
  {/if}
</div>

{#if previewError}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/70 p-4"
    role="alertdialog"
  >
    <div
      class="w-full max-w-md rounded-xl border border-lugos-border bg-lugos-surface p-6"
    >
      <h2 class="font-semibold text-white">Could not load app manifest</h2>
      <p class="mt-2 text-sm text-red-200">{previewError}</p>
      <button
        type="button"
        class="mt-4 inline-flex items-center justify-center gap-2 rounded-lg border border-lugos-border px-4 py-2 text-sm text-white hover:bg-white/5"
        onclick={closeError}
      >
        <Icon icon={mdiClose} class="size-4 shrink-0" />
        Close
      </button>
    </div>
  </div>
{/if}

<PermissionDialog
  open={dialogOpen}
  manifest={preview}
  oncancel={closeDialog}
  onconfirm={confirmInstall}
/>
