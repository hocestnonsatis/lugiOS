/**
 * Lowers installed demo app manifest versions so GitHub releases/latest (v0.1.0) looks newer.
 * Edits app_data_dir()/apps/<id>/app.manifest.json for demo-hello and demo-notes.
 */
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";

const IDS = ["demo-hello", "demo-notes"];
const SIMULATED_VERSION = "0.0.1";

function appDataRoot() {
  if (process.platform === "win32") {
    const base = process.env.APPDATA;
    if (!base) throw new Error("APPDATA is not set");
    return path.join(base, "com.hocestnonsatis.lugios");
  }
  if (process.platform === "darwin") {
    return path.join(
      os.homedir(),
      "Library",
      "Application Support",
      "com.hocestnonsatis.lugios",
    );
  }
  return path.join(
    os.homedir(),
    ".local",
    "share",
    "com.hocestnonsatis.lugios",
  );
}

const root = path.dirname(fileURLToPath(import.meta.url));
const projectRoot = path.resolve(root, "..");

const dataRoot = appDataRoot();
const appsDir = path.join(dataRoot, "apps");

for (const id of IDS) {
  const manifestPath = path.join(appsDir, id, "app.manifest.json");
  if (!fs.existsSync(manifestPath)) {
    console.warn(
      `Skip ${id}: not found at ${manifestPath}\n  (Install from Marketplace first.)`,
    );
    continue;
  }
  const raw = fs.readFileSync(manifestPath, "utf8");
  const j = JSON.parse(raw);
  const prev = j.version;
  j.version = SIMULATED_VERSION;
  fs.writeFileSync(manifestPath, `${JSON.stringify(j, null, 2)}\n`, "utf8");
  console.log(`${id}: version ${prev} → ${SIMULATED_VERSION}`);
}

console.log(
  "\nOpen LugiOS → Installed → Check for updates. Expect latest v0.1.0 > installed 0.0.1.",
);
console.log(`Data dir: ${dataRoot}`);
