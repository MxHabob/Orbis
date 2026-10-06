#!/usr/bin/env node
import fs from "node:fs";
import path from "node:path";
import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";

const rootDir = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const tag = process.argv[2] ?? process.env.GITHUB_REF_NAME;
if (!/^v\d+\.\d+\.\d+$/.test(tag ?? "")) throw new Error(`Invalid release tag: ${tag}`);
const assetRoot = path.join(rootDir, "release-assets", tag);
const files = [];

function visit(directory) {
  if (!fs.existsSync(directory)) return;
  for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
    if (entry.name === ".gitkeep") continue;
    const fullPath = path.join(directory, entry.name);
    if (entry.isDirectory()) visit(fullPath);
    else files.push(fullPath);
  }
}

visit(assetRoot);
if (!files.length) {
  console.log(`No additional release assets to upload for ${tag}.`);
  process.exit(0);
}

const basenames = files.map((file) => path.basename(file).toLowerCase());
if (new Set(basenames).size !== basenames.length) {
  throw new Error("Release asset filenames must be unique because GitHub Release assets share one flat namespace.");
}

const result = spawnSync("gh", ["release", "upload", tag, "--clobber", ...files], {
  cwd: rootDir,
  env: process.env,
  stdio: "inherit",
});
if (result.error) throw result.error;
if (result.status !== 0) process.exit(result.status ?? 1);
