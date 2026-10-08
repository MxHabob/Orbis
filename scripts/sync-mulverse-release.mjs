#!/usr/bin/env node
import { createHash } from "node:crypto";
import { createReadStream } from "node:fs";
import { readFile, readdir, stat } from "node:fs/promises";
import path from "node:path";

const version = process.env.RELEASE_VERSION ?? process.argv[2];
const artifactDir = process.env.RELEASE_ARTIFACT_DIR ?? process.argv[3];
const apiBaseUrl = process.env.MULVERSE_API_BASE_URL?.replace(/\/+$/, "");
const apiToken = process.env.MULVERSE_API_TOKEN;
const productId = process.env.MULVERSE_PRODUCT_ID;
const channelKey = process.env.MULVERSE_CHANNEL_STABLE_KEY ?? "stable";

if (!/^\d+\.\d+\.\d+$/.test(version ?? "")) {
  throw new Error(`Invalid release version: ${version}`);
}
if (!artifactDir) throw new Error("RELEASE_ARTIFACT_DIR is required.");
for (const [name, value] of Object.entries({
  MULVERSE_API_BASE_URL: apiBaseUrl,
  MULVERSE_API_TOKEN: apiToken,
  MULVERSE_PRODUCT_ID: productId,
})) {
  if (!value) throw new Error(`Missing required environment variable ${name}.`);
}

async function request(
  pathname,
  options = {},
  { expected = [200], retries = 3 } = {},
) {
  const url = `${apiBaseUrl}${pathname}`;
  let lastError;
  for (let attempt = 0; attempt <= retries; attempt += 1) {
    try {
      const response = await fetch(url, {
        ...options,
        signal: options.signal ?? AbortSignal.timeout(30 * 60 * 1000),
        headers: {
          Authorization: `Bearer ${apiToken}`,
          ...options.headers,
        },
      });
      if (expected.includes(response.status)) {
        if (response.status === 204) return null;
        return await response.json();
      }

      const body = await response.text();
      if (
        [429, 500, 502, 503, 504].includes(response.status) &&
        attempt < retries
      ) {
        await new Promise((resolve) => setTimeout(resolve, 500 * 2 ** attempt));
        continue;
      }
      const error = new Error(
        `Mulverse API ${options.method ?? "GET"} ${pathname} returned ${response.status}: ${body}`,
      );
      error.status = response.status;
      throw error;
    } catch (error) {
      if (error.status || attempt >= retries) throw error;
      lastError = error;
      await new Promise((resolve) => setTimeout(resolve, 500 * 2 ** attempt));
    }
  }
  throw lastError;
}

async function digestFile(filePath) {
  const hash = createHash("sha256");
  for await (const chunk of createReadStream(filePath)) hash.update(chunk);
  const fileStat = await stat(filePath);
  return { checksum: hash.digest("hex"), size: fileStat.size };
}

async function getUpdaterArtifact(globPattern, platform, architecture, label) {
  const names = await readdir(artifactDir);
  const archives = names.filter((name) => globPattern.test(name));
  if (archives.length !== 1) {
    throw new Error(
      `Expected exactly one ${label} updater archive; found ${archives.length}.`,
    );
  }

  const fileName = archives[0];
  const filePath = path.join(artifactDir, fileName);
  const signaturePath = `${filePath}.sig`;
  const signature = (await readFile(signaturePath, "utf8")).trim();
  if (!signature) throw new Error(`${label} updater signature is empty.`);
  const { checksum, size } = await digestFile(filePath);
  return {
    fileName,
    filePath,
    signature,
    checksum,
    size,
    platform,
    architecture,
    label,
  };
}

const updaterMetadata = JSON.parse(
  await readFile(path.join(artifactDir, "latest.json"), "utf8"),
);
if (updaterMetadata.version?.replace(/^v/, "") !== version) {
  throw new Error(
    `Updater metadata version ${updaterMetadata.version} does not match requested version ${version}.`,
  );
}
const releaseNotes = updaterMetadata.notes;
if (typeof releaseNotes !== "string" || !releaseNotes.trim()) {
  throw new Error("Updater metadata does not contain release notes.");
}

const artifacts = [
  await getUpdaterArtifact(
    /\.nsis\.zip$/i,
    "windows",
    "x86_64",
    "Windows NSIS",
  ),
  await getUpdaterArtifact(/\.app\.tar\.gz$/i, "macos", "aarch64", "macOS"),
];
for (const artifact of artifacts) {
  const expectedTarget =
    artifact.platform === "windows" ? "windows-x86_64" : "darwin-aarch64";
  if (
    updaterMetadata.platforms?.[expectedTarget]?.signature?.trim() !==
    artifact.signature
  ) {
    throw new Error(
      `Updater metadata has no signed manifest for ${expectedTarget}.`,
    );
  }
}

