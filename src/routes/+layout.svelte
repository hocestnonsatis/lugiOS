<script lang="ts">
  import "../app.css";
  import ToastHost from "$lib/components/ToastHost.svelte";
  import Icon from "@iconify/svelte";
  import {
    mdiCogOutline,
    mdiPackageVariant,
    mdiStoreOutline,
    mdiWindowClose,
    mdiWindowMaximize,
    mdiWindowMinimize,
  } from "$lib/iconData";
  import { page } from "$app/stores";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import type { Snippet } from "svelte";
  import type { IconifyIcon } from "@iconify/types";

  interface Props {
    children: Snippet;
  }

  let { children }: Props = $props();

  const appWindow = getCurrentWindow();

  const links: { href: string; label: string; icon: IconifyIcon }[] = [
    { href: "/marketplace", label: "Marketplace", icon: mdiStoreOutline },
    { href: "/installed", label: "Installed", icon: mdiPackageVariant },
    { href: "/settings", label: "Settings", icon: mdiCogOutline },
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
            class="flex items-center gap-2 rounded-lg px-3 py-2 text-sm font-medium transition-colors {$page.url.pathname.startsWith(
              link.href,
            )
              ? 'bg-lugos-accent/15 text-white'
              : 'text-lugos-muted hover:bg-white/5 hover:text-white'}"
          >
            <Icon icon={link.icon} class="size-4 shrink-0 opacity-90" />
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
            <Icon icon={mdiWindowMinimize} class="size-[15px]" aria-hidden="true" />
          </button>
          <button
            type="button"
            class="flex w-10 items-center justify-center text-slate-400 transition-colors hover:bg-white/10 hover:text-white"
            aria-label="Maximize or restore window"
            onclick={toggleMaximize}
          >
            <Icon icon={mdiWindowMaximize} class="size-[15px]" aria-hidden="true" />
          </button>
          <button
            type="button"
            class="flex w-11 items-center justify-center text-slate-400 transition-colors hover:bg-red-600 hover:text-white"
            aria-label="Close window"
            onclick={close}
          >
            <Icon icon={mdiWindowClose} class="size-[15px]" aria-hidden="true" />
          </button>
        </div>
      </div>
    </div>
  </header>
  <main class="mx-auto max-w-5xl px-4 py-8">
    {@render children()}
  </main>
  <ToastHost />
</div>
