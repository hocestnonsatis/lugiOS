<script lang="ts">
  import Icon from "@iconify/svelte";
  import ListingAppLogo from "$lib/components/ListingAppLogo.svelte";
  import PermissionDialog from "$lib/components/PermissionDialog.svelte";
  import {
    mdiArrowLeft,
    mdiBugOutline,
    mdiClose,
    mdiDownloadOutline,
    mdiEyeOutline,
    mdiGithub,
    mdiOpenInNew,
    mdiPlay,
    mdiRefresh,
    mdiSourceFork,
    mdiStar,
  } from "$lib/iconData";
  import { invokeErrorMessage } from "$lib/invokeError";
  import { initRegistry, registryEntries, registryLoading } from "$lib/stores/registry";
  import {
    installApp,
    launchApp,
    loadInstalled,
    installedApps,
  } from "$lib/stores/installed";
  import type { AppManifest, GitHubRepoStats } from "$lib/types";
  import { page } from "$app/stores";
  import { afterNavigate } from "$app/navigation";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  const id = $derived($page.params.id ?? "");

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

  const entry = $derived($registryEntries.find((e) => e.id === id) ?? null);

  let stats = $state<GitHubRepoStats | null>(null);
  let statsError = $state<string | null>(null);
  let statsLoading = $state(false);

  $effect(() => {
    const repo = entry?.repo;
    if (!repo) {
      stats = null;
      statsError = null;
      statsLoading = false;
      return;
    }
    let cancelled = false;
    statsLoading = true;
    statsError = null;
    stats = null;
    invoke<GitHubRepoStats>("get_github_repo_stats", { repoUrl: repo })
      .then((s) => {
        if (!cancelled) stats = s;
      })
      .catch((e) => {
        if (!cancelled) statsError = invokeErrorMessage(e);
      })
      .finally(() => {
        if (!cancelled) statsLoading = false;
      });
    return () => {
      cancelled = true;
    };
  });

  let dialogOpen = $state(false);
  let preview = $state<AppManifest | null>(null);
  let previewError = $state<string | null>(null);
  let installBusy = $state(false);
  let installErr = $state<string | null>(null);
  let launchErr = $state<string | null>(null);

  const installedManifest = $derived(
    entry ? $installedApps.find((a) => a.id === entry.id) : undefined,
  );
  const installed = $derived(installedManifest !== undefined);

  async function onInstall() {
    if (!entry) return;
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
    preview = null;
    previewError = null;
  }

  function closePreviewError() {
    previewError = null;
  }

  async function confirmInstall(grants: string[]) {
    if (!entry) return;
    installBusy = true;
    installErr = null;
    try {
      await installApp(entry.id, entry.repo, grants);
      closeDialog();
    } catch (e) {
      installErr = invokeErrorMessage(e);
    } finally {
      installBusy = false;
    }
  }

  async function onOpen() {
    if (!entry) return;
    launchErr = null;
    try {
      await launchApp(entry.id);
    } catch (e) {
      launchErr = invokeErrorMessage(e);
    }
  }

  function formatDate(iso: string | null | undefined): string {
    if (!iso) return "—";
    const d = new Date(iso);
    if (Number.isNaN(d.getTime())) return iso;
    return d.toLocaleDateString(undefined, {
      year: "numeric",
      month: "short",
      day: "numeric",
    });
  }
</script>

