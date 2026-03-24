<script lang="ts">
  import { appListingImageUrl } from "$lib/appListingLogo";
  import type { GitHubRepoStats, RegistryEntry } from "$lib/types";

  interface Props {
    entry: RegistryEntry;
    stats?: GitHubRepoStats | null;
    variant?: "card" | "hero";
    class?: string;
  }

  let {
    entry,
    stats = null,
    variant = "card",
    class: className = "",
  }: Props = $props();

  const dimension = $derived(variant === "hero" ? 96 : 48);
  const src = $derived(appListingImageUrl(entry, stats, dimension));
  let broken = $state(false);

  const boxClass = $derived(
    variant === "hero" ? "size-24 text-2xl" : "size-12 text-lg",
  );
  const initial = $derived(
    entry.displayName.trim().charAt(0).toUpperCase() || "?",
  );

  $effect(() => {
    void entry.id;
    void src;
    broken = false;
  });
</script>

{#if src && !broken}
  <img
    src={src}
    alt={entry.displayName}
    width={dimension}
    height={dimension}
    draggable="false"
    class="shrink-0 rounded-xl border border-lugos-border bg-lugos-bg object-cover {variant ===
    'hero'
      ? 'size-24'
      : 'size-12'} {className}"
    onerror={() => {
      broken = true;
    }}
  />
{:else}
  <div
    class="flex shrink-0 items-center justify-center rounded-xl border border-lugos-border bg-lugos-bg font-semibold text-slate-300 {boxClass} {className}"
    aria-hidden="true"
  >
    {initial}
  </div>
{/if}
