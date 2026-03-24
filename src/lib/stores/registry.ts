import { invoke } from "@tauri-apps/api/core";
import { invokeErrorMessage } from "$lib/invokeError";
import { derived, writable } from "svelte/store";
import type { RegistryEntry } from "$lib/types";

const entries = writable<RegistryEntry[]>([]);
const loading = writable(false);
const error = writable<string | null>(null);

export const registryEntries = derived(entries, ($e) => $e);
export const registryLoading = derived(loading, ($l) => $l);
export const registryError = derived(error, ($e) => $e);

async function fetchList(refresh: boolean) {
  loading.set(true);
  error.set(null);
  try {
    const list = await invoke<RegistryEntry[]>(
      refresh ? "refresh_registry" : "get_registry",
    );
    entries.set(list);
  } catch (e) {
    error.set(invokeErrorMessage(e));
  } finally {
    loading.set(false);
  }
}

export function initRegistry() {
  void fetchList(false);
}

export async function refreshRegistry() {
  await fetchList(true);
}
