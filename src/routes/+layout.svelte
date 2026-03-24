<script lang="ts">
  import "../app.css";
  import { page } from "$app/stores";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import type { Snippet } from "svelte";

  interface Props {
    children: Snippet;
  }

  let { children }: Props = $props();

  const appWindow = getCurrentWindow();

  const links = [
    { href: "/marketplace", label: "Marketplace" },
    { href: "/installed", label: "Installed" },
  ];

  function minimize() {
    void appWindow.minimize();
  }

  function toggleMaximize() {
    void appWindow.toggleMaximize();
  }

  function close() {
    void appWindow.close();
  }

  function onDragStripDblClick() {
    void appWindow.toggleMaximize();
  }
</script>

<div class="min-h-screen">
  <header
    class="sticky top-0 z-50 w-full border-b border-lugos-border bg-lugos-surface/90 shadow-sm shadow-black/20 backdrop-blur-md supports-[(-webkit-app-region:drag)]:cursor-default"
  >
    <!-- Full-width bar: logo flush left, tabs true center, window controls flush right -->
    <div class="flex h-11 w-full min-w-0 items-stretch">
      <div class="flex min-h-11 min-w-0 flex-1 items-center pl-3 sm:pl-4">
        <a
          draggable="false"
          href="/marketplace"
          class="app-no-drag shrink-0 py-2 text-lg font-semibold tracking-tight text-slate-100 transition-colors hover:text-white"
        >
          Lugi<span class="text-lugos-accent">OS</span>
        </a>
        <div
          class="min-h-11 min-w-6 flex-1 self-stretch select-none"
          data-tauri-drag-region
          role="presentation"
          ondblclick={onDragStripDblClick}
        ></div>
      </div>
      <nav
        class="app-no-drag flex shrink-0 items-center gap-0.5 px-2 py-2 sm:gap-1"
      >
        {#each links as link (link.href)}
          <a
            draggable="false"
            href={link.href}
            class="rounded-lg px-3 py-2 text-sm font-medium transition-colors {$page.url.pathname.startsWith(
              link.href,
            )
              ? 'bg-lugos-accent/15 text-white'
              : 'text-lugos-muted hover:bg-white/5 hover:text-white'}"
          >
            {link.label}
          </a>
        {/each}
      </nav>
      <div class="flex min-h-11 min-w-0 flex-1 items-stretch justify-end">
        <div
          class="min-h-11 min-w-6 flex-1 self-stretch select-none"
          data-tauri-drag-region
          role="presentation"
          ondblclick={onDragStripDblClick}
        ></div>
        <div
          class="app-no-drag flex h-11 shrink-0 items-stretch border-l border-lugos-border/70"
        >
          <button
            type="button"
            class="flex w-10 items-center justify-center text-slate-400 transition-colors hover:bg-white/10 hover:text-white"
            aria-label="Minimize window"
            onclick={minimize}
          >
            <svg width="10" height="1" viewBox="0 0 10 1" aria-hidden="true">
              <rect width="10" height="1" rx="0.5" fill="currentColor" />
            </svg>
          </button>
          <button
            type="button"
            class="flex w-10 items-center justify-center text-slate-400 transition-colors hover:bg-white/10 hover:text-white"
            aria-label="Maximize or restore window"
            onclick={toggleMaximize}
          >
            <svg
              width="10"
              height="10"
              viewBox="0 0 10 10"
              fill="none"
              aria-hidden="true"
            >
              <rect
                x="0.75"
                y="0.75"
                width="8.5"
                height="8.5"
                rx="0.5"
                stroke="currentColor"
                stroke-width="1.25"
              />
            </svg>
          </button>
          <button
            type="button"
            class="flex w-11 items-center justify-center text-slate-400 transition-colors hover:bg-red-600 hover:text-white"
            aria-label="Close window"
            onclick={close}
          >
            <svg
              width="10"
              height="10"
              viewBox="0 0 10 10"
              fill="none"
              aria-hidden="true"
            >
              <path
                d="M1.5 1.5l7 7M8.5 1.5l-7 7"
                stroke="currentColor"
                stroke-width="1.25"
                stroke-linecap="round"
              />
            </svg>
          </button>
        </div>
      </div>
    </div>
  </header>
  <main class="mx-auto max-w-5xl px-4 py-8">
    {@render children()}
  </main>
</div>
