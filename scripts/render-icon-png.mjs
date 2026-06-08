#!/usr/bin/env node
import { deflateSync } from "node:zlib";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const outputDir = process.argv[2];
if (!outputDir) {
  console.error("Usage: render-icon-png.mjs <iconset-dir>");
  process.exit(1);
}

mkdirSync(outputDir, { recursive: true });

const sizes = [
  ["icon_16x16.png", 16],
  ["icon_16x16@2x.png", 32],
  ["icon_32x32.png", 32],
  ["icon_32x32@2x.png", 64],
  ["icon_128x128.png", 128],
  ["icon_128x128@2x.png", 256],
  ["icon_256x256.png", 256],
  ["icon_256x256@2x.png", 512],
  ["icon_512x512.png", 512],
  ["icon_512x512@2x.png", 1024],
];

for (const [name, size] of sizes) {
  writeFileSync(join(outputDir, name), png(size));
}

function png(size) {
  const channels = 4;
  const rows = [];
  const center = (size - 1) / 2;
  const radius = size * 0.31;
  const ringWidth = Math.max(2, size * 0.075);

  for (let y = 0; y < size; y += 1) {
    const row = Buffer.alloc(1 + size * channels);
    row[0] = 0;
    for (let x = 0; x < size; x += 1) {
      const offset = 1 + x * channels;
      const dx = x - center;
      const dy = y - center;
      const dist = Math.sqrt(dx * dx + dy * dy);
      const ring = Math.abs(dist - radius) <= ringWidth;
      const shaft =
        x >= size * 0.34 &&
        x <= size * 0.72 &&
        y >= size * 0.45 &&
        y <= size * 0.56;
      const head =
        x >= size * 0.58 &&
        x <= size * 0.78 &&
        Math.abs(y - center) <= (size * 0.78 - x) * 0.78 + size * 0.055;

      if (ring || shaft || head) {
        const shade = Math.max(0, Math.min(1, (x + y) / (size * 2)));
        row[offset] = Math.round(25 + 35 * shade);
        row[offset + 1] = Math.round(122 + 44 * shade);
        row[offset + 2] = Math.round(190 + 36 * shade);
        row[offset + 3] = 255;
      }
    }
    rows.push(row);
  }

  const raw = Buffer.concat(rows);
  const chunks = [
    chunk("IHDR", ihdr(size, size)),
    chunk("IDAT", deflateSync(raw, { level: 9 })),
    chunk("IEND", Buffer.alloc(0)),
  ];

  return Buffer.concat([Buffer.from("89504e470d0a1a0a", "hex"), ...chunks]);
}

function ihdr(width, height) {
  const data = Buffer.alloc(13);
  data.writeUInt32BE(width, 0);
  data.writeUInt32BE(height, 4);
  data[8] = 8;
  data[9] = 6;
  data[10] = 0;
  data[11] = 0;
  data[12] = 0;
  return data;
}

function chunk(type, data) {
  const typeBuffer = Buffer.from(type);
  const length = Buffer.alloc(4);
  length.writeUInt32BE(data.length, 0);
  const crc = Buffer.alloc(4);
  crc.writeUInt32BE(crc32(Buffer.concat([typeBuffer, data])), 0);
  return Buffer.concat([length, typeBuffer, data, crc]);
}

function crc32(buffer) {
  let crc = 0xffffffff;
  for (const byte of buffer) {
    crc ^= byte;
    for (let bit = 0; bit < 8; bit += 1) {
      crc = (crc >>> 1) ^ (0xedb88320 & -(crc & 1));
    }
  }
  return (crc ^ 0xffffffff) >>> 0;
}