const channels = await request(`/api/v1/admin/products/${productId}/channels`);
const channel = channels.find(
  (candidate) => candidate.stable_key === channelKey,
);
if (!channel)
  throw new Error(`Mulverse channel '${channelKey}' does not exist.`);
if (channel.status !== "active")
  throw new Error(`Mulverse channel '${channelKey}' is not active.`);

async function listReleases() {
  return request(`/api/v1/admin/products/${productId}/releases`);
}

let releases = await listReleases();
let release = releases.find((candidate) => candidate.version === version);
if (release) {
  if (release.channel_id !== channel.id) {
    throw new Error(
      `Version ${version} already exists in a different Mulverse channel.`,
    );
  }
  if ((release.release_notes ?? "") !== releaseNotes) {
    throw new Error(
      `Version ${version} already exists with different release notes.`,
    );
  }
} else {
  try {
    release = await request(
      `/api/v1/admin/products/${productId}/releases`,
      {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          channel_id: channel.id,
          version,
          release_notes: releaseNotes,
          min_upgrade_version: null,
        }),
      },
      { expected: [201] },
    );
  } catch (error) {
    if (error.status !== 409) throw error;
    releases = await listReleases();
    release = releases.find((candidate) => candidate.version === version);
    if (!release || release.channel_id !== channel.id) throw error;
    if ((release.release_notes ?? "") !== releaseNotes) {
      throw new Error(
        `Version ${version} already exists with different release notes.`,
      );
    }
  }
}
if (release.status === "withdrawn") {
  throw new Error(
    `Mulverse release ${version} is withdrawn and cannot be republished.`,
  );
}
if (release.version !== version || release.channel_id !== channel.id) {
  throw new Error(`Mulverse returned inconsistent metadata for release ${version}.`);
}

let registeredArtifacts = await request(
  `/api/v1/admin/releases/${release.id}/artifacts`,
);
function matchesArtifact(record, artifact) {
  return (
    record.file_name === artifact.fileName &&
    record.platform === artifact.platform &&
    record.architecture === artifact.architecture &&
    record.checksum_sha256.toLowerCase() === artifact.checksum &&
    Number(record.file_size_bytes) === artifact.size &&
    record.signature === artifact.signature
  );
}

for (const artifact of artifacts) {
  const existing = registeredArtifacts.find(
    (candidate) =>
      candidate.platform === artifact.platform &&
      candidate.architecture === artifact.architecture,
  );
  if (existing) {
    if (!matchesArtifact(existing, artifact)) {
      throw new Error(
        `Existing Mulverse artifact ${artifact.platform}-${artifact.architecture} does not match the GitHub release.`,
      );
    }
    console.log(`Reusing verified ${artifact.label} artifact ${existing.id}.`);
    continue;
  }

  const signatureBase64 = Buffer.from(artifact.signature, "utf8").toString(
    "base64",
  );
  const registered = await request(
    `/api/v1/admin/releases/${release.id}/artifacts/${artifact.platform}/${artifact.architecture}`,
    {
      method: "PUT",
      headers: {
        "Content-Type": "application/octet-stream",
        "x-file-name": artifact.fileName,
        "x-checksum-sha256": artifact.checksum,
        "x-signature-base64": signatureBase64,
      },
      body: createReadStream(artifact.filePath),
      duplex: "half",
    },
    { expected: [200, 201], retries: 0 },
  );
  if (
    registered.file_name !== artifact.fileName ||
    registered.platform !== artifact.platform ||
    registered.architecture !== artifact.architecture ||
    registered.checksum_sha256?.toLowerCase() !== artifact.checksum ||
    Number(registered.file_size_bytes) !== artifact.size ||
    registered.signature !== artifact.signature
  ) {
    throw new Error(
      `Mulverse did not confirm the uploaded ${artifact.label} artifact.`,
    );
  }
  registeredArtifacts.push(registered);
  console.log(
    `Uploaded and verified ${artifact.label} artifact ${registered.id}.`,
  );
}

registeredArtifacts = await request(
  `/api/v1/admin/releases/${release.id}/artifacts`,
);
for (const artifact of artifacts) {
  const registered = registeredArtifacts.find(
    (candidate) =>
      candidate.platform === artifact.platform &&
      candidate.architecture === artifact.architecture,
  );
  if (!registered || !matchesArtifact(registered, artifact)) {
    throw new Error(
      `Mulverse did not persist the expected ${artifact.label} artifact.`,
    );
  }
}

if (release.status !== "published") {
  const published = await request(
    `/api/v1/admin/releases/${release.id}/publish`,
    { method: "POST" },
    { expected: [200] },
  );
  if (published.status !== "published") {
    throw new Error(`Mulverse did not publish version ${version}.`);
  }
}

console.log(`Published Orbis ${version} to Mulverse channel '${channelKey}'.`);
