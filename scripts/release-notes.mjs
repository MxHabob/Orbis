#!/usr/bin/env node
import fs from "node:fs";
import path from "node:path";
import { execFileSync } from "node:child_process";
import { fileURLToPath } from "node:url";

const rootDir = path.resolve(
  path.dirname(fileURLToPath(import.meta.url)),
  "..",
);
const desktopPackagePath = path.join(rootDir, "apps/desktop/package.json");
const releasesDir = path.join(rootDir, "releases");
const assetsRoot = path.join(rootDir, "release-assets");

function git(args) {
  return execFileSync("git", args, { cwd: rootDir, encoding: "utf8" });
}

function latestTag() {
  try {
    return (
      git([
        "tag",
        "--merged",
        "HEAD",
        "--list",
        "v[0-9]*",
        "--sort=-version:refname",
      ])
        .split(/\r?\n/)
        .map((tag) => tag.trim())
        .find(Boolean) ?? null
    );
  } catch {
    return null;
  }
}

function getCommits() {
  const tag = latestTag();
  const args = ["log", "--no-merges", "--format=%s%x1f%b%x1e"];
  if (tag) args.splice(1, 0, `${tag}..HEAD`);
  else args.splice(1, 0, "-n", "100");

  return git(args)
    .split("\x1e")
    .map((record) => record.trim())
    .filter(Boolean)
    .map((record) => {
      const [subject = "", ...body] = record.split("\x1f");
      return { subject: subject.trim(), body: body.join("\x1f").trim() };
    });
}

function classify({ subject, body }) {
  const match =
    /^(feat|fix|perf|refactor|docs|build|ci|test|chore)(?:\(([^)]+)\))?(!)?:\s*(.+)$/i.exec(
      subject,
    );
  const breaking = Boolean(match?.[3]) || /BREAKING CHANGE:/i.test(body);
  const category = breaking
    ? "breaking"
    : (match?.[1].toLowerCase() ?? "other");
  const scope = match?.[2] ? ` (${match[2]})` : "";
  const text = match?.[4] ?? subject;
  return { category, entry: `- ${text}${scope}` };
}

function readAssets(version) {
  const versionDir = path.join(assetsRoot, `v${version}`);
  const imagesDir = path.join(versionDir, "images");
  const resourcesDir = path.join(versionDir, "resources");
  fs.mkdirSync(imagesDir, { recursive: true });
  fs.mkdirSync(resourcesDir, { recursive: true });
  for (const folder of [imagesDir, resourcesDir]) {
    const keepFile = path.join(folder, ".gitkeep");
    if (!fs.existsSync(keepFile)) fs.writeFileSync(keepFile, "");
  }
  return { versionDir };
}

function createNotes(version, commits) {
  const buckets = new Map([
    ["breaking", "Breaking Changes"],
    ["feat", "Added"],
    ["fix", "Fixed"],
    ["perf", "Performance"],
    ["refactor", "Changed"],
    ["docs", "Documentation"],
    ["build", "Build"],
    ["ci", "CI"],
    ["test", "Tests"],
    ["chore", "Maintenance"],
    ["other", "Other"],
  ]);
  const grouped = new Map([...buckets.keys()].map((key) => [key, []]));
  for (const commit of commits) {
    const result = classify(commit);
    grouped.get(result.category).push(result.entry);
  }

  const lines = [
    `# Orbis v${version}`,
    "",
    "## Summary",
    "<!-- Replace this instruction with a concise, user-facing summary before committing the release. -->",
    "",
  ];

  for (const [key, heading] of buckets) {
    const entries = grouped.get(key);
    if (entries.length) lines.push(`## ${heading}`, "", ...entries, "");
  }
  if (!commits.length)
    lines.push(
      "## Changes",
      "",
      "- No commits were found since the previous version tag; add the user-facing changes manually.",
      "",
    );

  lines.push(
    "## Known Issues",
    "",
    '<!-- Write "None" or list user-relevant known issues. -->',
    "",
  );
  lines.push(
    "## Release Assets",
    "",
    "Version-specific images and supporting resources are attached to this GitHub Release. Images are also shown in the app's release history when available.",
    "",
  );
  return lines.join("\n");
}

function main() {
  const force = process.argv.includes("--force");
  const { version } = JSON.parse(fs.readFileSync(desktopPackagePath, "utf8"));
  const notesPath = path.join(releasesDir, `v${version}.md`);
  const assets = readAssets(version);
  fs.mkdirSync(releasesDir, { recursive: true });

  if (fs.existsSync(notesPath) && !force) {
    console.log(
      `Release notes already exist; left unchanged: ${path.relative(rootDir, notesPath)}`,
    );
    return;
  }

  const notes = createNotes(version, getCommits());
  fs.writeFileSync(notesPath, notes);
  console.log(
    `Generated editable release notes: ${path.relative(rootDir, notesPath)}`,
  );
  console.log(
    `Add screenshots to ${path.relative(rootDir, path.join(assets.versionDir, "images"))}`,
  );
  console.log(
    `Add supporting files to ${path.relative(rootDir, path.join(assets.versionDir, "resources"))}`,
  );
}

try {
  main();
} catch (error) {
  console.error(error instanceof Error ? error.message : String(error));
  process.exit(1);
}
