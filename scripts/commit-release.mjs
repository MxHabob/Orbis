#!/usr/bin/env node
import fs from "node:fs";
import path from "node:path";
import { execFileSync } from "node:child_process";
import { fileURLToPath } from "node:url";
import { validateReleaseNotes } from "./release-notes-validation.mjs";

const rootDir = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

function git(args, options = {}) {
  return execFileSync("git", args, {
    cwd: rootDir,
    encoding: "utf8",
    stdio: options.stdio ?? "pipe",
  }).trim();
}

function main() {
  const { version } = JSON.parse(fs.readFileSync(path.join(rootDir, "apps/desktop/package.json"), "utf8"));
  const tag = `v${version}`;
  const notesPath = path.join(rootDir, "releases", `${tag}.md`);
  const notes = fs.readFileSync(notesPath, "utf8");
  validateReleaseNotes(notes, `Release notes at ${path.relative(rootDir, notesPath)}`);
  if (git(["tag", "--list", tag])) throw new Error(`Tag ${tag} already exists.`);
  if (git(["diff", "--cached", "--name-only"])) {
    throw new Error("The Git index has staged changes. Commit or unstage them before preparing a release commit.");
  }

  const assetDir = path.join(rootDir, "release-assets", tag);
  const filenames = [];
  function collectFiles(directory) {
    for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
      const fullPath = path.join(directory, entry.name);
      if (entry.isDirectory()) collectFiles(fullPath);
      else filenames.push(path.relative(rootDir, fullPath));
    }
  }
  collectFiles(assetDir);

  const basenames = filenames
    .filter((filename) => path.basename(filename) !== ".gitkeep")
    .map((filename) => path.basename(filename).toLowerCase());
  if (new Set(basenames).size !== basenames.length) {
    throw new Error("Release asset filenames must be unique because GitHub Release assets share one flat namespace.");
  }

  const files = [
    "apps/desktop/package.json",
    "apps/desktop/src-tauri/Cargo.toml",
    "apps/desktop/src-tauri/tauri.conf.json",
    "apps/desktop/src-tauri/tauri.staging.conf.json",
    "bun.lock",
    path.relative(rootDir, notesPath),
    ...filenames,
  ];
  git(["add", "--", ...files]);
  git([
    "commit",
    "-m",
    `chore(release): ${tag}`,
    "-m",
    "Co-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>",
  ], { stdio: "inherit" });
  git(["tag", "-a", tag, "-m", `Orbis ${tag}`], { stdio: "inherit" });
  console.log(`Created release commit and annotated tag ${tag}. Push the commit and tag to trigger CI.`);
}

try {
  main();
} catch (error) {
  console.error(error instanceof Error ? error.message : String(error));
  process.exit(1);
}
