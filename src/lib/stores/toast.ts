import { writable } from "svelte/store";

export type ToastVariant = "success" | "error" | "info";

export interface ToastState {
  message: string;
  variant: ToastVariant;
}

const state = writable<ToastState | null>(null);
let hideTimer: ReturnType<typeof setTimeout> | null = null;

export const toastState = state;

export function showToast(
  message: string,
  variant: ToastVariant = "success",
  durationMs = 4500,
) {
  if (hideTimer) {
    clearTimeout(hideTimer);
    hideTimer = null;
  }
  state.set({ message, variant });
  hideTimer = setTimeout(() => {
    state.set(null);
    hideTimer = null;
  }, durationMs);
}

export function dismissToast() {
  if (hideTimer) {
    clearTimeout(hideTimer);
    hideTimer = null;
  }
  state.set(null);
}