<div class="flex flex-col gap-8">
  <div>
    <a
      href="/marketplace"
      class="app-no-drag inline-flex items-center gap-1.5 text-sm text-lugos-muted transition-colors hover:text-white"
    >
      <Icon icon={mdiArrowLeft} class="size-4 shrink-0" />
      Marketplace
    </a>
  </div>

  {#if $registryLoading && !entry}
    <p class="text-lugos-muted text-sm">Loading…</p>
  {:else if !entry}
    <div
      class="rounded-xl border border-lugos-border bg-lugos-surface p-8 text-center"
    >
      <h1 class="text-lg font-semibold text-white">App not found</h1>
      <p class="mt-2 text-sm text-lugos-muted">
        No listing with id “{id}”. It may have been removed from the registry,
        or the catalog has not finished loading.
      </p>
      <a
        href="/marketplace"
        class="app-no-drag mt-4 inline-flex items-center justify-center gap-2 rounded-lg bg-lugos-accent px-4 py-2 text-sm font-medium text-white hover:bg-blue-600"
      >
        <Icon icon={mdiArrowLeft} class="size-4 shrink-0" />
        Back to Marketplace
      </a>
    </div>
  {:else}
    <div class="flex flex-col gap-6">
      <header class="flex flex-wrap items-start gap-5">
        <ListingAppLogo {entry} stats={stats} variant="hero" />
        <div class="flex min-w-0 flex-1 flex-col gap-2">
          <div class="flex flex-wrap items-start justify-between gap-3">
            <div class="min-w-0">
              <h1 class="text-2xl font-semibold text-white">
                {entry.displayName}
              </h1>
              {#if installedManifest}
                <p class="mt-1 text-sm font-medium text-emerald-300/90">
                  Installed · v{installedManifest.version}
                </p>
              {/if}
            </div>
            {#if entry.verified}
              <span
                class="shrink-0 rounded bg-emerald-500/20 px-2 py-1 text-xs font-medium text-emerald-300"
                >Verified</span
              >
            {:else}
              <span
                class="shrink-0 rounded bg-amber-500/15 px-2 py-1 text-xs text-amber-200"
                >Unverified</span
              >
            {/if}
          </div>
          <p class="font-mono text-xs text-slate-500">{entry.id}</p>
        </div>
      </header>

      <p class="text-sm leading-relaxed text-lugos-muted">
        {entry.description}
      </p>

      <div class="flex flex-wrap gap-2">
        {#each entry.tags as tag (tag)}
          <span
            class="rounded bg-lugos-bg px-2 py-1 text-xs text-slate-400"
            >{tag}</span
          >
        {/each}
      </div>

      <p class="text-xs text-slate-500">
        Listed by {entry.author} · published {formatDate(entry.publishedAt)}
      </p>

      <section
        class="rounded-xl border border-lugos-border bg-lugos-surface/60 p-5"
      >
        <h2
          class="flex items-center gap-2 text-sm font-semibold uppercase tracking-wide text-slate-400"
        >
          <Icon icon={mdiGithub} class="size-4 text-slate-400" />
          Repository (GitHub)
        </h2>

        {#if statsLoading}
          <p class="mt-3 text-sm text-lugos-muted">Loading repository data…</p>
        {:else if statsError}
          <p class="mt-3 text-sm text-amber-200/90">{statsError}</p>
          <p class="mt-1 text-xs text-slate-500">
            Open the repo link below if the API is rate-limited or the repo is private.
          </p>
        {:else if stats}
          <div class="mt-4 grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
            <div>
              <p class="text-2xl font-semibold text-white">
                {stats.stars.toLocaleString()}
              </p>
              <p class="flex items-center gap-1 text-xs text-lugos-muted">
                <Icon icon={mdiStar} class="size-3.5 text-amber-400/90" />
                Stars
              </p>
            </div>
            <div>
              <p class="text-2xl font-semibold text-white">
                {stats.forks.toLocaleString()}
              </p>
              <p class="flex items-center gap-1 text-xs text-lugos-muted">
                <Icon
                  icon={mdiSourceFork}
                  class="size-3.5 text-slate-400"
                />
                Forks
              </p>
            </div>
            <div>
              <p class="text-2xl font-semibold text-white">
                {stats.openIssues.toLocaleString()}
              </p>
              <p class="flex items-center gap-1 text-xs text-lugos-muted">
                <Icon icon={mdiBugOutline} class="size-3.5 text-orange-300/80" />
                Open issues
              </p>
            </div>
            <div>
              <p class="text-2xl font-semibold text-white">
                {stats.watchers.toLocaleString()}
              </p>
              <p class="flex items-center gap-1 text-xs text-lugos-muted">
                <Icon icon={mdiEyeOutline} class="size-3.5 text-sky-300/80" />
                Watchers
              </p>
            </div>
          </div>

          {#if stats.description}
            <p class="mt-4 text-sm text-lugos-muted">{stats.description}</p>
          {/if}

          <dl class="mt-4 grid gap-2 text-sm sm:grid-cols-2">
            {#if stats.language}
              <div>
                <dt class="text-xs text-slate-500">Language</dt>
                <dd class="text-slate-200">{stats.language}</dd>
              </div>
            {/if}
            <div>
              <dt class="text-xs text-slate-500">Default branch</dt>
              <dd class="font-mono text-slate-200">{stats.defaultBranch}</dd>
            </div>
            <div>
              <dt class="text-xs text-slate-500">Last push</dt>
              <dd class="text-slate-200">{formatDate(stats.pushedAt)}</dd>
            </div>
            {#if stats.licenseName}
              <div>
                <dt class="text-xs text-slate-500">License</dt>
                <dd class="text-slate-200">{stats.licenseName}</dd>
              </div>
            {/if}
            {#if stats.homepage}
              <div class="sm:col-span-2">
                <dt class="text-xs text-slate-500">Homepage</dt>
                <dd>
                  <a
                    href={stats.homepage}
                    target="_blank"
                    rel="noreferrer"
                    class="break-all text-blue-400 hover:underline"
                    >{stats.homepage}</a
                  >
                </dd>
              </div>
            {/if}
          </dl>

          {#if stats.topics.length > 0}
            <div class="mt-4">
              <p class="text-xs text-slate-500">Topics</p>
              <div class="mt-1 flex flex-wrap gap-1">
                {#each stats.topics as t (t)}
                  <span
                    class="rounded border border-lugos-border bg-lugos-bg/50 px-2 py-0.5 text-xs text-slate-300"
                    >{t}</span
                  >
                {/each}
              </div>
            </div>
          {/if}
        {/if}

        <a
          href={stats?.htmlUrl ?? entry.repo}
          target="_blank"
          rel="noreferrer"
          class="app-no-drag mt-4 inline-flex items-center gap-2 text-sm font-medium text-blue-400 hover:underline"
        >
          <Icon icon={mdiGithub} class="size-4 shrink-0" />
          View on GitHub
          <Icon icon={mdiOpenInNew} class="size-3.5 shrink-0 opacity-70" />
        </a>
      </section>

      {#if installErr}
        <p
          class="rounded-lg border border-red-900/50 bg-red-950/40 p-4 text-sm text-red-200"
        >
          {installErr}
        </p>
      {/if}

      {#if launchErr}
        <p
          class="rounded-lg border border-red-900/50 bg-red-950/40 p-4 text-sm text-red-200"
        >
          {launchErr}
        </p>
      {/if}

      {#if installed}
        <div class="flex w-full max-w-md flex-col gap-3 sm:flex-row">
          <button
            type="button"
            class="app-no-drag flex items-center justify-center gap-2 rounded-lg bg-lugos-accent py-3 text-sm font-medium text-white hover:bg-blue-600 disabled:opacity-50 sm:flex-1"
            disabled={installBusy}
            onclick={onOpen}
          >
            <Icon icon={mdiPlay} class="size-4 shrink-0" />
            Open
          </button>
          <button
            type="button"
            class="app-no-drag flex items-center justify-center gap-2 rounded-lg border border-lugos-border py-3 text-sm font-medium text-slate-200 hover:bg-white/5 disabled:opacity-50 sm:flex-1"
            disabled={installBusy}
            onclick={onInstall}
          >
            <Icon icon={mdiRefresh} class="size-4 shrink-0" />
            Reinstall…
          </button>
        </div>
      {:else}
        <button
          type="button"
          class="app-no-drag flex w-full max-w-xs items-center justify-center gap-2 rounded-lg bg-lugos-accent py-3 text-sm font-medium text-white hover:bg-blue-600 disabled:opacity-50 sm:w-auto"
          disabled={installBusy}
          onclick={onInstall}
        >
          <Icon icon={mdiDownloadOutline} class="size-4 shrink-0" />
          Install…
        </button>
      {/if}
    </div>
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
        onclick={closePreviewError}
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
