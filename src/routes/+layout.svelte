<script lang="ts">
  import "../app.css";
  import { page } from "$app/stores";
  import type { Snippet } from "svelte";

  interface Props {
    children: Snippet;
  }

  let { children }: Props = $props();

  const links = [
    { href: "/marketplace", label: "Marketplace" },
    { href: "/installed", label: "Installed" },
  ];
</script>

<div class="min-h-screen">
  <header
    class="border-b border-lugos-border bg-lugos-surface/80 backdrop-blur"
  >
    <div class="mx-auto flex max-w-5xl items-center justify-between px-4 py-3">
      <a href="/marketplace" class="text-lg font-semibold tracking-tight">
        Lugi<span class="text-lugos-accent">OS</span>
      </a>
      <nav class="flex gap-1">
        {#each links as link (link.href)}
          <a
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
    </div>
  </header>
  <main class="mx-auto max-w-5xl px-4 py-8">
    {@render children()}
  </main>
</div>
