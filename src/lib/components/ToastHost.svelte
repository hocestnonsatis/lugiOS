<script lang="ts">
  import { dismissToast, toastState } from "$lib/stores/toast";
  import Icon from "@iconify/svelte";
  import { mdiCheckCircleOutline, mdiClose, mdiInformationOutline, mdiAlertCircleOutline } from "$lib/iconData";

  const variantClass = $derived.by(() => {
    const v = $toastState?.variant;
    if (v === "error") {
      return "border-red-900/60 bg-red-950/90 text-red-100";
    }
    if (v === "info") {
      return "border-slate-600/60 bg-slate-900/95 text-slate-100";
    }
    return "border-emerald-900/50 bg-emerald-950/90 text-emerald-50";
  });

  const icon = $derived.by(() => {
    const v = $toastState?.variant;
    if (v === "error") return mdiAlertCircleOutline;
    if (v === "info") return mdiInformationOutline;
    return mdiCheckCircleOutline;
  });
</script>

{#if $toastState}
  <div
    class="pointer-events-none fixed inset-x-0 bottom-6 z-[200] flex justify-center px-4"
    role="status"
  >
    <div
      class="pointer-events-auto flex max-w-md items-start gap-3 rounded-xl border px-4 py-3 shadow-lg shadow-black/40 {variantClass}"
    >
      <Icon icon={icon} class="mt-0.5 size-5 shrink-0 opacity-90" aria-hidden="true" />
      <p class="min-w-0 flex-1 text-sm leading-snug">{$toastState.message}</p>
      <button
        type="button"
        class="app-no-drag shrink-0 rounded p-1 text-current opacity-70 hover:opacity-100"
        aria-label="Dismiss"
        onclick={() => dismissToast()}
      >
        <Icon icon={mdiClose} class="size-4" aria-hidden="true" />
      </button>
    </div>
  </div>
{/if}
