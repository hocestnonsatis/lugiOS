/**
 * Writes path-only SVGs (no fonts) for Tauri `icon` and portable wordmarks.
 * Tauri's rasterizer skips @font-face; outlined paths keep DM Sans 600 shapes.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import opentype from "opentype.js";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const fontPath = path.join(root, "branding", "fonts", "dm-sans-latin-600-normal.woff");

const fontBuf = fs.readFileSync(fontPath);
const font = opentype.parse(new Uint8Array(fontBuf).buffer);

function centerFullStringBaseline(fontSize, text) {
  const p = font.getPath(text, 0, 0, fontSize);
  const bb = p.getBoundingBox();
  const ox = (1024 - (bb.x2 - bb.x1)) / 2 - bb.x1;
  const oy = (1024 - (bb.y2 - bb.y1)) / 2 - bb.y1;
  return { ox, oy, bb };
}

/** App icon: 1024 square, dark bg */
function writeAppIcon() {
  const fontSize = 200;
  const { ox, oy } = centerFullStringBaseline(fontSize, "LugiOS");
  const lugiAdvance = font.getAdvanceWidth("Lugi", fontSize);
  const lugiD = font.getPath("Lugi", ox, oy, fontSize).toPathData(2);
  const osD = font.getPath("OS", ox + lugiAdvance, oy, fontSize).toPathData(2);

  const svg = `<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1024 1024" width="1024" height="1024" role="img" aria-label="LugiOS">
  <rect width="1024" height="1024" fill="#0c0f14"/>
  <path fill="#f1f5f9" d="${lugiD}"/>
  <path fill="#3b82f6" d="${osD}"/>
</svg>
`;
  const out = path.join(root, "branding", "lugios-app-icon.svg");
  fs.writeFileSync(out, svg, "utf8");
  return out;
}

/** Horizontal wordmark */
function writeWordmark() {
  const fontSize = 36;
  const p = font.getPath("LugiOS", 0, 0, fontSize);
  const bb = p.getBoundingBox();
  const pad = 4;
  const w = Math.ceil(bb.x2 - bb.x1 + pad * 2);
  const h = Math.ceil(bb.y2 - bb.y1 + pad * 2);
  const ox = pad - bb.x1;
  const oy = pad - bb.y1;
  const lugiAdvance = font.getAdvanceWidth("Lugi", fontSize);
  const lugiD = font.getPath("Lugi", ox, oy, fontSize).toPathData(2);
  const osD = font.getPath("OS", ox + lugiAdvance, oy, fontSize).toPathData(2);

  const svg = `<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 ${w} ${h}" width="${w}" height="${h}" role="img" aria-label="LugiOS">
  <path fill="#f1f5f9" d="${lugiD}"/>
  <path fill="#3b82f6" d="${osD}"/>
</svg>
`;
  const out = path.join(root, "branding", "lugios-wordmark.svg");
  fs.writeFileSync(out, svg, "utf8");
  return out;
}

const a = writeAppIcon();
const b = writeWordmark();
process.stdout.write(`Wrote ${path.relative(root, a)}\nWrote ${path.relative(root, b)}\n`);
