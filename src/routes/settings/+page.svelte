<script lang="ts">
  import Icon from "@iconify/svelte";
  import {
    mdiCloudDownloadOutline,
    mdiGithub,
    mdiInformationOutline,
    mdiRefresh,
  } from "$lib/iconData";
  import { invokeErrorMessage } from "$lib/invokeError";
  import { refreshRegistry, registryError } from "$lib/stores/registry";
  import { showToast } from "$lib/stores/toast";
  import type { HostSettingsPayload } from "$lib/types";
  import { getIdentifier, getName, getTauriVersion, getVersion } from "@tauri-apps/api/app";
  import { invoke } from "@tauri-apps/api/core";
  import { relaunch } from "@tauri-apps/plugin-process";
  import { check } from "@tauri-apps/plugin-updater";
  import { onMount } from "svelte";
  import { get } from "svelte/store";

  let appVersion = $state("—");
  let tauriVersion = $state("—");
  let appName = $state("—");
  let bundleId = $state("—");
  let metaError = $state<string | null>(null);

  let refreshing = $state(false);
  let refreshMessage = $state<string | null>(null);
  let refreshErr = $state<string | null>(null);

  let hostSettings = $state<HostSettingsPayload | null>(null);
  let hostSettingsLoadErr = $state<string | null>(null);
  let registryInput = $state("");
  let savingRegistry = $state(false);
  let registrySaveErr = $state<string | null>(null);

  let hostUpdateBusy = $state(false);
  let hostUpdateErr = $state<string | null>(null);

  onMount(() => {
    void (async () => {
      try {
        const [v, tv, n, id] = await Promise.all([
          getVersion(),
          getTauriVersion(),
          getName(),
          getIdentifier(),
        ]);
        appVersion = v;
        tauriVersion = tv;
        appName = n;
        bundleId = id;
      } catch (e) {
        metaError = invokeErrorMessage(e);
      }
    })();
    void loadHostSettings();
  });

  async function loadHostSettings() {
    hostSettingsLoadErr = null;
    try {
      const h = await invoke<HostSettingsPayload>("get_host_settings");
      hostSettings = h;
      registryInput = h.savedRegistryUrl ?? "";
    } catch (e) {
      hostSettingsLoadErr = invokeErrorMessage(e);
    }
  }

  async function onForceRefreshRegistry() {
    refreshing = true;
    refreshMessage = null;
    refreshErr = null;
    try {
      await refreshRegistry();
      const re = get(registryError);
      if (re) {
        refreshErr = re;
      } else {
        refreshMessage = "Registry refreshed from the network.";
      }
    } catch (e) {
      refreshErr = invokeErrorMessage(e);
    } finally {
      refreshing = false;
    }
  }

  async function saveRegistryUrl() {
    savingRegistry = true;
    registrySaveErr = null;
    try {
      const trimmed = registryInput.trim();
      const h = await invoke<HostSettingsPayload>("set_host_registry_url", {
        url: trimmed.length > 0 ? trimmed : null,
      });
      hostSettings = h;
      registryInput = h.savedRegistryUrl ?? "";
      await refreshRegistry();
      const re = get(registryError);
      if (re) {
        registrySaveErr = re;
        showToast("URL saved, but refreshing the catalog failed. Check the URL or your connection.", "error");
      } else {
        showToast("Registry URL saved and catalog refreshed.", "success");
      }
    } catch (e) {
      registrySaveErr = invokeErrorMessage(e);
    } finally {
      savingRegistry = false;
    }
  }

  async function clearSavedRegistryUrl() {
    registryInput = "";
    savingRegistry = true;
    registrySaveErr = null;
    try {
      const h = await invoke<HostSettingsPayload>("set_host_registry_url", { url: null });
      hostSettings = h;
      await refreshRegistry();
      showToast("Using default registry URL.", "info");
    } catch (e) {
      registrySaveErr = invokeErrorMessage(e);
    } finally {
      savingRegistry = false;
    }
  }

  async function checkHostUpdates() {
    hostUpdateBusy = true;
    hostUpdateErr = null;
    try {
      const update = await check();
      if (!update) {
        showToast("LugiOS is up to date.", "info");
        return;
      }
      await update.downloadAndInstall();
      await relaunch();
    } catch (e) {
      hostUpdateErr = invokeErrorMessage(e);
    } finally {
      hostUpdateBusy = false;
    }
  }
