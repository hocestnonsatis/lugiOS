<script lang="ts">
  import { goto } from "$app/navigation";
  import { onMount } from "svelte";

  onMount(() => {
    goto("/marketplace");
  });
</script>

<p class="text-lugos-muted text-sm">Opening Marketplace…</p>
