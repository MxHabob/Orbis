#!/usr/bin/env node
import { execFileSync } from "node:child_process";
import path from "node:path";
import { fileURLToPath } from "node:url";

const rootDir = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const scriptsDir = path.join(rootDir, "scripts");
const args = process.argv.slice(2);

function run(script, forwardedArgs) {
  execFileSync(process.execPath, [path.join(scriptsDir, script), ...forwardedArgs], {
    cwd: rootDir,
    stdio: "inherit",
  });
}

function main() {
  if (args.some((arg) => arg.startsWith("--") && arg !== "--dry-run")) {
    throw new Error("Supported option: --dry-run. Otherwise provide patch, minor, major, or a SemVer version.");
  }
  const versionArgs = args.filter((arg) => arg !== "--dry-run");
  if (versionArgs.length > 1) throw new Error("Provide one version bump type or one explicit SemVer version.");

  if (args.includes("--dry-run")) {
    run("release-version.mjs", [...versionArgs, "--dry-run"]);
    return;
  }

  const status = execFileSync("git", ["status", "--porcelain"], { cwd: rootDir, encoding: "utf8" });
  if (status.trim()) {
    throw new Error("Release preparation requires a clean working tree. Commit or stash current changes first.");
  }

  run("release-version.mjs", versionArgs);
  run("release-notes.mjs", []);
  console.log("Review and complete releases/vX.Y.Z.md, add versioned images/resources, then run bun run release:commit.");
}

try {
  main();
} catch (error) {
  console.error(error instanceof Error ? error.message : String(error));
  process.exit(1);
}
