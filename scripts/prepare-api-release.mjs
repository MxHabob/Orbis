#!/usr/bin/env node
import { createHash } from "node:crypto";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";

const rootDir = path.resolve(
  path.dirname(fileURLToPath(import.meta.url)),
  "..",
);
const version = process.env.RELEASE_VERSION;
const tag = process.env.RELEASE_TAG;
const dockerDigest = process.env.DOCKER_IMAGE_DIGEST;
const privateKey = process.env.TAURI_SIGNING_PRIVATE_KEY;

if (!/^\d+\.\d+\.\d+$/.test(version ?? "") || tag !== `v${version}`) {
  throw new Error(`Invalid API release version or tag: ${version} (${tag}).`);
}
if (!/^sha256:[a-f0-9]{64}$/.test(dockerDigest ?? "")) {
  throw new Error(`Invalid Docker image digest: ${dockerDigest}.`);
}
if (!privateKey) {
  throw new Error(
    "TAURI_SIGNING_PRIVATE_KEY is required to sign the API manifest.",
  );
}

const outputDir = path.resolve(
  rootDir,
  process.env.API_RELEASE_OUTPUT_DIR ?? "release-output",
);
fs.mkdirSync(outputDir, { recursive: true });

const dockerStage = fs.mkdtempSync(path.join(os.tmpdir(), "orbis-api-docker-"));
const dockerFiles = [
  "compose.yaml",
  ".env.example",
  "server.example.json",
  "README.md",
];
for (const name of dockerFiles) {
  const source = path.join(
    rootDir,
    "apps",
    "api-rust",
    "deploy",
    "docker",
    name,
  );
  const destination = path.join(dockerStage, name);
  if (name === ".env.example") {
    fs.writeFileSync(
      destination,
      fs
        .readFileSync(source, "utf8")
        .replace(
          /^ORBIS_API_IMAGE=.*$/m,
          `ORBIS_API_IMAGE=ghcr.io/mxhabob/orbis-api@${dockerDigest}`,
        ),
    );
  } else {
    fs.copyFileSync(source, destination);
  }
}

const dockerBundle = `Orbis-API-${version}-docker-bundle.zip`;
const zip = spawnSync(
  "zip",
  [
    "-j",
    "-q",
    path.join(outputDir, dockerBundle),
    ...dockerFiles.map((name) => path.join(dockerStage, name)),
  ],
  { cwd: rootDir, stdio: "inherit" },
);
if (zip.error) throw zip.error;
if (zip.status !== 0)
  throw new Error("Could not create the Docker Compose release bundle.");
fs.rmSync(dockerStage, { recursive: true, force: true });

const packages = [
  ["linux-x86_64", `Orbis-API-${version}-linux-x86_64.tar.gz`],
  ["linux-aarch64", `Orbis-API-${version}-linux-aarch64.tar.gz`],
  ["windows-x86_64", `Orbis-API-${version}-windows-x86_64.zip`],
  ["macos-aarch64", `Orbis-API-${version}-macos-aarch64.tar.gz`],
  ["docker-bundle", dockerBundle],
];
const artifacts = packages.map(([platform, name]) => {
  const file = path.join(outputDir, name);
  if (!fs.statSync(file, { throwIfNoEntry: false })?.isFile()) {
    throw new Error(`Missing API release package: ${name}.`);
  }
  const sha256 = createHash("sha256")
    .update(fs.readFileSync(file))
    .digest("hex");
  return {
    name,
    platform,
    url: `https://github.com/MxHabob/Orbis/releases/download/${tag}/${name}`,
    sha256,
  };
});

const manifest = {
  schema_version: 1,
  version,
  tag,
  artifacts,
  docker_image: `ghcr.io/mxhabob/orbis-api@${dockerDigest}`,
  docker_digest: dockerDigest,
};
const manifestPath = path.join(outputDir, "orbis-api-manifest.json");
fs.writeFileSync(manifestPath, `${JSON.stringify(manifest, null, 2)}\n`);

const sign = spawnSync(
  "bun",
  ["run", "--cwd", "apps/desktop", "tauri", "signer", "sign", manifestPath],
  {
    cwd: rootDir,
    env: process.env,
    stdio: "inherit",
  },
);
if (sign.error) throw sign.error;
if (sign.status !== 0) process.exit(sign.status ?? 1);
if (!fs.statSync(`${manifestPath}.sig`, { throwIfNoEntry: false })?.isFile()) {
  throw new Error(
    "Tauri signer did not create the API release manifest signature.",
  );
}
