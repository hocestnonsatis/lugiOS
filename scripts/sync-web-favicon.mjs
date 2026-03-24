import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
fs.copyFileSync(
  path.join(root, "src-tauri", "icons", "32x32.png"),
  path.join(root, "static", "favicon.png"),
);
