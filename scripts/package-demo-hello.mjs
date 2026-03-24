/**
 * Builds fixtures/demo-hello-dist.zip for GitHub Releases (host installer expects .zip/.tar.gz asset).
 */
import { createWriteStream } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import archiver from "archiver";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const srcDir = path.join(root, "fixtures", "demo-hello-app");
const outZip = path.join(root, "fixtures", "demo-hello-dist.zip");

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
