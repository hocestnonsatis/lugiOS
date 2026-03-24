/**
 * Zip fixtures/<name>/ → fixtures/<slug>-dist.zip (slug = name without optional "-app" suffix).
 * Usage: node scripts/package-fixture.mjs demo-hello-app
 */
import { createWriteStream } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import archiver from "archiver";

const fixture = process.argv[2];
if (!fixture) {
  console.error("Usage: node scripts/package-fixture.mjs <folder-under-fixtures/>");
  process.exit(1);
}

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const slug = fixture.replace(/-app$/u, "");
const srcDir = path.join(root, "fixtures", fixture);
const outZip = path.join(root, "fixtures", `${slug}-dist.zip`);

const output = createWriteStream(outZip);
const archive = archiver("zip", { zlib: { level: 9 } });

await new Promise((resolve, reject) => {
  output.on("close", resolve);
  archive.on("error", reject);
  archive.pipe(output);
  archive.directory(srcDir, false);
  archive.finalize();
});

console.log(`Wrote ${path.relative(root, outZip)} (${archive.pointer()} bytes)`);
