#!/usr/bin/env node
import fs from "node:fs";
import path from "node:path";
import { execFileSync } from "node:child_process";
import { fileURLToPath } from "node:url";

const rootDir = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const paths = {
  desktopPackage: "apps/desktop/package.json",
  cargo: "apps/desktop/src-tauri/Cargo.toml",
  solver: "apps/solver-service/pom.xml",
  tauri: "apps/desktop/src-tauri/tauri.conf.json",
  staging: "apps/desktop/src-tauri/tauri.staging.conf.json",
  lockfile: "bun.lock",
};

function absolute(relativePath) {
  return path.join(rootDir, relativePath);
}

function read(relativePath) {
  return fs.readFileSync(absolute(relativePath), "utf8");
}

function parseSemver(value) {
  const match = /^v?(\d+)\.(\d+)\.(\d+)$/.exec(value);
  if (!match) throw new Error(`Expected a stable SemVer version (x.y.z), received "${value}".`);
  return match.slice(1).map(Number);
}

function versionFromCargo(source) {
  const match = /^\[package\][\s\S]*?^version\s*=\s*"([^"]+)"/m.exec(source);
  if (!match) throw new Error("Could not read [package].version from desktop Cargo.toml.");
  return match[1];
}

function versionFromLockfile(source) {
  const match = /"apps\/desktop":\s*\{\s*"name":\s*"@orbis\/desktop",\s*"version":\s*"([^"]+)"/.exec(source);
  if (!match) throw new Error("Could not find the @orbis/desktop workspace version in bun.lock.");
  return match[1];
}

function replaceVersion(source, expression, version, label) {
  if (!expression.test(source)) throw new Error(`Could not find ${label} version field.`);
  return source.replace(expression, `$1${version}$2`);
}

function bump(currentVersion, level) {
  const [major, minor, patch] = parseSemver(currentVersion);
  if (level === "major") return `${major + 1}.0.0`;
  if (level === "minor") return `${major}.${minor + 1}.0`;
  if (level === "patch") return `${major}.${minor}.${patch + 1}`;
  throw new Error(`Unknown bump type "${level}". Use patch, minor, major, or an explicit x.y.z version.`);
}

function compareVersions(left, right) {
  const a = parseSemver(left);
  const b = parseSemver(right);
  for (let index = 0; index < a.length; index += 1) {
    if (a[index] !== b[index]) return a[index] - b[index];
  }
  return 0;
}

function latestTaggedVersion() {
  const tags = execFileSync("git", ["tag", "--list", "v[0-9]*", "--sort=-version:refname"], {
    cwd: rootDir,
    encoding: "utf8",
  });
  return tags.split(/\r?\n/).map((tag) => tag.trim()).find((tag) => /^v\d+\.\d+\.\d+$/.test(tag))?.slice(1) ?? null;
}

function main() {
  const args = process.argv.slice(2);
  const dryRun = args.includes("--dry-run");
  const requested = args.find((arg) => !arg.startsWith("--")) ?? "patch";
  const pkg = JSON.parse(read(paths.desktopPackage));
  const currentVersion = pkg.version;
  const latestTag = latestTaggedVersion();
  const baselineVersion = latestTag && compareVersions(latestTag, currentVersion) > 0
    ? latestTag
    : currentVersion;
  const nextVersion = ["patch", "minor", "major"].includes(requested)
    ? bump(baselineVersion, requested)
    : requested.replace(/^v/, "");
  parseSemver(nextVersion);

  if (compareVersions(nextVersion, baselineVersion) <= 0) {
    throw new Error(`The new version ${nextVersion} must be greater than the current/latest tagged version ${baselineVersion}.`);
  }
  const existingTag = execFileSync("git", ["tag", "--list", `v${nextVersion}`], {
    cwd: rootDir,
    encoding: "utf8",
  }).trim();
  if (existingTag) throw new Error(`Release tag v${nextVersion} already exists.`);

  const original = {
    package: read(paths.desktopPackage),
    cargo: read(paths.cargo),
    solver: read(paths.solver),
    tauri: read(paths.tauri),
    staging: read(paths.staging),
    lockfile: read(paths.lockfile),
  };
  const versions = {
    package: pkg.version,
    cargo: versionFromCargo(original.cargo),
    solver: original.solver.match(/<orbis\.version>([^<]+)<\/orbis\.version>/)?.[1],
    tauri: JSON.parse(original.tauri).version,
    staging: JSON.parse(original.staging).version,
    lockfile: versionFromLockfile(original.lockfile),
  };
  if (Object.values(versions).some((version) => version !== currentVersion)) {
    throw new Error(`Version files are inconsistent: ${JSON.stringify(versions)}. Align them before bumping.`);
  }

  const updated = {
    package: `${JSON.stringify({ ...pkg, version: nextVersion }, null, 2)}\n`,
    cargo: replaceVersion(original.cargo, /(^\[package\][\s\S]*?^version\s*=\s*")([^"]+)(")/m, nextVersion, "Cargo package"),
    solver: replaceVersion(original.solver, /(<orbis\.version>)([^<]+)(<\/orbis\.version>)/, nextVersion, "Orbis solver"),
    tauri: `${JSON.stringify({ ...JSON.parse(original.tauri), version: nextVersion }, null, 2)}\n`,
    staging: `${JSON.stringify({ ...JSON.parse(original.staging), version: nextVersion }, null, 2)}\n`,
    lockfile: replaceVersion(
      original.lockfile,
      /("apps\/desktop":\s*\{\s*"name":\s*"@orbis\/desktop",\s*"version":\s*")([^"]+)(")/,
      nextVersion,
      "Bun lockfile workspace",
    ),
  };

  if (dryRun) {
    console.log(`Planned version bump: ${currentVersion} -> ${nextVersion}${latestTag ? ` (latest repository tag: v${latestTag})` : ""}`);
    for (const relativePath of Object.values(paths)) console.log(`- ${relativePath}`);
    return;
  }

  for (const [key, relativePath] of Object.entries(paths)) {
    fs.writeFileSync(absolute(relativePath), updated[key]);
  }
  console.log(`Updated the Orbis release version to ${nextVersion} in all manifests and the Bun lockfile.`);
}

try {
  main();
} catch (error) {
  console.error(error instanceof Error ? error.message : String(error));
  process.exit(1);
}
