#!/usr/bin/env node
import { createHash } from "node:crypto";
import { readFileSync, statSync, writeFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const MAX_ARTIFACT_BYTES = 512 * 1024 * 1024;
const PLATFORMS = [
  ["x86_64-pc-windows-msvc", ".exe"],
  ["aarch64-apple-darwin", ""],
];

export function createComponentPayload({
  version,
  artifactDirectory,
  artifactBaseUrl,
}) {
  if (!/^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)$/.test(version ?? "")) {
    throw new Error("Component version must use stable MAJOR.MINOR.PATCH SemVer.");
  }
  const baseUrl = new URL(artifactBaseUrl);
  if (
    baseUrl.protocol !== "https:" ||
    baseUrl.username ||
    baseUrl.password ||
    baseUrl.search ||
    baseUrl.hash ||
    !baseUrl.pathname.endsWith("/")
  ) {
    throw new Error("Artifact base URL must be an HTTPS directory URL.");
  }

  const platforms = Object.fromEntries(
    PLATFORMS.map(([target, extension]) => {
      const fileName = `orbis-solver-${version}-${target}${extension}`;
      const filePath = path.join(artifactDirectory, fileName);
      const stats = statSync(filePath);
      if (!stats.isFile() || stats.size <= 0 || stats.size > MAX_ARTIFACT_BYTES) {
        throw new Error(`Solver artifact ${fileName} is missing, empty, or too large.`);
      }
      const sha256 = createHash("sha256")
        .update(readFileSync(filePath))
        .digest("hex");
      return [
        target,
        {
          file_name: fileName,
          url: new URL(encodeURIComponent(fileName), baseUrl).toString(),
          sha256,
          size_bytes: stats.size,
        },
      ];
    }),
  );

  return {
    schema_version: 1,
    component_id: "orbis-solver",
    version,
    platforms,
  };
}

if (
  process.argv[1] &&
  path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)
) {
  const artifactDirectory = process.env.COMPONENT_ARTIFACT_DIRECTORY;
  const outputPath = process.env.COMPONENT_PAYLOAD_PATH;
  if (!artifactDirectory || !outputPath) {
    throw new Error("COMPONENT_ARTIFACT_DIRECTORY and COMPONENT_PAYLOAD_PATH are required.");
  }
  const payload = createComponentPayload({
    version: process.env.COMPONENT_VERSION,
    artifactDirectory,
    artifactBaseUrl: process.env.COMPONENT_ARTIFACT_BASE_URL,
  });
  writeFileSync(outputPath, `${JSON.stringify(payload, null, 2)}\n`, {
    flag: "wx",
  });
}
