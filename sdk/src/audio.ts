import { appIdFromEnv, requireGrant } from "./grants.js";
import { lugosInvoke } from "./invoke.js";

/** Loads bundled audio bytes; play via Web Audio API in the app. */
export async function loadAsset(assetPath: string): Promise<ArrayBuffer> {
  requireGrant("audio:play");
  const bytes = await lugosInvoke("lugos_audio_read_asset", {
    app_id: appIdFromEnv(),
    asset_path: assetPath,
  });
  return new Uint8Array(bytes as number[]).buffer;
}

export async function play(assetPath: string): Promise<void> {
  const buf = await loadAsset(assetPath);
  const ctx = new AudioContext();
  const audioBuf = await ctx.decodeAudioData(buf.slice(0));
  const src = ctx.createBufferSource();
  src.buffer = audioBuf;
  src.connect(ctx.destination);
  src.start();
}
