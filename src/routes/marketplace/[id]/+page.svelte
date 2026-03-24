<script lang="ts">
  import ListingAppLogo from "$lib/components/ListingAppLogo.svelte";
  import PermissionDialog from "$lib/components/PermissionDialog.svelte";
  import { invokeErrorMessage } from "$lib/invokeError";
  import { initRegistry, registryEntries, registryLoading } from "$lib/stores/registry";
  import { installApp } from "$lib/stores/installed";
  import type { AppManifest, GitHubRepoStats } from "$lib/types";
  import { page } from "$app/stores";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  const id = $derived($page.params.id ?? "");

  onMount(() => initRegistry());

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
      class="app-no-drag text-sm text-lugos-muted transition-colors hover:text-white"
    >
      ← Marketplace
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
        class="mt-4 inline-block rounded-lg bg-lugos-accent px-4 py-2 text-sm font-medium text-white hover:bg-blue-600"
      >
        Back to Marketplace
      </a>
    </div>
  {:else}
    <div class="flex flex-col gap-6">
      <header class="flex flex-wrap items-start gap-5">
        <ListingAppLogo {entry} stats={stats} variant="hero" />
        <div class="flex min-w-0 flex-1 flex-col gap-2">
          <div class="flex flex-wrap items-start justify-between gap-3">
            <h1 class="text-2xl font-semibold text-white">
              {entry.displayName}
            </h1>
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
        <h2 class="text-sm font-semibold uppercase tracking-wide text-slate-400">
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
              <p class="text-xs text-lugos-muted">Stars</p>
            </div>
            <div>
              <p class="text-2xl font-semibold text-white">
                {stats.forks.toLocaleString()}
              </p>
              <p class="text-xs text-lugos-muted">Forks</p>
            </div>
            <div>
              <p class="text-2xl font-semibold text-white">
                {stats.openIssues.toLocaleString()}
              </p>
              <p class="text-xs text-lugos-muted">Open issues</p>
            </div>
            <div>
              <p class="text-2xl font-semibold text-white">
                {stats.watchers.toLocaleString()}
              </p>
              <p class="text-xs text-lugos-muted">Watchers</p>
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
          class="app-no-drag mt-4 inline-flex text-sm font-medium text-blue-400 hover:underline"
        >
          View on GitHub →
        </a>
      </section>

      {#if installErr}
        <p
          class="rounded-lg border border-red-900/50 bg-red-950/40 p-4 text-sm text-red-200"
        >
          {installErr}
        </p>
      {/if}

      <button
        type="button"
        class="app-no-drag w-full max-w-xs rounded-lg bg-lugos-accent py-3 text-sm font-medium text-white hover:bg-blue-600 disabled:opacity-50 sm:w-auto"
        disabled={installBusy}
        onclick={onInstall}
      >
        Install…
      </button>
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
        class="mt-4 rounded-lg border border-lugos-border px-4 py-2 text-sm text-white hover:bg-white/5"
        onclick={closePreviewError}
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