</script>

<svelte:head>
  <title>Settings · LugiOS</title>
</svelte:head>

<div class="space-y-10">
  <div>
    <h1 class="text-2xl font-semibold tracking-tight text-white">Settings</h1>
    <p class="mt-1 text-sm text-lugos-muted">
      Host preferences and diagnostics. Mini-app permissions are granted per app at install time.
    </p>
  </div>

  <section class="rounded-xl border border-lugos-border bg-lugos-surface/60 p-5 shadow-sm shadow-black/10">
    <h2 class="flex items-center gap-2 text-sm font-semibold uppercase tracking-wide text-lugos-muted">
      <Icon icon={mdiRefresh} class="size-4 opacity-90" aria-hidden="true" />
      Marketplace registry
    </h2>
    <p class="mt-2 max-w-prose text-sm text-slate-300">
      The catalog is cached for one hour. Set a custom JSON URL below (saved on disk), or use
      <code class="rounded bg-black/30 px-1.5 py-0.5 text-xs text-slate-200">LUGIOS_REGISTRY_URL</code>
      to override for this process (takes precedence; requires restart to clear).
    </p>
    {#if hostSettingsLoadErr}
      <p class="mt-3 text-sm text-red-300/90">{hostSettingsLoadErr}</p>
    {:else if hostSettings}
      <div class="mt-4 space-y-2 text-sm">
        <p class="text-lugos-muted">
          Effective URL
          {#if hostSettings.envOverrideActive}
            <span class="text-amber-200/90">(environment variable is active)</span>
          {/if}
        </p>
        <p class="break-all rounded border border-lugos-border/80 bg-black/20 px-3 py-2 font-mono text-xs text-slate-200">
          {hostSettings.resolvedRegistryUrl}
        </p>
      </div>
      <div class="mt-4 space-y-2">
        <label class="block text-sm font-medium text-slate-300" for="registry-url">
          Custom registry URL (optional)
        </label>
        <input
          id="registry-url"
          type="url"
          autocomplete="off"
          placeholder="https://…/registry.json"
          bind:value={registryInput}
          disabled={hostSettings.envOverrideActive || savingRegistry}
          class="app-no-drag w-full max-w-2xl rounded-lg border border-lugos-border bg-lugos-bg px-3 py-2 font-mono text-sm text-slate-100 placeholder:text-slate-500 focus:border-lugos-accent focus:outline-none focus:ring-1 focus:ring-lugos-accent disabled:cursor-not-allowed disabled:opacity-60"
        />
        {#if hostSettings.envOverrideActive}
          <p class="text-xs text-amber-200/90">
            Unset <code class="rounded bg-black/30 px-1">LUGIOS_REGISTRY_URL</code> and restart to edit the saved URL.
          </p>
        {/if}
      </div>
      <div class="mt-4 flex flex-wrap items-center gap-3">
        <button
          type="button"
          class="inline-flex items-center gap-2 rounded-lg bg-lugos-accent px-4 py-2 text-sm font-medium text-lugos-bg transition-opacity hover:opacity-95 disabled:cursor-not-allowed disabled:opacity-50"
          disabled={hostSettings.envOverrideActive || savingRegistry}
          onclick={saveRegistryUrl}
        >
          {savingRegistry ? "Saving…" : "Save and refresh"}
        </button>
        <button
          type="button"
          class="inline-flex items-center gap-2 rounded-lg border border-lugos-border px-4 py-2 text-sm font-medium text-slate-200 hover:bg-white/5 disabled:cursor-not-allowed disabled:opacity-50"
          disabled={hostSettings.envOverrideActive || savingRegistry}
          onclick={clearSavedRegistryUrl}
        >
          Use default URL
        </button>
        <button
          type="button"
          class="inline-flex items-center gap-2 rounded-lg border border-lugos-border px-4 py-2 text-sm font-medium text-slate-200 hover:bg-white/5 disabled:opacity-50"
          disabled={refreshing}
          onclick={onForceRefreshRegistry}
        >
          <Icon icon={mdiRefresh} class={refreshing ? "size-4 animate-spin" : "size-4"} aria-hidden="true" />
          {refreshing ? "Refreshing…" : "Refresh only"}
        </button>
      </div>
      {#if registrySaveErr}
        <p class="mt-3 text-sm text-red-300/90">{registrySaveErr}</p>
      {/if}
      {#if refreshMessage}
        <p class="mt-3 text-sm text-emerald-300/90">{refreshMessage}</p>
      {/if}
      {#if refreshErr}
        <p class="mt-3 text-sm text-red-300/90">{refreshErr}</p>
      {/if}
    {/if}
  </section>

  <section class="rounded-xl border border-lugos-border bg-lugos-surface/60 p-5 shadow-sm shadow-black/10">
    <h2 class="flex items-center gap-2 text-sm font-semibold uppercase tracking-wide text-lugos-muted">
      <Icon icon={mdiCloudDownloadOutline} class="size-4 opacity-90" aria-hidden="true" />
      LugiOS updates
    </h2>
    <p class="mt-2 max-w-prose text-sm text-slate-300">
      Check whether a newer signed build of this host app is published on GitHub Releases. Installing requires a valid
      update manifest and matching signing keys from the maintainer.
    </p>
    <div class="mt-4">
      <button
        type="button"
        class="inline-flex items-center gap-2 rounded-lg border border-lugos-border bg-lugos-bg px-4 py-2 text-sm font-medium text-slate-100 hover:bg-white/5 disabled:cursor-not-allowed disabled:opacity-50"
        disabled={hostUpdateBusy}
        onclick={checkHostUpdates}
      >
        <Icon
          icon={mdiCloudDownloadOutline}
          class={hostUpdateBusy ? "size-4 animate-pulse" : "size-4"}
          aria-hidden="true"
        />
        {hostUpdateBusy ? "Checking…" : "Check for LugiOS updates"}
      </button>
    </div>
    {#if hostUpdateErr}
      <p class="mt-3 text-sm text-amber-100/90">{hostUpdateErr}</p>
    {/if}
  </section>

  <section class="rounded-xl border border-lugos-border bg-lugos-surface/60 p-5 shadow-sm shadow-black/10">
    <h2 class="flex items-center gap-2 text-sm font-semibold uppercase tracking-wide text-lugos-muted">
      <Icon icon={mdiInformationOutline} class="size-4 opacity-90" aria-hidden="true" />
      About this app
    </h2>
    {#if metaError}
      <p class="mt-3 text-sm text-red-300/90">{metaError}</p>
    {:else}
      <dl class="mt-4 grid gap-2 text-sm sm:grid-cols-2">
        <div class="text-lugos-muted">Name</div>
        <div class="text-slate-100">{appName}</div>
        <div class="text-lugos-muted">Version</div>
        <div class="text-slate-100">{appVersion}</div>
        <div class="text-lugos-muted">Bundle ID</div>
        <div class="break-all text-slate-100">{bundleId}</div>
        <div class="text-lugos-muted">Tauri</div>
        <div class="text-slate-100">{tauriVersion}</div>
      </dl>
    {/if}
    <a
      href="https://github.com/hocestnonsatis/lugiOS"
      target="_blank"
      rel="noreferrer"
      class="app-no-drag mt-4 inline-flex items-center gap-2 text-sm font-medium text-lugos-accent hover:underline"
    >
      <Icon icon={mdiGithub} class="size-4" aria-hidden="true" />
      Source on GitHub
    </a>
  </section>
</div>
