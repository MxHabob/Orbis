#!/usr/bin/env node
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const targetPlatforms = {
  "x86_64-pc-windows-msvc": {
    updaterPlatform: "windows-x86_64",
    artifactPattern: /-setup\.exe$/,
  },
  "aarch64-apple-darwin": {
    updaterPlatform: "darwin-aarch64",
    artifactPattern: /\.app\.tar\.gz$/,
  },
};

export function createUpdaterManifest({
  repository,
  tag,
  version,
  target,
  artifactName,
  signature,
  notes,
}) {
  const platform = targetPlatforms[target];
  if (!platform) {
    throw new Error(`Unsupported Tauri updater target: ${target}.`);
  }
  if (!/^[^/]+\/[^/]+$/.test(repository ?? "")) {
    throw new Error(`Invalid GitHub repository name: ${repository}.`);
  }
  if (!/^v\d+\.\d+\.\d+$/.test(tag ?? "") || tag !== `v${version}`) {
    throw new Error(`Invalid release tag or version: ${tag} (${version}).`);
  }
  if (
    typeof artifactName !== "string" ||
    !platform.artifactPattern.test(artifactName) ||
    /[\\/]/.test(artifactName)
  ) {
    throw new Error(
      `Invalid updater artifact name for ${platform.updaterPlatform}.`,
    );
  }
  if (typeof signature !== "string" || !signature.trim()) {
    throw new Error(`Updater signature is missing for ${artifactName}.`);
  }
  if (typeof notes !== "string" || !notes.trim()) {
    throw new Error(`Release notes are missing for ${tag}.`);
  }

  const artifactUrl = new URL(
    encodeURIComponent(artifactName),
    `https://github.com/${repository}/releases/download/${tag}/`,
  ).toString();
  const normalizedNotes = notes.replace(/\r\n?/g, "\n").trim();

  return {
    version,
    notes: normalizedNotes,
    platforms: {
      [platform.updaterPlatform]: {
        url: artifactUrl,
        signature: signature.trim(),
      },
    },
  };
}

if (
  process.argv[1] &&
  path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)
) {
  const target = process.env.TAURI_UPDATER_TARGET;
  const platform = targetPlatforms[target];
  if (!platform) {
    throw new Error(
      `Unsupported Tauri updater target: ${target ?? "(missing)"}.`,
    );
  }

  const artifactDir = path.resolve(process.env.CI_ARTIFACT_DIR ?? "ci-artifacts");
  const artifactNames = fs
    .readdirSync(artifactDir, { withFileTypes: true })
    .filter(
      (entry) =>
        entry.isFile() && platform.artifactPattern.test(entry.name),
    )
    .map((entry) => entry.name);
  if (artifactNames.length !== 1) {
    throw new Error(
      `Expected one ${platform.updaterPlatform} updater artifact in ${artifactDir}; found ${artifactNames.length}.`,
    );
  }

  const [artifactName] = artifactNames;
  const signaturePath = path.join(artifactDir, `${artifactName}.sig`);
  if (!fs.statSync(signaturePath, { throwIfNoEntry: false })?.isFile()) {
    throw new Error(`Updater signature is missing: ${signaturePath}.`);
  }
  const releaseNotesPath = path.resolve(
    process.env.RELEASE_NOTES_PATH ??
      path.join(process.cwd(), "releases", `${process.env.RELEASE_TAG}.md`),
  );
  if (!fs.statSync(releaseNotesPath, { throwIfNoEntry: false })?.isFile()) {
    throw new Error(`Release notes file is missing: ${releaseNotesPath}.`);
  }

  const manifest = createUpdaterManifest({
    repository: process.env.GITHUB_REPOSITORY,
    tag: process.env.RELEASE_TAG,
    version: process.env.RELEASE_VERSION,
    target,
    artifactName,
    signature: fs.readFileSync(signaturePath, "utf8"),
    notes: fs.readFileSync(releaseNotesPath, "utf8"),
  });
  fs.writeFileSync(
    path.join(artifactDir, "latest.json"),
    `${JSON.stringify(manifest, null, 2)}\n`,
    { flag: "wx" },
  );
  console.log(`Created updater metadata for ${target}.`);
}
