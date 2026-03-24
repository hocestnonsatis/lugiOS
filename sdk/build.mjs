import * as esbuild from "esbuild";
import { copyFileSync, mkdirSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = dirname(fileURLToPath(import.meta.url));
const outDir = join(__dirname, "dist");
const resourcesDir = join(__dirname, "..", "src-tauri", "resources");
mkdirSync(outDir, { recursive: true });
mkdirSync(resourcesDir, { recursive: true });

await esbuild.build({
  entryPoints: [join(__dirname, "src", "index.ts")],
  bundle: true,
  format: "iife",
  platform: "browser",
  target: "es2022",
  outfile: join(outDir, "bridge.iife.js"),
});

copyFileSync(join(outDir, "bridge.iife.js"), join(resourcesDir, "bridge.iife.js"));
console.log("Wrote sdk/dist/bridge.iife.js and src-tauri/resources/bridge.iife.js");
