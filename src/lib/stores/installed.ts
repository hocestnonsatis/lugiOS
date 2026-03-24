import { invoke } from "@tauri-apps/api/core";
import { invokeErrorMessage } from "$lib/invokeError";
import { derived, writable } from "svelte/store";
import type { AppManifest, AppUpdateStatus } from "$lib/types";

const apps = writable<AppManifest[]>([]);
const loading = writable(false);
const error = writable<string | null>(null);

const updateStatuses = writable<AppUpdateStatus[]>([]);
const updatesLoading = writable(false);
const updatesError = writable<string | null>(null);

export const installedApps = derived(apps, ($a) => $a);
export const installedLoading = derived(loading, ($l) => $l);
export const installedError = derived(error, ($e) => $e);

export const appUpdateStatuses = derived(updateStatuses, ($u) => $u);
export const updatesCheckLoading = derived(updatesLoading, ($l) => $l);
export const updatesCheckError = derived(updatesError, ($e) => $e);

export async function loadInstalled() {
  loading.set(true);
  error.set(null);
  try {
    const list = await invoke<AppManifest[]>("list_installed");
    apps.set(list);
  } catch (e) {
    error.set(invokeErrorMessage(e));
  } finally {
    loading.set(false);
  }
}

export async function installApp(
  appId: string,
  repoUrl: string,
  grants: string[],
) {
  await invoke("install_app", { appId, repoUrl, grants });
  await loadInstalled();
}

export async function uninstallApp(appId: string) {
  await invoke("uninstall_app", { appId });
  await loadInstalled();
}

export async function launchApp(appId: string) {
  await invoke("launch_app", { appId });
}

export async function checkAppUpdates() {
  updatesLoading.set(true);
  updatesError.set(null);
  try {
    const list = await invoke<AppUpdateStatus[]>("check_app_updates");
    updateStatuses.set(list);
  } catch (e) {
    updatesError.set(invokeErrorMessage(e));
  } finally {
    updatesLoading.set(false);
  }
}

export async function upgradeApp(appId: string) {
  await invoke("upgrade_app", { appId });
  await loadInstalled();
  await checkAppUpdates();
}
