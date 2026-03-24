<script lang="ts">
  import Icon from "@iconify/svelte";
  import { mdiCheck, mdiClose, mdiShieldLockOutline } from "$lib/iconData";
  import { describePermission } from "$lib/permissionLabels";
  import type { AppManifest } from "$lib/types";

  interface Props {
    open: boolean;
    manifest: AppManifest | null;
    oncancel: () => void;
    onconfirm: (grants: string[]) => void;
  }

  let { open, manifest, oncancel, onconfirm }: Props = $props();

  let selected = $state<Record<string, boolean>>({});

  $effect(() => {
    if (manifest && open) {
      const next: Record<string, boolean> = {};
      for (const p of manifest.permissions) next[p] = true;
      selected = next;
    }
  });

  function toggle(k: string) {
    selected = { ...selected, [k]: !selected[k] };
  }

  function confirm() {
    const grants = Object.entries(selected)
      .filter(([, v]) => v)
      .map(([k]) => k);
    onconfirm(grants);
  }
</script>

{#if open && manifest}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/70 p-4"
    role="presentation"
  >
    <div
      class="bg-lugos-surface max-h-[90vh] w-full max-w-lg overflow-y-auto rounded-xl border border-lugos-border p-6 shadow-2xl"
      role="dialog"
      aria-modal="true"
      aria-labelledby="perm-title"
    >
      <h2
        id="perm-title"
        class="flex items-start gap-2 text-lg font-semibold text-white"
      >
        <Icon
          icon={mdiShieldLockOutline}
          class="mt-0.5 size-5 shrink-0 text-lugos-accent"
        />
        <span>Install “{manifest.displayName}”?</span>
      </h2>
      <p class="mt-2 text-sm text-lugos-muted">
        Version {manifest.version} — The app requests the following capabilities.
        Only what you leave checked will be granted.
      </p>

      <ul class="mt-4 space-y-2">
        {#each manifest.permissions as perm (perm)}
          <li>
            <label
              class="flex cursor-pointer gap-3 rounded-lg border border-lugos-border bg-lugos-bg/40 p-3 hover:border-lugos-accent/50"
            >
              <input
                type="checkbox"
                class="mt-1 rounded border-lugos-border"
                checked={selected[perm] ?? false}
                onchange={() => toggle(perm)}
              />
              <span>
                <span class="font-mono text-xs text-lugos-accent">{perm}</span>
                <span class="mt-1 block text-sm text-slate-200"
                  >{describePermission(perm)}</span
                >
              </span>
            </label>
          </li>
        {/each}
      </ul>

      <div class="mt-6 flex justify-end gap-3">
        <button
          type="button"
          class="inline-flex items-center justify-center gap-2 rounded-lg border border-lugos-border px-4 py-2 text-sm text-slate-200 hover:bg-white/5"
          onclick={oncancel}
        >
          <Icon icon={mdiClose} class="size-4 shrink-0" />
          Cancel
        </button>
        <button
          type="button"
          class="inline-flex items-center justify-center gap-2 rounded-lg bg-lugos-accent px-4 py-2 text-sm font-medium text-white hover:bg-blue-600"
          onclick={confirm}
        >
          <Icon icon={mdiCheck} class="size-4 shrink-0" />
          Allow &amp; install
        </button>
      </div>
    </div>
  </div>
{/if}
