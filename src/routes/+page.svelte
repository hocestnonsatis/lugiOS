<script lang="ts">
  import Icon from "@iconify/svelte";
  import { mdiLoading } from "$lib/iconData";
  import { goto } from "$app/navigation";
  import { onMount } from "svelte";

  onMount(() => {
    goto("/marketplace");
  });
</script>

<p class="flex items-center gap-2 text-sm text-lugos-muted">
  <Icon icon={mdiLoading} class="size-4 shrink-0 animate-spin text-lugos-accent" />
  Opening Marketplace…
</p>
