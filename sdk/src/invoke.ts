type Internals = {
  invoke: (cmd: string, args?: Record<string, unknown>) => Promise<unknown>;
};

export function lugosInvoke(
  cmd: string,
  args: Record<string, unknown> = {},
): Promise<unknown> {
  const w = window as unknown as { __TAURI_INTERNALS__?: Internals };
  const i = w.__TAURI_INTERNALS__;
  if (!i?.invoke) {
    throw new Error("LugiOS AppBridge requires a Tauri webview (Tauri internals missing)");
  }
  return i.invoke(cmd, args);
}
