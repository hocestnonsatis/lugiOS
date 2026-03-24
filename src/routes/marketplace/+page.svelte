<script lang="ts">
  import PermissionDialog from "$lib/components/PermissionDialog.svelte";
  import {
    initRegistry,
    refreshRegistry,
    registryEntries,
    registryError,
    registryLoading,
  } from "$lib/stores/registry";
  import { installApp } from "$lib/stores/installed";
  import type { AppManifest, RegistryEntry } from "$lib/types";
  import { invoke } from "@tauri-apps/api/core";
  import { invokeErrorMessage } from "$lib/invokeError";
  import { onMount } from "svelte";

  let dialogOpen = $state(false);
  let activeEntry = $state<RegistryEntry | null>(null);
  let preview = $state<AppManifest | null>(null);
  let previewError = $state<string | null>(null);
  let installBusy = $state(false);
  let installErr = $state<string | null>(null);

  onMount(() => initRegistry());

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
</script>

<div class="flex flex-col gap-6">
  <div class="flex flex-wrap items-end justify-between gap-4">
    <div>
      <h1 class="text-2xl font-semibold text-white">Marketplace</h1>
      <p class="mt-1 text-sm text-lugos-muted">
        Apps from the community registry (GitHub). Install adds a release
        bundle to your machine.
      </p>
    </div>
    <button
      type="button"
      class="rounded-lg border border-lugos-border px-3 py-2 text-sm text-slate-200 hover:bg-white/5 disabled:opacity-50"
      disabled={$registryLoading}
      onclick={() => refreshRegistry()}
    >
      Refresh
    </button>
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

  {#if $registryLoading && $registryEntries.length === 0}
    <p class="text-lugos-muted text-sm">Loading registry…</p>
  {:else if $registryEntries.length === 0}
    <p class="text-lugos-muted text-sm">
      No entries yet. Check your network or whether the registry URL is
      reachable.
    </p>
  {:else}
    <ul class="grid gap-4 sm:grid-cols-2">
      {#each $registryEntries as entry (entry.id)}
        <li
          class="flex flex-col rounded-xl border border-lugos-border bg-lugos-surface p-5"
        >
          <div class="flex items-start justify-between gap-2">
            <h2 class="font-semibold text-white">{entry.displayName}</h2>
            {#if entry.verified}
              <span
                class="rounded bg-emerald-500/20 px-2 py-0.5 text-xs font-medium text-emerald-300"
                >Verified</span
              >
            {:else}
              <span
                class="rounded bg-amber-500/15 px-2 py-0.5 text-xs text-amber-200"
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
          <button
            type="button"
            class="mt-4 rounded-lg bg-lugos-accent py-2 text-sm font-medium text-white hover:bg-blue-600 disabled:opacity-50"
            disabled={installBusy}
            onclick={() => onInstall(entry)}
          >
            Install
          </button>
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
        class="mt-4 rounded-lg border border-lugos-border px-4 py-2 text-sm text-white hover:bg-white/5"
        onclick={closeError}
      >
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
